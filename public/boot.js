///////////////////// common utilities

let console = {
	log: function(one,two,three) {
		let args = Array.prototype.slice.call(arguments)
		message("/log",args.join(""))
	}
}

///////////////////// test logic

message("/camera","start");
message("/camera","stop");

let x = {
	a:12,
	b:"hello",
	c:"start",
}

message("/camera",x.a)

console.log("asdf","aasdf","eee");