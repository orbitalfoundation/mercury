

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

    ? 


    - 2d
    	- open a window
        - draw a box
        - draw a circle
        - images
        - draw some text
        - various scaling, cutting, pasting, pruning operations
        - paths, bezier and so on
        - thick lines and so on

    - 3d
    	- a retained mode scene graph (i can make it?)
        - load a gltf
        - lighting
        - camera

    - behavior
        - layout engine
        - detect mouse intersection with 2d and 3d objects
        - object to object collision
        - physics and animation built in or using a third party tool

    - maths library


## todo oct 2021

	- test having js call back up to make some components
	- test wiring components together from js; basically make a tensor that listens to a camera -> i need to be able to pass args to a tensor or other service somehow
	- test getting input from mouse from a display emitter
	- test drawing to display

## useful?
	https://docs.rs/crossbeam-channel/0.4.0/src/crossbeam_channel/counter.rs.html
	https://google.github.io/mediapipe/solutions/objectron.html


