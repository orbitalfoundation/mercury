
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

	camera_service(&b,"{}","/camera");
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
pub type MessageVec = Vec<MessageSender>;

#[derive(Clone)]
pub enum Message {

	// Ask to receive a copy of all traffic to a topic
	Observe(String,MessageSender),

	// Send a string to a topic
	Post(String,String),

	// Send a frame buffer to a topic
	Share(String,Arc<Mutex<Box<[u32;921600]>>>),

	// Send a spawn command to a topic
	Spawn(String,String),
}

pub struct Listeners {
	pub listeners: RefCell<MessageVec>,
}
impl Listeners {
	fn new() -> Listeners {
		Listeners {
			listeners: RefCell::new(MessageVec::new()),
		}
	}
	fn insert(&self,l:&MessageSender) {
		self.listeners.borrow_mut().push(l.clone());
	}
}

fn broker_service() -> MessageSender {
	let (s,r) = unbounded::<Message>();
	thread::spawn(move || {
		let mut registry = HashMap::<String,Listeners>::new();
		while let Ok(message) = r.recv() {
			match message {
				Message::Observe(topic,sender) => {
					if !registry.contains_key(&topic) {
						println!("Broker: adding observer {}",&topic);
						registry.insert(topic.to_string(),Listeners::new());
					}
					registry[&topic].insert(&sender);
				},
				Message::Spawn(policy,topic) => {
					if registry.contains_key(&topic) {
						let x = &registry[&topic];
						let y = &x.listeners;
						let z = y.borrow_mut();
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

fn camera_service(b:&MessageSender,policy:&str,observe:&str) {
	let b = b.clone();
	let observe = observe.to_string();
	thread::spawn(move || {
		let (s,r) = unbounded::<Message>();
		b.send(Message::Observe(observe,s));
		while let Ok(message) = r.recv() {
			match message {
				Message::Spawn(policy,observe) => {
					camera_service(&b,&policy,&observe);
				},
				_ => {
					println!("camera_spawn: got message");
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

	// a name
	let name = "scripting";

	// message handlers

	// spawn a thread and watch for events
	let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

		// some rando chicken waving js bootstrap crap
		let platform = v8::new_default_platform(0, false).make_shared();
		v8::V8::initialize_platform(platform);
		v8::V8::initialize();

		// some kind of context concept? dunno, weird whatever dead chicken waving
		let isolate = &mut v8::Isolate::new(Default::default());
		let handle = &mut v8::HandleScope::new(isolate);
		let context = v8::Context::new(handle);
		let scope = &mut v8::ContextScope::new(handle, context);

		// some kind of 'template' concept
		let object_templ = v8::ObjectTemplate::new(scope);

			/* cleaner way to handle vars
			let my_u8: u8 = "42".parse::<u8>().unwrap();
			let my_u32: u32 = "42".parse::<u32>().unwrap();

			// or, to be safe, match the `Err`
			match "foobar".parse::<i32>() {
			  Ok(n) => do_something_with(n),
			  Err(e) => weep_and_moan(),
			}
			*/

			// a fn
			fn log_callback( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
				let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
				println!("Logged: {}", message);
				let my_int: i32 = message.parse().unwrap();
				_retval.set(v8::Integer::new(scope, my_int).into());
			}
			object_templ.set( v8::String::new(scope,"log").unwrap().into(), v8::FunctionTemplate::new(scope,log_callback).into() );

			// another fn
			fn log2_callback( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
				let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
				println!("Logged2: {}", message);
				let my_int: i32 = message.parse().unwrap();
				_retval.set(v8::Integer::new(scope, my_int).into());
			}
			object_templ.set( v8::String::new(scope,"log2").unwrap().into(), v8::FunctionTemplate::new(scope,log2_callback).into() );

			// another fn -> see https://github.com/denoland/rusty_v8/blob/fa8f636397822ba7dbb823d25dc637bf77f8b1ce/tests/test_api.rs#L1091-L1101
			// also see -> https://github.com/denoland/rusty_v8/issues/404
			// see https://github.com/denoland/deno/blob/ccd0d0eb79db6ad33095ca06e9d491a27379b87a/core/examples/http_bench.rs#L184-L195
			// see https://github.com/denoland/rusty_v8/blob/main/examples/process.rs
			//let create_message = v8::Function::new(scope,
			//	|scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue| {
			//		let message = v8::Exception::create_message(scope, args.get(0));
			//		let message_str = message.get(scope);
			//		rv.set(message_str.into())
			//  },
			//).unwrap();
			//object_templ.set( v8::String::new(scope,"msg").unwrap().into(), create_message.into() );


		let context = v8::Context::new_from_template(scope, object_templ);
		let scope = &mut v8::ContextScope::new(scope, context);

		//let scope = scope.enter();
		//let context = v8::Context::new_from_function(scope, create_message);
		//let scope = &mut v8::ContextScope::new(scope, context);

		// test code
		let code = v8::String::new(scope, "let x = log(12) + log2(14); x").unwrap();

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

