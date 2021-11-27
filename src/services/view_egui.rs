
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


// docs are bad???

pub view_egui_service(b:&MessageSender,topic:&str) {

	ui.heading("My egui Application");
	ui.horizontal(|ui| {
	    ui.label("Your name: ");
	    ui.text_edit_singleline(&mut name);
	});
	ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
	if ui.button("Click each year").clicked() {
	    age += 1;
	}
	ui.label(format!("Hello '{}', age {}", name, age));

}





