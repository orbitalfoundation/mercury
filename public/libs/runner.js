

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Service abstraction
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


class SERVICE {
	update() {}
}

let SERVICEFACTORY = {
	"/camera": SERVICE,
	"/tensor": SERVICE,
	"/scripting": SERVICE,
};


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// An app
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class APP extends SERVICE {

	constructor(manifest) {
		super();
		let services = this.services = {}
		for(let [name,node] of Object.entries(manifest)) {
			if( typeof node === 'object' && node.kind != null) {
				let service = new SERVICEFACTORY[node.kind]()
				services[name]=service
				service.load(node.load)
			}
		}
	}

	update() {
		for(let [name,service] of Object.entries(this.services)) {
			service.update()
		}
	}

}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// an app runner for the javascript side
//
//		- presents an idea of users and applications and manifests
//
//		- there is a rust side 'system'; a broker and some services that are bootstrapped up
//
//		- this wrapper acts like a proxy or facade to that rust side capability (to reduce the labor to talk to the rust side)
//
//		- it's useful for the javascript side to be able to load, start, stop, wire up (rust side) services
//
//		- it's useful to invent an idea of a 'user' and a 'user area'
//
//		- it's useful to invent an idea of 'user managed services'; basically similar to a UNIX userland
//
//		- note that some services are off limits, others are singletons and so on
//
//		- note that for now i manually build this graph based on knowledge of the other side, but later it should be generated
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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
	load(manifest) {

		// for now just use a default party; login may not really be a sensible concept here... debate TODO
		let party = this.parties.default;

		// produce app out of a pile of services
		let app = new APP(manifest);

		// register app
		party.apps[manifest.name]=app
	}

	// update all apps, including newly registered apps
	update() {
		for(let [name1,party] of Object.entries(this.parties)) {
			for(let [name2,app] of Object.entries(party.apps)) {
				app.update()
			}
		}
	}

}
