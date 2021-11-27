
TODO WAYS TO WIRE SERVICES TOGETHER

	- manifests can be declared which bundle a bunch of services into an 'application'

		let app = {
			path:"/accounts/root/apps/myapp",
			mytimer:{
				target:"/service/timer", // service to talk to
				command:"spawn", // command for it to do
				millis:"1000",  // 
				listen:"*mytimer", // in the case of spawning, where to mount a listener relatively to app
				echo:"*"
			},
			myview:{
				target:"/service/view",
				command:"echo",
				echo:"*",
			},
			myboot:{
				target:"/service/scripting",
				command:"spawn",
				script:"/public/root/app/boot.js",
				listen:"*"
			},
		}

