
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// a boot / login / desktop app
//
//		- an 'app' in orbital is a group of built-in services wired together
//
//		- apps are declared in a 'manifest' which is basically an enumeration of services to run, security and wires
//
//		- the user desktop itself is such an app
//
//		- the below is an exploration of what a desktop needs to provide
//
//		- a desktop needs to let a user manage other apps, to fetch them, enumerate them, start and stop them
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// not used yet
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

let boot_view_fragment = {

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
				message("/view") 				// TODO this doesn't work, we need a formal way to revise what is on the display list - maybe the "id"
			}
		}
	}

}

//
// A boot app like any other app, it is a collection of services and wires in a manifest
//
//		- my convention is that a manifest is a graph that defines the services to bring up and their wires
//
//		- in this case it says it wants a display service to exist
//
//		- and it wants the display service to have a login_page scene graph fragment in it
//

let boot_app = {
	name:"boot",
	myview: {
		kind:"/view",							// requests that this exist (it's a device so it hopefully shall exist)
		load: boot_view_fragment,				// pass it something to do
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 
// bootup
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


// load and run this app
let apps = new RUNNER()
apps.load(boot_app)

// let callbacks drive stuff
function special_callback(args) {

//	console.log("event=" + args.event + " x=" + args.x + " y=" + args.y);

	if(args.event == "tick") {
		apps.update()
	}

	if(args.event == "mousemove") {

		// hack
		let mymouse = {
			id:1555,
			kind:CIRCLE2D,
			x:args.x,
			y:args.y,
			w:20.0,
			h:20.0,
			text:"hello",
		}
		message("/view",JSON.stringify(mymouse))
	}

}









