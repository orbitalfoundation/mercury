

# Orbital design sketch oct 26 2021

	- be able to run lots of possibly separate threads ( cameras, machine learning, wasm blobs, displays, logging, io, scripts )
	- let services talk to each other
	- have security between services
	- have a nice package manager to fetch services over the net
	- a group of services can be conceptually considered to be an 'application'

## scripting

	- let me drive the system from js
	- later on introduce a visual wiring language
	- define the main user interface in js

## display

	? i could use wgpu + say wgpu_glyph to compose and render primitives by hand
		this requires building out concepts like lights, cameras, objects and so on
		and integrating that with 2d painting and widgets as well

	? i could use say egui and bind it to wgpu as well it looks like
		this does have 2d drawing primitives
		and it does give me some rich ux

	? nannou would get me part way...


	- basic primitive abstractions to expose to js

		- display -> a whole display surface; can only have one
		- area -> may as well be 3d
		- area -> nesting and layout concepts may be here as well
		- box -> relative to parent scope
		- circle
		- image
		- text; scaling
		- paths; lines

		- gltfs
		- lights
		- cameras
		- tubes
		- sphere
		- box

	- richer features to expose
		- a layout engine
		- mouse events
		- collision
		- physics and animation
		- maths library?

## todo oct 2021

	- test having js call back up to make some components
	- test wiring components together from js; basically make a tensor that listens to a camera -> i need to be able to pass args to a tensor or other service somehow
	- test getting input from mouse from a display emitter
	- test drawing to display

## useful?
	https://docs.rs/crossbeam-channel/0.4.0/src/crossbeam_channel/counter.rs.html
	https://google.github.io/mediapipe/solutions/objectron.html


