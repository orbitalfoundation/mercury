
use crate::services::broker;

pub fn camera_service(path:&str) {
	let r = broker::listen(path);
	std::thread::spawn(move || {
		while let Ok(message) = r.recv() {
			match message {
				broker::Message::Event(path,args) => {
					println!("camera got post {} {}",&path,&args);
				},
				_ => {
					println!("camera: got message");
				},
			}
		}
	});
}
