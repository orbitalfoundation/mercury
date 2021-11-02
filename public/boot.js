
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// General shared utils to stuff somewhere
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


let console = {
	log: function(one,two,three) {
		let args = Array.prototype.slice.call(arguments)
		message("/log",args.join(""))
	}
}

let setTimeout = (time,callback) => {
	sleep(time);
	callback();
}

let setInterval = (time,callback) => {
	while(true) {
		sleep(time);
		callback();
	}
}

sleep(100); // let system catch up


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Display related stuff - move later
//
// How this works:
//
// 		- The heavy lifting of the display engine is written in native rust.
//
// 		- It has a scene graph that it walks every frame to paint the display.
//
//		- In Javascript we are expected to feed the display engine fragments of a scene graph for it to paint
//
//		- I compose the entire user interface as a declarative scene graph here, with callbacks, to deliver user experience
//
//		- Generally speaking this parallels the idea of a DOM; it should be familiar to people who know HTML
//
//		- You can have an "immediate mode" ability such as "draw a circle now!" by feeding the engine a small graph fragment
//
//		- The display engine can deal with some 'kinds' of things innately - see types below.
//
//		- Engine is 3d focused but has 2d capabilities also.
//
//		- Nominally we have an idea of 'elements' - every display engine node in the graph is an 'element'
//
//		- Every element has some common properties, such as visibility, location, material, event handling, children
//
//		- Groups are a convenience concept, they are just elements.
//
//		- Every element innately is also able to render text (since text is so universally needed)
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

function scenehelper(scene) {
	for(let [name,node] of Object.entries(scene)) {
		if(!node.kind) return
		if(node.update) node.update(node)
		message("/pixels",JSON.stringify(node))
	}
}

