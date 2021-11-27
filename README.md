# Orbital Design Sketch Oct 26 2021

This is a sketch of the orbital browser. Despite being called a browser this browser doesn't support HTML. It is better phrased as an 'app runner' with strong security.

This particular repo is arranges all the parts and attempts to bring up a whole system from those parts; this includes a multi-threaded kernel, a message system, dynamically loaded blobs and a simple user interface.

## Running

cargo run

## PARTS AND TERMINOLOGY

1) Broker. This service runs at startup. This becomes a place where other services can register. Services are individual threads of execution that can receive and send messages (typically via the broker). The only way services can communicate with each other or with anything at all is via messages.

2) Messaging. There is a concept of a shared messaging namespace to allow services to talk to each other. The namespace is arranged (by convention) with some services mounted at canonical locations such as "/device/mouse".

3) Package Manager. Some services are built in, but other services can be fetched over the internet. There is a system for managing service revisions.

4) Factory. Services themselves are their own factories. Built-in services are built by hand at startup and are manually registered with the broker. Subsequent services can be instantiated by talking to existing services.

5) Security. There is security around each service instance.

6) Apps. Apps are collections of related services.

7) Manifest. Apps are described by manifests. Manifests define services and wires between services and security policies between each of the services as well.

## RUST SIDE BOOTSTRAPPING

1) Broker is started by hand.

2) Several core services are started by hand and registered with broker.

3) Rust side main() logic orders the scripting engine to run "boot.js"

## JAVASCRIPT SIDE BOOTSTRAPPING

1) App Runner. The boot.js logic passes a boot application manifest to an application runner / manager.

2) Boot Login. The boot manifest produces user interface (on the display service) and lets the user login to the desktop.

3) Desktop. The desktop produces a simple desktop that lets users fetch, start, stop and manage other apps.

