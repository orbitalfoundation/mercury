

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
/// Manage a local version of the view scenegraph and send changes to real view engine
///

class VIEW extends SERVICE {
	load(fragment) {
		this.fragment = fragment
	}
	update() {
		for(let [name,node] of Object.entries(this.fragment)) {
			if(!node.kind) return
			if(node.update) node.update(node)
			broker_event("localhost:/orbital/service/view",JSON.stringify(node))
		}
	}
}

///
/// Register javascript wrapper modules with a javascript version of the rust side services layer
///

SERVICEFACTORY["localhost:/orbital/service/view"] = VIEW
