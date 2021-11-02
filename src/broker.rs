

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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Broker
///
/// The broker allows components to broadcast to topics and to listen to topics; it has some security capabilities
///
/// TODO persistence?
///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub type MessageSender = Sender<Message>;
pub type MessageReceiver = Receiver<Message>;
pub type SenderVec = Vec<MessageSender>;

//#[macro_use]
//extern crate lazy_static;
//lazy_static! { static ref BROKER: (MessageSender,MessageReceiver) = { unbounded::<Message>() }; }

pub static BROKER: once_cell::sync::OnceCell<MessageSender> = once_cell::sync::OnceCell::new();

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

pub fn broker_service() -> MessageSender {
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
