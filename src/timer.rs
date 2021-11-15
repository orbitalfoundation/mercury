
use crate::broker::*;

pub fn timer_service() {
	std::thread::spawn(move || {
		loop {
			std::thread::sleep(std::time::Duration::from_millis(1000));
			let _ = BROKER.get().unwrap().send( Message::Post("/timer".to_string(),"{event:'tick'}".to_string()));
		}
	});
}
