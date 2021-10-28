use nannou::prelude::*;

use crossbeam::channel::*;
use crate::broker::*;

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
}

struct ViewState {
	r: MessageReceiver,
	scene: Vec<Node>
}

pub fn view_nannou_service(b:&MessageSender) {
	nannou::app(view_state_build).update(view_logic_update).run();
}

fn view_state_build(app: &App) -> ViewState {

	// make app 
	let _window = app
		.new_window()
		.title(format!("Orbital Demonstration - `{:?}`",app.loop_mode()))
		.key_pressed(view_key_pressed)
		.view(view_paint_update)
		.build()
		.unwrap();

	// set message channel
	let (s,r) = unbounded::<Message>();
	BROKER.get().unwrap().send( Message::Observe("/pixels".to_string(),s));

	// return state
	ViewState {
		r: r,
		scene: Vec::<Node>::new()
	}
}


fn view_key_pressed(app: &App, state: &mut ViewState, _key: Key) {
	//let title = format!("`LoopMode` Demonstration - `{:?}`", app.loop_mode());
	//app.main_window().set_title(&title);
}

fn view_logic_update(_app: &App, state: &mut ViewState, update: Update) {
	//println!("{:?}", update);

	// get requests
	while let Ok(message) = state.r.try_recv() {
		match message {
			Message::Post(topic,params) => {

				// get json parsed
				let v :Value = serde_json::from_str(&params).unwrap();

				// capture to node
				let n = Node {
					id: v["id"].to_string().parse().unwrap(),
					x: v["x"].to_string().parse().unwrap(),
					y: v["y"].to_string().parse().unwrap(),
					w: v["w"].to_string().parse().unwrap(),
					h: v["h"].to_string().parse().unwrap(),
					kind: v["kind"].to_string().parse().unwrap(),
					text: v["text"].to_string(),
				};

				// save or update
				let mut found = 0;
				for o in state.scene.iter_mut() {
					if o.id == n.id {
						o.w = n.w;
						o.h = n.h;
						found = 1;
					}
				}

				if found == 0 {
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
	frame.clear(DIMGRAY);

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
			1 => {
				draw.line()
					.weight(10.0)
					.caps_round()
					.color(PALEGOLDENROD)
					.points( pt2(n.x,n.y), pt2(n.w,n.h) )
					;

			},
			2 => {
				draw.ellipse().x_y(n.x,n.y).radius(n.w/2.0).color(RED);
			},
			3 => {
			    let win_rect = app.main_window().rect().pad(20.0);
			    //                         L     o     r     e     m           i    p    s    u    m
			    let glyph_colors = vec![BLUE, BLUE, BLUE, BLUE, BLUE, BLACK, RED, RED, RED, RED, RED];

			    draw.text(&n.text)
			        .color(BLACK)
			        .glyph_colors(glyph_colors)
			        .font_size(24)
			        .wh(win_rect.wh());


			},
			_ => {

				// Draw a quad that follows the inverse of the ellipse.
				draw.quad()
					.x_y(-app.mouse.x, app.mouse.y)
					.color(DARKGREEN)
					.rotate(t);

				// Draw a rect that follows a different inverse of the ellipse.
				draw.rect()
					.x_y(app.mouse.y, app.mouse.x)
					.w(app.mouse.x * 0.25)
					.hsv(t, 1.0, 1.0);

			}
		}
	}


	// Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
}


