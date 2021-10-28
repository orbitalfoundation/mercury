
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


use crate::broker::*;

use rusty_v8 as v8;

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// scripting
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

//#[allow(clippy::needless_pass_by_value)] // this function should follow the callback type

pub fn scripting_service(b:&MessageSender, path: &str) {

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

		// callback1: send a message to broker
		fn callback1( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
			let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
			let params = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
			BROKER.get().unwrap().send(Message::Post(message.to_string(),params.to_string()));
		}
		myglobals.set( v8::String::new(scope,"message").unwrap().into(), v8::FunctionTemplate::new(scope,callback1).into() );

		// callback2: send a message to broker
		fn callback2( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
			//let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
			thread::sleep(std::time::Duration::from_millis(1000));
			let params = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
		}
		myglobals.set( v8::String::new(scope,"sleep").unwrap().into(), v8::FunctionTemplate::new(scope,callback2).into() );

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

		// - it would be nice to have javascript listen to traffic, such as say mouse input
		// - to do that i do need to make a sender and a receiver per thread
		// - and then i need to pass the sender to anything i want to observe; ideally from a js callback
		// - and then i also need to query the receiver with try_recv() from js and return the results

		let (s,r) = unbounded::<Message>();

		while let Ok(message) = r.try_recv() {
			match message {
				_ => {},
			}
		};

	});

}

