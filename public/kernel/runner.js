
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// App Runner
//
//		- There's a concept of a 'service' which is a proxy for a rust side service that can only be reached by messaging
//
//		- There's a concept of an 'app' which is a collection of services
//
//		- There's a concept of a 'manifest' that is used to produce an app
//
//		- I have some conception of a user, but it is weak right now
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


class SERVICE {
	update() {}
}

let SERVICEFACTORY = {
};

class APP extends SERVICE {

	constructor(manifest) {
		super();
		// these are the services in the app
		let services = this.services = {}
		// walk through the  list of commands
		for(let [name,node] of Object.entries(manifest)) {
			// pick out commands to make a service - this is somewhat implicit and arguably could be formalized better in the grammar
			if( typeof node === 'object' && node.kind != null) {
				// find the service from a factory if any - services on rust side do have javascript side proxies for convenience
				let factory = SERVICEFACTORY[node.kind]
				if(!factory) {
					console.error("script::service:: error - missing factory for " + node.kind)
				} else {
					// this makes a proxy for a built in service
					// TODO later security -> there's no guarantee that built-in is going to be allowed to run!
					let service = new factory()
					// remember it
					services[name]=service
					// for now let's hardcode it to just load whatever is in this field
					// TODO later - pass all the props
					service.load(node.load)
				}
			}
		}
		// - TODO this would be a good time to do wiring between services, but also the service.load() above can do some of that
	}

	update() {
		for(let [name,service] of Object.entries(this.services)) {
			service.update()
		}
	}

}

class RUNNER {

	constructor() {
		// a list of parties
		this.parties = {
			default: {
				kind:"/party",
				name:"default",
				apps: {},
			}
		}
	}

	// attach a new app to the system... as specified by a manifest
	run(manifest) {

		// for now just use a default party; login may not really be a sensible concept here... debate TODO
		let party = this.parties.default;

		// produce app out of a pile of services
		let app = new APP(manifest);

		// register app
		party.apps[manifest.name]=app

		// - could run init logic on app services?

		// TODO refine later - for now just run one update at least
		// this.update()
	}

	// update all - this may not be needed once apps can listen to events such as tick events TODO
	update() {
		for(let [name1,party] of Object.entries(this.parties)) {
			for(let [name2,app] of Object.entries(party.apps)) {
				app.update()
			}
		}
	}

}

let APPRUNNER = new RUNNER();



//
// event propagation from rust...
//
// this is currently a bit messy: I need some global method in javascript userland to catch rust side events and this is it
// 
// on this side we ask a system service to start sending us tick events
//
// and also as a hack I am watching for mouse events and painting some ux for that - this can move to an ordinary app later
//
// - TODO - refine this to propagate events nicely through all apps
//


function special_callback(args) {

	// update all on a tick

	if(args.event == "tick") {
		APPRUNNER.update()
	}

	// hack - hammer a message into the rust side display engine - TODO move to an app and send events more nicely later

	if(args.event == "mousemove") {

		let mymouse = {
			id:1555,
			kind:CIRCLE2D,
			x:args.x,
			y:args.y,
			w:20.0,
			h:20.0,
			text:"hello",
		}

		broker_event("localhost:/orbital/service/view",JSON.stringify(mymouse))

	}

}

console.log("We see listen port here as " + LISTENPATH );

// build a message for the timer, telling it to publish state to this path

let msg = {
	path:"localhost:/orbital/service/timer",
	command:"echo",
	millis:"1000",
	echo:LISTENPATH,
}

broker_event(msg.path,JSON.stringify(msg));





