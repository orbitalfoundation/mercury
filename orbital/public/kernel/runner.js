
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Broker Wrapper - simply wraps access to rust side broker and all rust side services
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

let BROKER = {
	event: (blob) => {
		broker_event(blob.path,JSON.stringify(blob))
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// A javascript side service concept
///
/// - lets me have javascript side proxies of rust side services if I want
///
///	- a service can receive events in the current pattern
///
/// - also a factory for convenience
///
/// - also a message forwarding scheme for convenience
///
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class SERVICE {
	load(blob) {}
	event(args) {}
}

let SERVICEFACTORY = {
}

let SERVICEEVENTS = {
	handlers: {},
	add: function(event,handler) {
		if(!this.handlers[event]) this.handlers[event] = []
		this.handlers[event].push(handler)
	},
	event: function(event) {
		if(!this.handlers[event.event]) return
		this.handlers[event.event].forEach(handler=>handler(event));
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// App Runner -> "an app is a pile of services"
///
///		- There's a concept of a 'service' which is a proxy for a rust side service that can only be reached by messaging
///
///		- There's a concept of an 'app' which is a collection of services
///
///		- There's a concept of a 'manifest' that is used to produce an app
///
///		- I have some conception of a user, but it is weak right now
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class APP extends SERVICE {
	constructor(manifest) {
		super();
		// define container for services in the app
		let services = this.services = {}
		// walk through the  list of commands and services to produce
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
					// TODO later - pass the entirety of the blob
					if(service.load) service.load(node.load)
				}
			}
		}
	}
}

///
/// A javascript side concept for managing an idea of an app
///
///		- TODO there could arguably be a single namespace / graph on the javascript side for like /home/root and so on?
///		- TODO think about accounts a bit more and user folders - such as where to remember a running app
///

class RUNNER {

	constructor() {
		// a list of parties - unused right now
		this.parties = {
			default: {
				kind:"/party",
				name:"default",
				apps: {},
			}
		}
	}

	///
	/// attach a new app to the system... as specified by a manifest
	///

	load(manifest) {

		// for now just use a default party; login may not really be a sensible concept here... debate TODO
		let party = this.parties.default

		// produce app out of a pile of services
		let app = new APP(manifest)

		// register app under a party
		party.apps[manifest.name]=app

	}

}

let APPRUNNER = new RUNNER();

///
/// special magical bridge from rust side events to javascript logic
///

function special_callback(args) {
	SERVICEEVENTS.event(args)
}





