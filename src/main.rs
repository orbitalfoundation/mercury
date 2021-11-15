

mod broker;
mod logging;
mod camera;
mod tensor;
mod wasm;
mod timer;
mod scripting;
mod view_nannou;

fn main() {

	// a broker allows other services to talk to each other; start it first
	let b = broker::broker_service();

	// logging is a built in service, useful to have it up early
	logging::logging_service(&b);

	// a pile of other miscellaneous built in services
	camera::camera_service(&b,"/camera");
	tensor::tensor_service(&b);
	timer::timer_service();

	// most all late binding services (as opposed to build in services) come in via wasm modules
	wasm::wasm_service(&b);

	// a built in scripting service instance which will run a script to produce some of the ux
	scripting::scripting_service(&b,"boot.js");

	// a built in display service, which has to be mounted last unfortunately due to a quirk with thread events
	view_nannou::view_nannou_service(&b);

}


