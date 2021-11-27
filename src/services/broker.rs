


use std::thread;
use std::collections::HashMap;
use std::cell::RefCell;
use std::vec::Vec;
use crossbeam::channel::*;
//use serde::{Serialize, Deserialize};

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Broker
///
/// The broker allows components to broadcast to paths and to listen to paths; it has some security capabilities
///
/// TODO persistence?
///
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub type MessageSender = Sender<Message>;
pub type MessageReceiver = Receiver<Message>;
//pub type SenderVec = Vec<MessageSender>;


#[derive(Clone)]
pub enum Message {

	// Ask to receive a copy of all traffic to a path
	Mount(String,MessageSender),

	// Send a string to a path
	Event(String,String),

	// Send a frame buffer to a path
	// Share(String,Arc<Mutex<Box<[u32;921600]>>>),

}

// this was an attempt to mount a static handle - it is just too much of a mess - see better way below
//#[macro_use]
//extern crate lazy_static;
//lazy_static! { static ref BROKER: (MessageSender,MessageReceiver) = { unbounded::<Message>() }; }

///
/// A static global handle on broker
///

static BROKER: once_cell::sync::OnceCell<MessageSender> = once_cell::sync::OnceCell::new();

///
/// Send an event to the broker where the path is packaged up in the event itself
///

//pub fn event(_event:&str) {
	// TODO
	// BROKER.get().unwrap().send(Message::Event(path.to_string(),args.to_string()));
//}

///
/// Send an event to the broker intended for a specified path
///

pub fn event_with_path(path:&str,args:&str) {
	let _ = BROKER.get().unwrap().send(Message::Event(path.to_string(),args.to_string()));
}

///
/// Mount a fresh listener on a path
///

pub fn listen(path:&str) -> MessageReceiver {
	let (s,r) = unbounded::<Message>();
	let _ = BROKER.get().unwrap().send( Message::Mount(path.to_string(),s));
	r
}

///
/// Broker service itself
///

pub fn broker_service(_path:&str) {
	let (s,r) = unbounded::<Message>();
	let _ = BROKER.set(s.clone());
	thread::spawn(move || {
		let mut registry = HashMap::<String,RefCell<Vec::<MessageSender>>>::new();
		while let Ok(message) = r.recv() {
			match message {
				Message::Mount(path,sender) => {
					// register a listener service that will receive traffic on a path
					if !registry.contains_key(&path) {
						let v = RefCell::new(Vec::<MessageSender>::new());
						registry.insert(path.to_string(),v);
					}
					registry[&path.to_string()].borrow_mut().push(sender);
				},
				Message::Event(path,args) => {
					// forward an event onwards to some service mounted on a path
					if registry.contains_key(&path) {
						for y in registry[&path.to_string()].borrow_mut().iter() {
							let m = Message::Event(path.to_string(),args.to_string());
							let _ = y.send(m);
						}
					}
				},
				//_ => {
				//	println!("Broker: got message");
				//},
			}
		}
	});
}
