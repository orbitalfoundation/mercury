
use broker;

pub fn tensor_service(path:&str) {
	let r = broker::listen(path);
	std::thread::spawn(move || {
		while let Ok(message) = r.recv() {
			match message {
				_ => {
					println!("tensor: got message");
				},
			}
		}
	});
}
