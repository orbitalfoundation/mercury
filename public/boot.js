///////////////////// common utilities

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

///////////////////// scene helper

const KIND = {
	LINE:1,
	CIRCLE:2,
	TEXT:3,
}

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

///////////////////// a test scene

let text = 
`“What will the web look like in 10 years?”.
We believe that reality is the platform for future computing.
We believe the web in the future will be largely based around 3D and presence. We believe web interfaces will start to blend with the real world, that there will be virtual actions or verbs attached to everyday objects. This is an augmented future where information is attached to place; not presented separately. One that uses natural human gestures and motions; not by poking at a screen.
We see web browsers not as text or content readers but as a “computational soup” running many durable and persistent user-agents with fine-grained permissions controls, all collaborating and communicating together, filtering and sense-making. Where computation moves between devices as needed, moving towards a decentralized future without walled gardens or large silo based social networks.
`


let scene = {
	face: {
		id:1,
		kind:KIND.CIRCLE,
		x:0,
		y:0,
		w:220.0,
		h:220.0,
		text:"hello",
	},
	sweep: {
		id:2,
		kind:KIND.LINE,
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
		id:3,
		kind:KIND.TEXT,
		x:0.0,
		y:0.0,
		w:0.0,
		h:0.0,
		text:text,		
	}
}

scenerun(scene);

/*

chores

	- basics
		- stroke style
		- fill style
		- text style
		- text font
		- vectors xyz, ypr, whd
		- thickness
		- bevel
		- padding
		- margin
		- maybe type should be a string?
		- can i have a like css concept where i can define basic properties on a class?

	- more types
		- display -> a whole display surface; can only have one
		- layout
		- area -> may as well be 3d
		- area -> nesting and layout concepts may be here as well
		- box -> relative to parent scope
		- circle
		- image
		- text; scaling
		- paths; lines
		- built in input boxes and stuff? or build up here?

	- view engine
		- can the v8 parser handle raw js thrown at it rather than json stringify?
		- can i not bother manufacturing nodes on the other end but just pass the raw state through?
		- deal with string carriage returns
		- js errors are poor

	- receive messages also

	- scene parser here
		- have a richer concept of memoization - like; pass it a startup and then let it run...
		- time - we want things to run only at 60fps ... so what is a way to do that ? variable speed would be nice also
		- maybe you register handlers rather than update just being called?
		- should handle children arrays as well as hashes
		- should support peeking at the child and detecting if it is a node type


*/