function scenerun(scene) {
	setInterval(1000, () => { scenehelper(scene) })
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Display related prototypical layout nodes
//
// These are prototypical layout elements that can be overridden in a CSS like way
//
// TODO this is not implemented yet
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

let prototypes = {
	kind:ELEMENT,
	visible:false,
	element: {
		kind:ELEMENT,
		extend:ELEMENT, // this is a concept to allow a node to clone part of the graph fragment; to do css
		rewrite:ELEMENT, // this is a concept to let a node rewrite basic fragment styles; to do css
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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// system - a proxy for the rust side broker basically
//
// - TODO may want a semantic abstraction bridge
// - TODO define security
// - there is some design intent to use the same graph idea everywhere to in the future support embodied programming
// - every user is actually a shared account or place; this is a multiuser system
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//
// system is a proxy for the internal state of the rust side broker/services system at startup time
// it is proxied here on the theory that the js side will want to have it for reference and as a state model
// some services are singletons, others can be asked to produce clones of themselves
// TODO later this graph should be actually fetched from the kernel and should be durable
//

let systemstate = {
	service: { 
		camera: {
			kind: "/camera",
		},
		tensor: {
			kind: "/tensor",
		},
		scripting: {
			kind: "/scripting",
		},
		display: {
			kind: "/display",
		},
	},
	party: {
		party1: { // TODO this is an example of something i want to store persistently
			kind: "/party",
			name: "default user",
		}
	}
}

let system = {

	login: () => {
		// login a user in some way... presumably bind to a party, set a current focus perhaps, or return a handle
		// i think if we are making this into a persistent concept we need a way to save it somewhere - we need kernel storage
		// feels like the broker ends up owning that job of storing stuff?
		// i guess we permission them to fiddle with some portion of the graph
	},

	logout: (args) => {
		// supply party in graph
	},


	run: (args) => {
		// supply party in graph
		// register an app here; this may involve messages to the broker to bring up various services and wire them up
		// also i probably want to keep a copy of the graph
		// for example i should be able to start a scripting engine and pass it a document or file to load

		scenerun(args.display.params);
	},

	listapps: (args) => {
		// supply party in graph
		// get a list of all apps
	}

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// an app to produce login and desktop
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// display description of desktop

let desktop = {
	kind: ELEMENT,
	hidden:true,
	mybackdrop: {
		kind:IMAGE,
		x:0.0,
		y:0.0,
		w:800.0,
		h:800.0,
		text:"/textures/matrix.jpg",
	},	
}

// display description of login

let login = {

	// kind:ELEMENT // TODO handle recursive

	image: {
		id:1000,
		kind:IMAGE2D,
		x:0.0,
		y:0.0,
		w:800.0,
		h:800.0,
		text:"/textures/matrix.jpg",
	},
	face: {
		id:1001,
		kind:CIRCLE2D,
		x:0,
		y:0,
		w:220.0,
		h:220.0,
		text:"hello",
	},
	sweep: {
		id:1002,
		kind:LINE2D,
		x:0.0,
		y:0.0,
		w:0.0,
		h:0.0,
		text:"hello",
		seconds:0,
		update: (self)=> {
			if(!self.initialized) {
				const d = new Date()
				self.seconds = d.getSeconds()
				console.log("seconds is " + self.seconds)
				self.initialized = 1
			}
			let r = 3.1459*2.0/60*self.seconds;
			self.seconds++;
			self.w = Math.sin(r)*100.0
			self.h = Math.cos(r)*100.0
		}
	},
	text: {
		id:1003,
		kind:TEXT2D,
		x:0.0,
		y:0.0,
		w:0.0,
		h:0.0,
		text:
			`“What will the web look like in 10 years?”.
			We believe that reality is the platform for future computing.
			We believe the web in the future will be largely based around 3D and presence. We believe web interfaces will start to blend with the real world, that there will be virtual actions or verbs attached to everyday objects. This is an augmented future where information is attached to place; not presented separately. One that uses natural human gestures and motions; not by poking at a screen.
			We see web browsers not as text or content readers but as a “computational soup” running many durable and persistent user-agents with fine-grained permissions controls, all collaborating and communicating together, filtering and sense-making. Where computation moves between devices as needed, moving towards a decentralized future without walled gardens or large silo based social networks.
			`
	},

	// This is an entirely unbuilt idea but basically here is what I want to do for events:
	//
	// TODO - i need to be able to catch events here in order to actually do a transition to desktop unless i want to push that down to the rust side
	// TODO - i need a way to tell the display to remove the login and add the desktop (there needs to be a concept of attachment points)
	// TODO - i could do a router abstraction up here to allow for generic rather than explicit transitions, or i could turn fragments of the graph off and on

	mybutton: {
		//kind:ELEMENT,								// TODO leave this off for now so that it does not trigger the logic
		extends:"/prototypes/button",				// TODO i'd like to actually use a prototype
		xyz:[0,0,0],
		whd:["80%","20%",0],						// TODO I'd like to support fancier kinds of props
		text:"Please Login",
		events: {
			click: (self)=> {
				self.login.hidden = true			// TODO this doesn't work, but the idea is that maybe we can hide this and unhide the desktop?
				self.desktop.hidden = false 		// TODO this won't work, we need to invent a concept for being able to refer to other nodes in this graph
				message("/display") 				// TODO this doesn't work, we need a formal way to revise what is on the display list - maybe the "id"
			}
		}
	}

}

// bootstrap app

let bootapp = {
	party: 0,
	display: {
		kind:"/display",	// requests that this exist (it's a device so it hopefully shall exist)
		params: login,		// pass it something to do
	}
}

system.run(bootapp);

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 
// a random example app - incomplete thinking
//
//	- TODO - see if i can load and run this
//	- TODO - can i protect the above from this code
//
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

let myapp = {

	camera: {
		name:"mycamera",
		kind:"/service/camera/0",
		publish:"/events/camera/0/frame",
		description:"Turn on the hardware camera - it will start publishing frames to a default location"
	},

	tensor: {
		name:"mytensor",
		kind:"/service/tensorflow",
		load:"segmenter.ten",
		observe:"/service/camera/0/frames",
		publish:"segments",
		description:"Create an instance of a fresh tensorflow and training set, process frames and publish as segments",
	},

	display: {
		kind:"/display",
		scene: {
			window:{
				width: 100,
				height:100,
				events:{
					observe:{
						topic:"*mytensor/segments",
						callback: ()=>{
							// either modify graph here or update this scene and tell the system about the change
						}
					}
				}
			}
		}
	}

}
