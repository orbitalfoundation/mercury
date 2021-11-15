

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

pub fn tensor_service(b:&MessageSender) {
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
