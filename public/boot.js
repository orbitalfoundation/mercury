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

///////////////////// test logic

//message("/camera","start");
//message("/camera","stop");

// i guess these are getting out too early
sleep(1000);

// clock back face

message("/pixels",JSON.stringify({
	"id":0,
	"kind":2,
	"x":0,
	"y":0,
	"w":220.0,
	"h":220.0
}));

// second hand

let r = 0;

setInterval(1000,()=> {
	r += 3.1459*2.0/60.0;
	let node = {
		"id":1,
		"kind":1,
		"x":0.0,
		"y":0.0,
		"w":Math.sin(r)*100.0,
		"h":Math.cos(r)*100.0
	}
	message("/pixels",JSON.stringify(node));
})




// TODO receive msgs also:
// broker_post
// broker_observe
// broker_fetch ... -> get all msgs