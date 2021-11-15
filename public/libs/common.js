
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Here are some global javascript helpers that I would like to stuff into every js context - for now I stuff it in here
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
