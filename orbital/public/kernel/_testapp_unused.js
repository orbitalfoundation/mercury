
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 
// incomplete thinking
//
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

let myapp = {

	camera: {
		name:"mycamera",
		kind:"/service/camera/0",
		publish:"/events/camera/0/frame",
		description:"Turn on the hardware camera - it will start publishing frames to a default location"
	},

	tensor: {
		name:"mytensor",
		kind:"/service/tensorflow",
		load:"segmenter.ten",
		observe:"/service/camera/0/frames",
		publish:"segments",
		description:"Create an instance of a fresh tensorflow and training set, process frames and publish as segments",
	},

	display: {
		kind:"/display",
		scene: {
			window:{
				width: 100,
				height:100,
				events:{ // an idea
					observe:{
						topic:"*mytensor/segments",
						callback: ()=>{
							// either modify graph here or update this scene and tell the system about the change
						}
					}
				}
			}
		}
	}
}

