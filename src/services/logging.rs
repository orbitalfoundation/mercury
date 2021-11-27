
use crate::services::broker;

pub fn logging_service(path:&str) {
	let r = broker::listen(path);
	std::thread::spawn(move || {
		while let Ok(message) = r.recv() {
			match message {
				broker::Message::Event(_path,args) => {
					println!("log: {}",&args);
				},
				_ => {
					println!("log: got unknown message");
				},
			}
		}
	});
}
