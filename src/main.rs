
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
/*

Orbital revised architecture - oct 22 2021

Thread goals:

	+ Be able to run a lot of separate independent services (cameras, tensorflow, wasm blobs, display, scripting)

	+ Each service will to be able to be its own totally asynchronous thread (I don't see other easy ways schedule or manage code evaluation)

Messaging Goals:

	+ Let services communicate with each other; nominally even have an idea of "wires" that connect services together

	+ The pattern I'm using is an idea of a pubsub model, where services can register to listen to a "topic" and can broadcast to a "topic"

	+ A "topic" doesn't necessarily have to have anybody broadcasting to it; there is no need to define a service prior to defining a topic.

	+ Many parties can broadcast to a topic - only the security system prevents this.

	+ Many parties can listen to a topic

Security Goals:

	+ We would like an ability to set a security policy around any given service; to prevent it from listening to or broadcasting to other services

Scripting Goals:

	+ Scripts such as wasm or javascript should be able to start services and wire them together

*/
/////////////////////////////////////////////////////////////////////////////////////////////// 

fn main() {

	let b = broker_service();

	camera_service(&b,"/camera");
	tensor_service(&b);
	wasm_service(&b);
	scripting_service(&b,"boot.js");
	display_service(&b);

	std::thread::sleep(std::time::Duration::from_millis(1000));

}

fn main_test() {
	let b = broker_service();
	let (s,r) = unbounded::<Message>();
	b.send( Message::Observe("stuff".to_string(),s));
	b.send( Message::Spawn("stuff".to_string(),"wow".to_string()));	
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

		// an "object" -> is apparently the equivalent of javascripts "const obj = {}"
		let myglobals = v8::ObjectTemplate::new(scope);

		// create space for a field later
		myglobals.set_internal_field_count(1);

		// add function
		myglobals.set( v8::String::new(scope,"log").unwrap().into(), v8::FunctionTemplate::new(scope,js_callback).into() );

		// promote this new object to be 'global'
		let context = v8::Context::new_from_template(scope, myglobals);
		let scope = &mut v8::ContextScope::new(scope, context);

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// stuff our back pointer to the messaging system into the javascript layer so that it is visible to the fucking callback
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// 1) in rust parlance this moves the artifact to the heap along with a nominal concept of ownership
		//let boxed_sender = Box::<MessageSender>::new(b);

		// 2) now violate that trust by getting a raw pointer to the thing
		//let boxed_ptr : *mut MessageSender = Box::into_raw(boxed_sender);

		// 3) and explicitly more exactly cast raw ptr as a raw 'unknown' pointer because rust
		//let raw_ptr = boxed_ptr as *mut std::ffi::c_void;

		// 4) wrap that radioactive waste in a v8 compatible 'external'
		//let ext = v8::External::new(scope,raw_ptr);

		// 5) stuff external an internal field area - no idea what "into" means
		//context.global(scope).set_internal_field(0,ext.into());

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// run the javascript
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// js code
		let code = v8::String::new(scope, "log(12); log(14);").unwrap();

		// run it
		let script = v8::Script::compile(scope, code, None).unwrap();
		let result = script.run(scope).unwrap();
		let result = result.to_string(scope).unwrap();
		println!("scripting: {}", result.to_rust_string_lossy(scope));

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


// test callback #1
fn js_callback( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {

	// 1) get context
	//let context = scope.get_current_context();

	// 2) get external from internal - preemptively unwrap because we believe it exists - ignore Some/None
	//let ext = context.global(scope).get_internal_field(scope,0).unwrap();

	// 3) cast it back
    //let ext = unsafe { v8::Local::<v8::External>::cast(ext) };

	// 4) get it as a c pointer again
	//let raw_ptr : *mut std::ffi::c_void = ext.value();
	//let raw_ptr2 = raw_ptr as *mut MessageSender;

	// 5) go back up to being a boxed messagesender
	//let recovered = unsafe { Box::<MessageSender>::from_raw( raw_ptr2 ) };

	// 6) send it a message as a test
	//recovered.send(Message::Post("/camera".to_string(),"amazing ".to_string()));

	// the ref is being refcounted away...
	//let boxed_ptr : *mut MessageSender = Box::into_raw(recovered);

	BROKER.get().unwrap().send(Message::Post("/camera".to_string(),"amazing ".to_string()));

	// do something to peek at the javascript first argument
	let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
	//println!("Logged: {}", message);

	// send something fun back
	let my_int: i32 = message.parse().unwrap();
	_retval.set(v8::Integer::new(scope, my_int).into());
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

