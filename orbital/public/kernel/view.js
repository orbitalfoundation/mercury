

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// View helper abstraction
//
// Here are a pile of capabilities that let js talk to built in display module more easily - not critical but helpful
//
// General features:
//
// 		- The heavy lifting of the display engine is written in native rust
//
// 		- It has a concept of a scene graph made up out of many "nodes" or "elements". It walks these every frame to paint the display
//
//		- The entire user interface is this one single declarative scene graph, with callbacks, to deliver user experience
//
//		- We can only change the display by passing requests to add or remove fragments of the scene graph
//
//		- Generally speaking this parallels the idea of a DOM; it should be familiar to people who know HTML
//
//		- You can emulate an "immediate mode" ability such as "draw a circle now!" by feeding the display engine a small graph fragment
//
//		- The specific nodes or elements are common visual concepts like box,circle,text,cube,gltf,light,camera and so on
//
//		- The base class of a node/element has common properties including visibility, location, material, event handling, children, text
//
// View prototypes support concept:
//
//		- this is a convenience concept to allow for a reasonably powerful CSS like concept
//
//		- the display engine has a concept of elements that are not being rendered
//
//		- it also has a concept of an element inheriting properties from any other element anywhere in the graph
//
//		- these prototypes are simply elements that you can add to your graph and not render
//
//		- if you inherit off of one of these, and then modify it, that change can propagate to all of your related elements
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// maybe use an enum TODO

let ELEMENT = 0

let GROUP = 10
let CAMERA = 20
let LIGHT = 30

let POINT = 100
let LINE = 110
let CUBE = 120
let SPHERE = 130
let TEXT = 140
let POLYEHDRA = 150
let IMAGE = 160
let GLTF = 170


let POINT2D = 1100
let LINE2D = 1110
let BOX2D = 1120
let CIRCLE2D = 1130
let TEXT2D = 1140
let POLYHEDRA2D = 1150
let IMAGE2D = 1160
let SVG2D = 1170

let prototypes = { // todo later actually use this - it is not used right now
	kind:ELEMENT,
	visible:false,
	element: {
		kind:ELEMENT,
		extend:ELEMENT, // this is a concept to allow a node to clone part of the graph fragment; to help do a css like macro
		rewrite:ELEMENT, // this is a concept to let a node back rewrite basic fragment style; to help do a css like macro
		path:"user/partyname/appname/page/element/id",
		name:"myexampleelementname",
		id:"0",
		children: [],
		visible:false,
		is2d:false, // 2d or 3d?
		collision_layer:0, // a concept of layers for collision
		collision_mask:0, // layers that this element can be collided with
		material:{
			color:"#ff00ff",
			alpha:1.0,
			gradient:{},
			pattern:{},
			thickness:1.0,
			blur:0.0,
			bevel:0.0,
			fill:true,
			mitre:0.0,
			caps:0.0,
		},
		xyz: [0,0,0],
		ypr: [0,0,0],
		whd: [0,0,0],
		text: "",
		shaders: {},
		events: {}, // collision, tick, attach, detach and so on
		layout: {
			rule:0,		// layout rule of 0 means ignore
			margin:0,	// in a 2d layout a margin can be respected
			padding:0,
			width:0,
			height:0,
		}
	},
	button: {
		kind:ELEMENT,
		text:"my sample button",
		is2d: true,
	},
	text: {
		kind:ELEMENT,
		is2d: true,
	},
	image: {
		kind:ELEMENT,
		is2d: true,
	},
	app: {
		kind:ELEMENT,
		is2d: true,
	},
}


///
/// A wrapper or proxy for the rust side view engine (which is a singleton)
///	- tell the view engine to echo mouse events here at startup
///

class VIEW extends SERVICE {
	constructor() {
		super();
		this.idgenerator = 1000
		// tell broker to tell rust engine to echo view related events to this javascript isolate as a whole
		BROKER.event({
			path:"localhost:/orbital/service/view",
			command:"echo",
			events:"io",
			echo:LISTENPATH,
		})
		// this isolate is now receiving events as a whole, but the view engine needs to further qualify:
		this.recurse = this.recurse.bind(this)
		SERVICEEVENTS.add("mousemove",this.recurse);
		SERVICEEVENTS.add("mousedown",this.recurse);
		SERVICEEVENTS.add("mouseup",this.recurse);
		// listen to rust view side issued events for 'tick'
		// TODO listening to tick here is really not perfect - I need to kind of better wire together timer and view in javascript
		SERVICEEVENTS.add("tick",this.recurse);
	}
	load(fragment) {
		this.fragment = fragment
		this.recurse({event:"tick"})
	}
	recurse(e) {
		if(e && e.event == "mousemove") this.drawmouse(e)
		this.recurse_node(this.fragment,e)
	}
	recurse_node(node,e) {
		if(!node) {
			return
		}
		if(Array.isArray(node)) {
			node.forEach(child => this.recurse_node(child,e))
			return
		}
		if(typeof node !== 'object' || !node.kind) {
			return
		}
		// grant fresh nodes an id since lower layer needs it
		if(!node.id) { node.id = this.idgenerator; node.dirty = true; this.idgenerator++ }
		// visit group children
		if(node.kind == GROUP) {
			for(let [name,child] of Object.entries(node)) this.recurse_node(child,e)
		}
		// pass events to node?
		if(e && e.event && node.event) { node.view = this; node.event(e); }
		// repaint if dirty
		if(node.dirty && node.kind != GROUP) {
			node.dirty = false
			let copied = {}
			for(let [k,v] of Object.entries(node)) {
				if(typeof v === 'object') continue
				if(typeof v === 'function') continue
				copied[k]=v
			}
			broker_event("localhost:/orbital/service/view",JSON.stringify(copied))
		}
	}
	drawmouse(args) {
		BROKER.event({
			path:"localhost:/orbital/service/view",
			id:1555,
			kind:CIRCLE2D,
			x:args.x,
			y:args.y,
			w:20.0,
			h:20.0,
			text:"hello",
		})
	}
}


SERVICEFACTORY["localhost:/orbital/service/view"] = VIEW


///
/// A proxy for the rust side timer service - stuffed in this file for now - move to a separate file later TODO
///

class TIMER extends SERVICE {
	constructor() {
		super();
	}
	load(fragment) {
		this.fragment = fragment
		// TODO - eventually actually parse above request - but for now just send some ticks to me
		// tell broker to tell rust side timer to echo events to this javascript isolate as a whole
		BROKER.event({
			path:"localhost:/orbital/service/timer",
			command:"echo",
			millis:"1000",
			echo:LISTENPATH,
		})
	}
	events(args) {}
}

SERVICEFACTORY["localhost:/orbital/service/timer"] = TIMER







