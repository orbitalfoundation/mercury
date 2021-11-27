
use crate::services::broker;
use serde_json::Value;

pub fn timer_service(path:&str) {
	let path2 = path.to_string();
	let r = broker::listen(&path2);
	std::thread::spawn(move || {
		let mut echos = Vec::new();
		loop {
			while let Ok(message) = r.try_recv() {
				match message {
					broker::Message::Event(_path,args) => {
						//println!("timer: got msg {} {}",&path,&args);
						let v:Value = serde_json::from_str(&args).unwrap();
						let command : String = v["command"].as_str().unwrap().to_string();
						if command.eq("echo") {
							let echo : String = v["echo"].as_str().unwrap().to_string();
							//println!("got echo command {}",echo);
							echos.push(echo);
						}
					},
					_ => {
						println!("timer: got unhandled message");
					},
				}
			}

			std::thread::sleep(std::time::Duration::from_millis(1000));

			for x in &echos {
				broker::event_with_path(&x,"{event:'tick'}");
				//println!("timer: echoes to {}",x);
			}

		}
	});
}

