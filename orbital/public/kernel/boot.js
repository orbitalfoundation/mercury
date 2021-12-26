
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// a boot / login / desktop scene fragment
//
//	- this is a scene graph describing a layout and how to handle user events - it is similar to HTML/Javascript
//
//	- a view service is largely driven by a scene graph that you supply
//
//	- (an 'immediate' mode is not super helpful because it's such a pain to throw stuff over the wall to rust)
//
//	- this is a scene graph that a view service can use to basically paint a display and handle user interactions
//
//	- it can have update and event handling logic - combined as a single event
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//
// post login desktop
//

let desktop_view_fragment = {
	mybackdrop: {
		kind:IMAGE,
		x:0.0,
		y:0.0,
		w:800.0,
		h:800.0,
		text:"/textures/matrix.jpg",
	},	
}

//
// a clock
//

let boot_view_clock = {
	kind:GROUP,
	image: {
		kind:IMAGE2D,
		x:0.0,
		y:0.0,
		w:800.0,
		h:800.0,
		text:"/textures/matrix.jpg",
	},
	face: {
		kind:CIRCLE2D,
		x:0,
		y:0,
		w:220.0,
		h:220.0,
		text:"hello",
	},
	sweep: {
		kind:LINE2D,
		x:0.0,
		y:0.0,
		w:0.0,
		h:0.0,
		text:"hello",
		seconds:0,
		event: function(e) {
			if(e.event != "tick") return
			if(!this.initialized) {
				const d = new Date()
				this.seconds = d.getSeconds()
				console.log("javascript:: clock: initialization -> seconds hand is exactly at " + this.seconds)
				this.initialized = 1
			}
			let r = 3.1459*2.0/60*this.seconds;
			this.seconds++;
			this.w = Math.sin(r)*100.0
			this.h = Math.cos(r)*100.0
			this.dirty = true
		}
	}
}

//
// a button
//
let x = -100
let y = -50
let w = 200
let h = 100

let mybutton = {

	kind:GROUP,
	x:0,
	y:0,
	w:w,
	h:h,

	//extends:"/prototypes/button",				// TODO i'd like to actually use a prototype - not used
	//xyz:[0,0,0],								// TODO improve notation and also move to 3d
	//whd:["80%","20%",0],						// TODO support relative parent dimensions?

	mybox: { kind:BOX2D, x:0, y:0, w:w, h:h, color:"green" },
	myline1: { kind:LINE2D, x:x, y:y, w:x, h:y+h, },
	myline2: { kind:LINE2D, x:x, y:y, w:x+w, h:y, },
	myline3: { kind:LINE2D, x:x, y:y+h, w:x+w, h:y+h, },
	myline4: { kind:LINE2D, x:x+w, y:y, w:x+w, h:y+h, },
	mytext: { kind:TEXT2D, x:0, y:0, w:0, h:0, text:"Please Login", },

	event: function(e) {
		if(this.timedtransition) {
			this.timedtransition--
			if(!this.timedtransition) {
				this.view.load(boot_view_clock)
			}
		}
		if(!e) return
		let overtop = true
		if(e.x < this.x - this.w/2) overtop = false
		if(e.x > this.x + this.w/2) overtop = false
		if(e.y < this.y - this.h/2) overtop = false
		if(e.y > this.y + this.h/2) overtop = false

		// perform action if all conditions met
		if(e.event == "mouseup" && overtop && this.selected) {
			this.timedtransition=2
		}

		// deselect if mouse up at all ever
		if(e.event=="mouseup") this.selected = 0

		// select only if overtop
		if(e.event=="mousedown" && overtop) this.selected = 1

		// if overtop and selected then make sure it is showing that state
		if((overtop && this.selected) && !this.showing) {
			this.mybox.color = "red"
			this.mybox.dirty = 1
			this.showing = 1
		}

		// if not overtop make sure it is showing that state
		if((!overtop || !this.selected) && this.showing) {
			this.mybox.color = "green"
			this.mybox.dirty = 1
			this.showing = 0
		}

		// ideally this could order the system to switch the view fragment; or change hide/show status
	}
}

// kind:ELEMENT // TODO handle recursive
let splash_text = {
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
}

//
// a boot page
//

let boot_view_fragment = {
	kind: GROUP,
	button: mybutton
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 
// boot app manifest
//
//		- an 'app' in orbital is a group of built-in services (actual rust threads) wired together (by messages)
//
//		- apps are declared in a 'manifest' which is basically an enumeration of services to run, security and wires
//
//		- the boot login page and the user desktop page is such an app
//
//		- the manifest declares that it wants a 'view' service to exist, and then passes it a scenegraph to paint
//
//		- the boot page lets a user login
//
//		- the desktop lets users load other apps and also shows you what is running, and how to start and stop them
//
//		- this javascript side is proxying rust side services to javascript for ease of use
//
//		- TODO explore security
//
//		- in this example i'm toying with routing timer ticks to the view; it is contrived but i am testing wires
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

let boot_app = {
	name:"boot",
	mytimer: {
		kind:"localhost:/orbital/service/timer",
		load:{
			millis:1000
		}
	},
	myview: {
		kind:"localhost:/orbital/service/view",
		load: boot_view_fragment,
	},
	mywire: {
		from:"mytimer",
		to:"myview",
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// app bootstrapping
//
//	- this call actually parses a manifest and actually fires up the services
//
//	- it does fall through once services are built
//
//	- the entire javascript isolate will be invoked over and over in a callback scheme based on events such as a timer
//
//	- when this is done it falls off the edge, but the javascript isolate is not destroyed, callbacks will drive things now
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

APPRUNNER.load(boot_app)

