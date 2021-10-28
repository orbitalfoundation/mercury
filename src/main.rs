
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

mod broker;
use crate::broker::*;

mod view_nannou;
use crate::view_nannou::*;

mod scripting;
use crate::scripting::*;

/////////////////////////////////////////////////////////////////////////////////////////////////
//
// entrypoint
//
/////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

	let b = broker_service();

	logging_service(&b);
	camera_service(&b,"/camera");
	tensor_service(&b);
	wasm_service(&b);
	scripting_service(&b,"boot.js");


	view_nannou_service(&b);


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
					println!("log: got message");
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

