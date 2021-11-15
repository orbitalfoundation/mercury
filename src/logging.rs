

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

pub fn logging_service(b:&MessageSender) {
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
