# Orbital design sketch oct 26 2021

## TODO

	- the broker probably needs to save state to disk, and also handle graph fragments

	- show off login
		- a photo button
		- feedback clicking
		- basically send the whole graph to the display engine if i want - or send frags?
		- but activate the login portion first either way?
		- it would be super cool if layout and event handling was handled behind the wall!
		- but it does not have to be, i can in fact write javascript land widgets

	- show off desktop view
		- handle input
		- let you see apps
		- let you run an app from a url
		- let you wire and unwire apps from parts
		- synthesize outputs to view (there is only one view)
		- i like an idea of little cute cards like weather cards
		- maybe show off a decorators concept where things can be decorated or enhanced
		- maybe have a css concept also to decorate or customize the graph
		- show off security

	- show off some cv
		- https://google.github.io/mediapipe/solutions/objectron.html

	- display helper -stuff
		- this thing passes graphs to the back end display
		- receive messages also
		- have a richer concept of memoization - like; pass it a startup and then let it run...?
		- time - we want things to run only at 60fps ... so what is a way to do that ? variable speed would be nice also
		- maybe you register handlers rather than update just being called?
		- should handle children arrays as well as hashes
		- should support peeking at the child and detecting if it is a node type?
		- i am keen to play with physicality of interfaces; weight, semi-regular shapes, physics?
		- need callbacks for events back up to here
		- need to be able to write to nested fragments of the graph cleanly, not just roots
		- probably need to not use serde at all
		- can the v8 parser handle raw js thrown at it rather than json stringify?
		- can i not bother manufacturing nodes on the other end but just pass the raw state through?
		- deal with string carriage returns
		- js errors are poor
		- maybe try a wgpu based approach - diy?
