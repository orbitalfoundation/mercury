
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


pub fn camera_service(b:&MessageSender,topic:&str) {
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
