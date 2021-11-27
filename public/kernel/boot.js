
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
		init: (self)=> {
			// I think i may not have an init - update below is fine?
			// 3js has no real concept of events
			// html dom has a concept of listeners that can be attached to objects; effectively collision detection
			// i have a variety of options; i could run some init code, and listen to things at will
			// i could also register listeners on objects themselves
		},
		update: (self)=> {
			if(!self.initialized) {
				const d = new Date()
				self.seconds = d.getSeconds()
				console.log("javascript:: clock: initialization -> seconds hand is exactly at " + self.seconds)
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

	// a button nov 14
	// TBD - not done
	//
	//		 - exploring event handling
	//
	//		- and maybe prototypes also
	//
	//		- i think the button could itself register to listen to events through a service exposed somehow to us here
	//		- then it can force a display transition to a new graph fragment (a desktop)
	//		- it might make sense to have a router abstraction

	mybutton: {
		//kind:ELEMENT,								// TODO leave this off for now so that it does not trigger the logic
		extends:"/prototypes/button",				// TODO i'd like to actually use a prototype
		xyz:[0,0,0],
		whd:["80%","20%",0],						// TODO I'd like to support fancier kinds of props
		text:"Please Login",

		// this is another way to do events
		init:(self)=>{

			// explicitly register here?
		},

		// or the display engine can scan for these and attach them
		events: {
			click: (self)=> {
				self.login.hidden = true			// TODO this doesn't work, but the idea is that maybe we can hide this and unhide the desktop?
				self.desktop.hidden = false 		// TODO this won't work, we need to invent a concept for being able to refer to other nodes in this graph
				broker_event("localhost:/orbital/service/view") 		// TODO this doesn't work, we need a formal way to revise what is on the display list - maybe the "id"
			}
		}
	}

}

//
// This is an example boot app
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
		kind:"localhost:/orbital/service/view",	// request a service to exist
		load: boot_view_fragment,				// pass it something to do
	}
	// TODO -> explore making this richer with wires here
	// TODO -> try out security as well
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 
// bootup
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

APPRUNNER.run(boot_app)

