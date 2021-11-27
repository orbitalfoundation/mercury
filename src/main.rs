
mod services {
	pub mod broker;
	pub mod logging;
	pub mod timer;
	pub mod wasm;
	pub mod camera;
	pub mod tensor;
	pub mod scripting;
	pub mod view_nannou;
}

fn main() {

	// run a pile of services
	services::broker::broker_service("localhost:/orbital/broker");
	services::logging::logging_service("localhost:/orbital/service/log");
	services::scripting::scripting_service("localhost:/orbital/service/scripting");
	services::camera::camera_service("localhost:/orbital/service/camera");
	services::tensor::tensor_service("localhost:/orbital/service/tensor");
	services::timer::timer_service("localhost:/orbital/service/timer");
	services::wasm::wasm_service("localhost:/orbital/service/wasm");

	// kick off a boot script
	services::broker::event_with_path("localhost:/orbital/service/scripting",r#"{"command":"spawn","file":"public/kernel/boot.js","listen":"localhost:/orbital/home/root/running/boot"}"#);

	// attach a display service (unfortunately this has to be run last due to a quirk with thread events - maybe could join thread?)
	services::view_nannou::view_nannou_service("localhost:/orbital/service/view");

}
