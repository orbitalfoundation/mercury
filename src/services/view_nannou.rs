

#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]


use nannou::prelude::*;

use crate::services::broker;

use serde_json::{Result, Value};


//pub static RECV: once_cell::sync::OnceCell<MessageReceiver> = once_cell::sync::OnceCell::new();

struct Node {
	id: i32,
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	kind: i32,
	text: String,
	textureid: usize,
}

type Texture = wgpu::Texture;

struct ViewState {
	r: broker::MessageReceiver,
	scene: Vec<Node>,
	textures: Vec<Texture>,
	path: String,
}

pub fn view_nannou_service(_path:&str) {
	// WHY do they pass my request through a needless function? - why do rust library developers always make it so hard to pass state through - see _path
	nannou::app(view_state_build).update(view_logic_update).run();
}

fn view_state_build(app: &App) -> ViewState {

	// make a window - TODO later let orbital apps make the window themselves rather than ahead of time
	let _window = app
		.new_window()
		.title(format!("Orbital Demonstration - `{:?}`",app.loop_mode()))
		.key_pressed(view_key_pressed)
		.mouse_moved(view_mouse_moved)
		.mouse_pressed(view_mouse_pressed)
		.mouse_released(view_mouse_released)
		.view(view_paint_update)
		.build()
		.unwrap();

	// set message channel - TODO use a supplied path
	let r = broker::listen("localhost:/orbital/service/view");

	// return state
	ViewState {
		r: r,
		scene: Vec::<Node>::new(),
		textures: Vec::<Texture>::new(),
		path: String::from("hello"),
	}
}

fn view_key_pressed(app: &App, state: &mut ViewState, e: Key) {
	// TODO  let str = format!("{}event:'mousemove',x:{},y:{}{}",&"{",e.x,e.y,&"}");
	broker::event_with_path("/service/view/out","{event:'key'}");
}

fn view_mouse_moved(app: &App, state: &mut ViewState, e: Vec2) {
	let str = format!("{}event:'mousemove',x:{},y:{}{}",&"{",e.x,e.y,&"}");
	broker::event_with_path("/service/view/out",&str);
}

fn view_mouse_pressed(app: &App, state: &mut ViewState, e: MouseButton) {
	broker::event_with_path("/service/view/out","{event:'mousedown'}");
}

fn view_mouse_released(app: &App, state: &mut ViewState, e: MouseButton) {
	broker::event_with_path("/service/view/out","{event:'mouseup'}");
}

fn view_logic_update(app: &App, state: &mut ViewState, update: Update) {
	//println!("{:?}", update);

	// handle new requests - especially messages that add stuff to the scene
	while let Ok(message) = state.r.try_recv() {
		match message {
			broker::Message::Event(path,args) => {

				// get json parsed
				let v :Value = serde_json::from_str(&args).unwrap();

				// capture to node
				let mut n = Node {
					id: v["id"].to_string().parse().unwrap(),
					x: v["x"].to_string().parse().unwrap(),
					y: v["y"].to_string().parse().unwrap(),
					w: v["w"].to_string().parse().unwrap(),
					h: v["h"].to_string().parse().unwrap(),
					kind: v["kind"].to_string().parse().unwrap(),
					text: v["text"].to_string(),
					textureid: 0,
				};

				// save or update
				let mut found = 0;
				for o in state.scene.iter_mut() {
					if o.id == n.id {
						o.x = n.x;
						o.y = n.y;
						o.w = n.w;
						o.h = n.h;
						found = 1;
					}
				}

				if found == 0 {

					if(n.kind == 1160) {
						if(state.textures.len() == 0) {
							let assets = app.assets_path().unwrap();
							let img_path = assets.join("textures").join("matrix.jpg");
							let texture = wgpu::Texture::from_path(app, img_path).unwrap();
							n.textureid = state.textures.len();
							state.textures.push(texture);
						}

					}

					state.scene.push(n);
				}
			},
			_ => {
				println!("view: got message");
			},
		}
	}

}

fn view_paint_update(app: &App, state: &ViewState, frame: Frame) {
	//frame.clear(DIMGRAY);

	// Begin drawing
	let draw = app.draw();

	// Clear the background to blue.
	draw.background().color(CORNFLOWERBLUE);

	let t = app.time;
	let win = app.window_rect();

	for n in &state.scene {
		match(n.kind) {
			0 => {
				// Draw a purple triangle in the top left half of the window.
				draw.tri().points(win.bottom_left(), win.top_left(), win.top_right()).color(VIOLET);
			},
			1110 => {
				draw.line()
					.weight(10.0)
					.caps_round()
					.color(PALEGOLDENROD)
					.points( pt2(n.x,n.y), pt2(n.w,n.h) )
					;
			},
			1120 => {
				draw.rect().x_y(n.x,n.y).w(n.w).h(n.h).color(DARKGREEN);
				//draw.rect().x_y(app.mouse.y, app.mouse.x).w(app.mouse.x * 0.25).hsv(t, 1.0, 1.0);
			},
			1130 => {
				draw.ellipse().x_y(n.x,n.y).radius(n.w/2.0).color(RED);
			},
			1140 => {
			    let win_rect = app.main_window().rect().pad(20.0);
			    //                         L     o     r     e     m           i    p    s    u    m
			    let glyph_colors = vec![BLUE, BLUE, BLUE, BLUE, BLUE, BLACK, RED, RED, RED, RED, RED];

			    draw.text(&n.text)
			        .color(WHITE)
			        .glyph_colors(glyph_colors)
			        .font_size(24)
			        .wh(win_rect.wh());
			},
			1160 => {
				draw.texture(&state.textures[n.textureid]).xy(pt2(n.x,n.y)).wh(pt2(n.w,n.h));
			}
			_ => {
			}
		}
	}


	// Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
}


