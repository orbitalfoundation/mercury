
#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::vec::Vec;
use crossbeam::channel::*;
use serde::{Serialize, Deserialize};

use rusty_v8 as v8;


/////////////////////////////////////////////////////////////////////////////////////////////////
//
// Orbital Core design oct 26 2021
//		- be able to run lots of possibly separate threads ( cameras, machine learning, wasm blobs, displays, logging, io, scripts )
//		- let services talk to each other
//		- have security between services
//		- have a nice package manager to fetch services over the net
//		- a group of services can be conceptually considered to be an 'application'
//
// Scripting support
//		- let me drive the system from js
//		- later on introduce a visual wiring language
//		- define the main user interface in js
//
// Display support
//		- have a retained model scenegraph
//		- can collect global state ( 3d reconstruction of world ) and share on demand
//
/////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

	let b = broker_service();

	logging_service(&b);
	camera_service(&b,"/camera");
	tensor_service(&b);
	wasm_service(&b);
	scripting_service(&b,"boot.js");
	display_service(&b);

	std::thread::sleep(std::time::Duration::from_millis(1000));

}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Broker
///
/// The broker allows components to broadcast to topics and to listen to topics; it has some security capabilities
///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub type MessageSender = Sender<Message>;
pub type MessageReceiver = Receiver<Message>;
pub type SenderVec = Vec<MessageSender>;

//#[macro_use]
//extern crate lazy_static;
//lazy_static! { static ref BROKER: (MessageSender,MessageReceiver) = { unbounded::<Message>() }; }

use once_cell::sync::OnceCell;

static BROKER: OnceCell<MessageSender> = OnceCell::new();

#[derive(Clone)]
pub enum Message {

	// Ask to receive a copy of all traffic to a topic
	Observe(String,MessageSender),

	// Send a spawn command to a topic
	Spawn(String,String),

	// Send a string to a topic
	Post(String,String),

	// Send a frame buffer to a topic
	Share(String,Arc<Mutex<Box<[u32;921600]>>>),

}

pub struct Listeners {
	pub listeners: RefCell<SenderVec>,
}
impl Listeners {
	fn new() -> Listeners {
		Listeners {
			listeners: RefCell::new(SenderVec::new()),
		}
	}
	fn insert(&self,l:&MessageSender) {
		self.listeners.borrow_mut().push(l.clone());
	}
}

