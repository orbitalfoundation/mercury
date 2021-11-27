# EVENTS Nov 21 2021

See also https://github.com/orbitalweb/orbitalweb.github.io/wiki/Event-Handling (this is more recent)

## Namespace

There is a shared namespace

	localhost:		-> traffic to local machine

	localhost:/home	-> user accounts
	localhost:/home/root -> root account
	localhost:/home/root/apps    -> some root account app manifests, that are not yet running
	localhost:/home/root/scripts -> some root acount scripts (not quite full blown apps - no formal manifest), that are not yet running
	localhost:/home/root/running -> root account apps that are running (may have been produced from apps but not necessarily so!)

## Events use of Namespace

There is a shared system namespace that services can register to listen to events within, and publish to. This follows a pubsub model. Events are always published to a path, and all listeners on that path get the event.

An "application" is a collection of services wired together (with named event paths). Services can be distributed over the internet, and in fact they can move around dynamically, they are not necessarily always on device.

To send a message to a service you address it by name in a planetary and global namespace that uses the same notation as DNS. Here are some built-in services and their paths:

	localhost:/orbital/service/broker			-> local broker instance
	localhost:/orbital/service/log
	localhost:/orbital/service/scripting
	localhost:/orbital/service/camera
	localhost:/orbital/service/tensor
	localhost:/orbital/service/timer
	localhost:/orbital/service/wasm
	localhost:/orbital/service/view
	orbital.eth/service/broker			-> orbital bridge gateway in the ENS namespace
	pinata.eth/service/broker			-> the pinata IFPS gateway

## API

1) Broker::event(). Sending. From native rust or scripting you can send a message to a path. This is useful to drive a service (make something do something).

   broker::event(payload)
   broker::event_with_path(path,payload)

2) Broker::listen(). From native rust or from a scripting layer you can ask the broker itself to add you as a listener on a path - so that all messages to that path go to you. This is useful if you are a "service" that does work based on inbound messages. Scripting languages don't get an actual dedicated "receiver" back, rather the entire scripting module has a global event receiver and all messages arrive there and you have to sort them out by hand.

   receiver = broker::listen(path)
   broker::listen_with_sender(path,crossbeam_sender_channel)

3) Broker:unlisten(). TBD. I haven't bothered implementing this yet.

4) Broker::route(). This is deprecated (for now). I used to have a capability where you could "wire" one path to another, basically connecting two services. This means listening to traffic on one path and forwarding it to another path (such that any listeners on that other path also get it). But I think it is better for one service to specifically ask another service if it can listen to traffic there.

## Event Payloads

What do you actually send as a message? This is largely a set of conventions.

1) "Ordinary events" - Order a service to do some work:

   {
   	   path:"localhost:/orbital/services/view" ,			<- where the message is going to
   	   command:"attach",						<- command is a convention for the receiver service
   	   args:{									<- args is an optional convention for arguments for the command
   	   		scenepath:"/myscene/mycube",
   	   		kind:"cube",
			whd:[1,1,1],
			ypr:[0,0,0],
			xyz:[0,0,0],
			color:"blue"
   	   }
   }

2) "Factory events" - Ask a service to produce a child instance from a factory (most built-in services offer this power)

	{
		path:"localhost:/orbital/services/scripting",
		command:"spawn",
		args:{
			script:"localhost:/orbital/accounts/root/scripts/boot.js",	<- run a script found on local disk here
			listen:"localhost:/orbital/accounts/root/running/boot.js",	<- have that script listen to traffic here
			echo:  "localhost:/orbital/accounts/root/running/other.js", <- have that script send any published state to here
		}
	}

3) "Rebroadcast events" - Ask a service to re-broadcast its own outbound events to you explicitly on a separate channel. This is probably not going to be needed much - a better pattern is to effectively ask a service to mint a new copy of itself and send traffic to you specifically. Also, another way to "wire" services together would be to directly ask the broker to do it - but I've decided to deprecate that.

	{
		path:"localhost:/orbital/services/view",
		command:"echo",
		args:{
			echo:"localhost:/orbital/accounts/root/running/boot.js",
		}
	}

## Security




