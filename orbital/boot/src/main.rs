use broker;
use logging;
use scripting;

use timer;
use wasm;
use tensor;

use camera;

use view_nannou;

fn main() {

	// run a pile of services
	broker::broker_service("localhost:/orbital/broker");
	logging::logging_service("localhost:/orbital/service/log");
	scripting::scripting_service("localhost:/orbital/service/scripting");
	camera::camera_service("localhost:/orbital/service/camera");
	tensor::tensor_service("localhost:/orbital/service/tensor");
	timer::timer_service("localhost:/orbital/service/timer");
	wasm::wasm_service("localhost:/orbital/service/wasm");

	// kick off a raw boot script - TODO later could be an full blown orbital style app (something with a manifest)
	broker::event_with_path("localhost:/orbital/service/scripting",r#"{"command":"spawn","file":"public/kernel/boot.js","listen":"localhost:/orbital/home/root/running/boot"}"#);

	// attach a display service (unfortunately this has to be run last due to a quirk with thread events - maybe could join thread?)
	view_nannou::view_nannou_service("localhost:/orbital/service/view");

}
