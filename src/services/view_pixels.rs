#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crossbeam::channel::*;
use crate::broker::*;


//use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;


pub fn pixels_service(b:&MessageSender) -> Result<(), Error> {

	let mut world = World::new(&b);

//    env_logger::init();
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("Hello Pixels")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};

	event_loop.run(move |event, _, control_flow| {
		// Draw the current frame
		if let Event::RedrawRequested(_) = event {
			world.draw(pixels.get_frame());
			if pixels
				.render()
				.map_err(|e| println!("pixels.render() failed: {}", e))
				.is_err()
			{
				*control_flow = ControlFlow::Exit;
				return;
			}
		}

		// Handle input events
		if input.update(&event) {
			// Close events
			if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
				*control_flow = ControlFlow::Exit;
				return;
			}

			// Resize the window
			if let Some(size) = input.window_resized() {
				pixels.resize_surface(size.width, size.height);
			}

			// Update internal state and request a redraw
			world.update();
			window.request_redraw();
		}
	});
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
	x: i16,
	y: i16,
	w: i16,
	h: i16,
	vx: i16,
	vy: i16,
	recv: MessageReceiver,
}

impl World {
	/// Create a new `World` instance that can draw a moving box.
	fn new(b:&MessageSender) -> Self {

		let (s,r) = unbounded::<Message>();
		b.send( Message::Observe("/pixels".to_string(),s));

		Self {
			x: 24,
			y: 16,
			w: 0,
			h: 0,
			vx: 1,
			vy: 1,
			recv: r,
		}
	}

	fn update(&mut self) {
		if self.x <= 0 || self.x + self.w > WIDTH as i16 {
			self.vx *= -1;
		}
		if self.y <= 0 || self.y + self.w > HEIGHT as i16 {
			self.vy *= -1;
		}

		self.x += self.vx;
		self.y += self.vy;

		while let Ok(message) = self.recv.try_recv() {
			match message {
				Message::Post(topic,params) => {
					self.w+=4;
					self.h+=4;
				},
				_ => {
					println!("pixels: got message");
				},
			}
		}


	}

	/// Draw the `World` state to the frame buffer.
	///
	/// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
	fn draw(&self, frame: &mut [u8]) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = (i % WIDTH as usize) as i16;
			let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.x
                && x < self.x + self.w
                && y >= self.y
                && y < self.y + self.h;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };
			pixel.copy_from_slice(&rgba);
		}
	}
}