fn broker_service() -> MessageSender {
	let (s,r) = unbounded::<Message>();
	BROKER.set(s.clone());
	thread::spawn(move || {
		let mut registry = HashMap::<String,RefCell<Vec::<MessageSender>>>::new();
		while let Ok(message) = r.recv() {
			match message {
				Message::Observe(topic,sender) => {
					if !registry.contains_key(&topic) {
						let mut v = RefCell::new(Vec::<MessageSender>::new());
						registry.insert(topic.to_string(),v);
					}
					registry[&topic.to_string()].borrow_mut().push(sender);
				},
				Message::Spawn(topic,policy) => {
					if registry.contains_key(&topic) {
						for y in registry[&topic.to_string()].borrow_mut().iter() {
							let m = Message::Spawn(topic.to_string(),policy.to_string());
							y.send(m);
						}
					}
				},
				Message::Post(topic,params) => {
					if registry.contains_key(&topic) {
						for y in registry[&topic.to_string()].borrow_mut().iter() {
							let m = Message::Post(topic.to_string(),params.to_string());
							y.send(m);
						}
					}
				},
				_ => {
					println!("Broker: got message");
				},
			}
		}
	});
	return s;
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Another component (test)
///
/////////////////////////////////////////////////////////////////////////////////////////////////

fn camera_service(b:&MessageSender,topic:&str) {
	let b = b.clone();
	let topic = topic.to_string();
	thread::spawn(move || {
		let (s,r) = unbounded::<Message>();
		b.send( Message::Observe(topic.to_string(),s));
		while let Ok(message) = r.recv() {
			match message {
				Message::Post(topic,params) => {
					println!("camera got post {} {}",&topic,&params);
				},
				_ => {
					println!("camera: got message");
				},
			}
		}
	});
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// logging
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

fn logging_service(b:&MessageSender) {
	let b = b.clone();
	thread::spawn(move || {
		let (s,r) = unbounded::<Message>();
		BROKER.get().unwrap().send( Message::Observe("/log".to_string(),s));
		while let Ok(message) = r.recv() {
			match message {
				Message::Post(topic,params) => {
					println!("log: {}",&params);
				},
				_ => {
					println!("tensor: got message");
				},
			}
		}
	});
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// tensor
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

fn tensor_service(b:&MessageSender) {
	let b = b.clone();
	thread::spawn(move || {
		let (s,r) = unbounded::<Message>();
		while let Ok(message) = r.recv() {
			match message {
				Message::Spawn(topic,policy) => {
				},
				_ => {
					println!("tensor: got message");
				},
			}
		}
	});
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// wasm
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

fn wasm_service(b:&MessageSender) {
}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// scripting
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

//#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type

fn scripting_service(b:&MessageSender, path: &str) {

	// shake off the lifetime scope of the borrow checker
	let b = b.clone();

	// spawn a thread and watch for events
	thread::spawn(move || {

		// Initialize V8
		let platform = v8::new_default_platform(0, false).make_shared();
		v8::V8::initialize_platform(platform);
		v8::V8::initialize();

		// An isolate is another copy of the v8 runtime for some unknown reason
		let isolate = &mut v8::Isolate::new(Default::default()); // v8::CreateParams::default() also works?

		// create a stack allocated handle scope
		let handle = &mut v8::HandleScope::new(isolate);

		// a "context"?
		let context = v8::Context::new(handle);

		// A "scope" in a context...?
		let scope = &mut v8::ContextScope::new(handle, context);

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// make an "object" that will become the global state, and stuff some callbacks into it
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// define an "object" -> is apparently the equivalent of javascripts "const obj = {}"
		let myglobals = v8::ObjectTemplate::new(scope);

		// callback: send a message to broker
		fn orbital_message( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
			let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
			let params = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
			BROKER.get().unwrap().send(Message::Post(message.to_string(),params.to_string()));
		}
		myglobals.set( v8::String::new(scope,"message").unwrap().into(), v8::FunctionTemplate::new(scope,orbital_message).into() );

		// promote this new object to be 'global'
		let context = v8::Context::new_from_template(scope, myglobals);
		let scope = &mut v8::ContextScope::new(scope, context);

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// run the javascript
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// fetch the js
		let contents = std::fs::read_to_string("public/boot.js").expect("Load error");
		let sourcecode = v8::String::new(scope,&contents).unwrap();

		// run it
		let script = v8::Script::compile(scope, sourcecode, None).unwrap();
		let result = script.run(scope).unwrap();
		let result = result.to_string(scope).unwrap();
		//println!("scripting: {}", result.to_rust_string_lossy(scope));

		// here i guess i would not watch but just return , and do a try_recv() as a callback, can i do a sleep??
		// basically below should be obsolete: TODO remove

		// watch stuff forever
		let (s,r) = unbounded::<Message>();
		while let Ok(message) = r.recv() {
			match message {
				_ => {},
			}
		};

	});

}

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// display
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

/*


- basic graphics in rust

	- open a window in rust

	- 2d
		- draw a box
		- draw a circle
		- images
		- draw some text
		- various scaling, cutting, pasting, pruning operations
		- paths, bezier and so on
		- thick lines and so on

	- 3d
		- load a gltf
		- lighting
		- camera

	- behavior
		- layout engine
		- detect mouse intersection with 2d and 3d objects
		- object to object collision
		- physics and animation built in or using a third party tool


	- maths library

*/

fn display_service(b:&MessageSender) {
}



/*

todo oct 2021

	- test having js call back up to make some components
	- test wiring components together from js; basically make a tensor that listens to a camera -> i need to be able to pass args to a tensor or other service somehow
	- test getting input from mouse from a display emitter
	- test drawing to display

useful?
	https://docs.rs/crossbeam-channel/0.4.0/src/crossbeam_channel/counter.rs.html
	https://google.github.io/mediapipe/solutions/objectron.html

*/

