
fn testme(b:&MessageSender, path: &str) {

	// shake off the lifetime scope of the borrow checker
	let b = b.clone();

	// a name
	let name = "scripting";

	// message handlers

	// spawn a thread and watch for events
	let _thread = std::thread::Builder::new().name(name.to_string()).spawn(move || {

		// https://github.com/danbev/learning-v8
		// https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs

		// Initialize V8
		let platform = v8::new_default_platform(0, false).make_shared();
		v8::V8::initialize_platform(platform);
		v8::V8::initialize();

		// An isolate is another copy of the v8 runtime for some unknown reason
		let isolate = &mut v8::Isolate::new(Default::default()); // v8::CreateParams::default() also works?

		// create a stack allocated handle scope
		let handle = &mut v8::HandleScope::new(isolate);

		// a "context"?
		let context = v8::Context::new(handle);

		// A "scope" in a context...?
		let scope = &mut v8::ContextScope::new(handle, context);

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// make an "object" that will become the global state, and stuff some callbacks into it
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// an "object" -> is apparently the equivalent of javascripts "const obj = {}"
		let myglobals = v8::ObjectTemplate::new(scope);

		// test: add a variable to an object template
		myglobals.set( v8::String::new(scope,"myvariable").unwrap().into(), v8::String::new(scope,"wheee").unwrap().into());

		// for some reason you have to declare your field count ahead of time - but it is assymetrical.
		// try set a magical field - cannot seem to do this ? set_internal_field does not exist?
		// https://stackoverflow.com/questions/16600735/what-is-an-internal-field-count-and-what-is-setinternalfieldcount-used-for
		// https://v8.dev/docs/embed
		myglobals.set_internal_field_count(1);

		// templates can set fields in general but they cannot get fields for some reason
		// handle.get( v8::String::new(handle,"myvariable").unwrap().into() );
		// let thing = myglobals.get( v8::String::new(scope,"myvariable").unwrap().into() );
		// println!("got {}",thing);

		// this would be more clean for variable fetching
		//let my_u8: u8 = "42".parse::<u8>().unwrap();
		//let my_u32: u32 = "42".parse::<u32>().unwrap();
		// or, to be safe, match the `Err`
		//match "foobar".parse::<i32>() {
		//  Ok(n) => do_something_with(n),
		//  Err(e) => weep_and_moan(),
		//}

		// test callback #1
		fn log_callback( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {

			// get context
			let context = scope.get_current_context();

			// get external from internal - preemptively unwrap because we believe it exists - ignore Some/None
			let ext = context.global(scope).get_internal_field(scope,0).unwrap();

			// cast it back
		    let ext = unsafe { v8::Local::<v8::External>::cast(ext) };

			// get it as a c pointer again
			let raw_ptr : *mut std::ffi::c_void = ext.value();
			let raw_ptr2 = raw_ptr as *mut MessageSender;

			// go back up to being a boxed messagesender
			let recovered = unsafe { Box::<MessageSender>::from_raw( raw_ptr2 ) };

			// send it a message as a test
			recovered.send(Message::Post("/camera".to_string(),"amazing".to_string()));

			// cannot directly do this because functions are not closures, and closures just plain are not allowed in v8 rust
			//	b.send( Message::Spawn("/camera".to_string(),"wow".to_string()));

			// unsafely try extract a value that was stuffed into a secret part of the context
			let mysender = context.global(scope).get_internal_field(scope,0).unwrap();
			let mysender = unsafe { v8::Local::<v8::Integer>::cast(mysender) };
			let mysender = mysender.value();
			println!("Found a value finally {}",mysender );

			// do something to peek at the javascript first argument
			let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
			println!("Logged: {}", message);

			// send something fun back
			let my_int: i32 = message.parse().unwrap();
			_retval.set(v8::Integer::new(scope, my_int).into());
		}
		myglobals.set( v8::String::new(scope,"log").unwrap().into(), v8::FunctionTemplate::new(scope,log_callback).into() );

		// test callback #2
		fn log2_callback( handle: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
			let message = args.get(0).to_string(handle).unwrap().to_rust_string_lossy(handle);
			println!("Rusty: Here is what I got in the callback as the first param: {}", message);
			let my_int: i32 = message.parse().unwrap();
			_retval.set(v8::Integer::new(handle, my_int).into());
		}
		myglobals.set( v8::String::new(scope,"log2").unwrap().into(), v8::FunctionTemplate::new(scope,log2_callback).into() );

		// another fn -> see https://github.com/denoland/rusty_v8/blob/fa8f636397822ba7dbb823d25dc637bf77f8b1ce/tests/test_api.rs#L1091-L1101
		// also see -> https://github.com/denoland/rusty_v8/issues/404
		// see https://github.com/denoland/deno/blob/ccd0d0eb79db6ad33095ca06e9d491a27379b87a/core/examples/http_bench.rs#L184-L195
		// see https://github.com/denoland/rusty_v8/blob/main/examples/process.rs
		//let create_message = v8::Function::new(scope,
		//	|scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue| {
		//		let message = v8::Exception::create_message(scope, args.get(0));
		//		let message_str = message.get(scope);
		//		rv.set(message_str.into())
		//  },
		//).unwrap();
		//myglobals.set( v8::String::new(scope,"msg").unwrap().into(), create_message.into() );

		// make the context global i guess? it becomes the global object
		let context = v8::Context::new_from_template(scope, myglobals);
		let scope = &mut v8::ContextScope::new(scope, context);

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// stuff our back pointer to the messaging system into the javascript layer so that it is visible to the fucking callback
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// this works but is pointless - i want to set on the actual global instance - where is that?
		//let object = myglobals.new_instance(scope).unwrap();
		//let fortytwo = v8::Integer::new(scope, 42).into();
		//let value = object.get_internal_field(scope, 0).unwrap();

		// it turns out that a context is an object...
		// can I write any kind of value at all into the internal field space?
		//let x = v8::String::new_from_utf8(isolate, "Process"); // not the right flavor of Local
		///let x = v8::String::new(scope,"test").into(); // dunno won't work - not "local"?
		//let x: v8::Local<v8::Value> = v8::Value::new(12); // does not build
		//let x = v8::Number::new(scope, 1.0).into();
		//let x = v8::Integer::new(scope, 42).into();

		// in rust parlance this moves the artifact to the heap along with a nominal concept of ownership
		let boxed_sender = Box::<MessageSender>::new(b);

		// now violate that trust by getting a raw pointer to the thing
		let boxed_ptr : *mut MessageSender = Box::into_raw(boxed_sender);

		// and explicitly more exactly cast raw ptr as a raw 'unknown' pointer because rust
		let raw_ptr = boxed_ptr as *mut std::ffi::c_void;

		// wrap that radioactive waste in a v8 compatible 'external'
		let ext = v8::External::new(scope,raw_ptr);

		// stuff external an internal field area - no idea what "into" means
		context.global(scope).set_internal_field(0,ext.into());


		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// run the javascript
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

		// js code
		let code = v8::String::new(scope, "let x = log(12) + log2(14); myvariable;").unwrap();

		// run it
		let script = v8::Script::compile(scope, code, None).unwrap();
		let result = script.run(scope).unwrap();
		let result = result.to_string(scope).unwrap();
		println!("scripting: {}", result.to_rust_string_lossy(scope));

		// here i guess i would not watch but just return , and do a try_recv() as a callback, can i do a sleep??
		// basically below should be obsolete: TODO remove

		// watch stuff forever
		let (s,r) = unbounded::<Message>();
		while let Ok(message) = r.recv() {
			match message {
				_ => {},
			}
		};

	});

}
