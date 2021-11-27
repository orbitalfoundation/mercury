
use std::thread;
use serde_json::Value;
use crate::services::broker;
use rusty_v8 as v8;

/////////////////////////////////////////////////////////////////////////////////////////////////
///
/// scripting - a way to run raw js code - for apps and manifests look elsewhere
///
/////////////////////////////////////////////////////////////////////////////////////////////// 

pub fn scripting_service(path: &str) {

	let recv = broker::listen(path);

	// Initialize V8
	let platform = v8::new_default_platform(0, false).make_shared();
	v8::V8::initialize_platform(platform);
	v8::V8::initialize();

	// spawn a thread and watch for events
	thread::spawn(move || {

		while let Ok(blob) = recv.recv() {
			match blob {
				broker::Message::Event(_path,args) => {

					// parse args
					let v:Value = serde_json::from_str(&args).unwrap();
					let command : String = v["command"].as_str().unwrap().to_string();

					if command.eq("spawn") {

						// mount a listener

						let listen : String = v["listen"].as_str().unwrap().to_string();
						println!("scripting: new script listening at {}",listen);
						let recv2 = broker::listen(&listen);

						// spawn a thread and watch for events for this js

						thread::spawn(move || {

							// An isolate is another copy of the v8 runtime for some unknown reason
							let isolate = &mut v8::Isolate::new(Default::default()); // v8::CreateParams::default() also works?

							// create a stack allocated handle scope
							let handle = &mut v8::HandleScope::new(isolate);

							// a "context"?
							let context = v8::Context::new(handle);

							// A "scope" in a context...?
							let scope = &mut v8::ContextScope::new(handle, context);

							//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
							//
							// register broker_event() - allows the javascript to send events to the brokerage
							// register sleep() - to allow thread sleep - not promise based so this diverges from orthodox js
							//
							//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

							// define an "object" -> is apparently the equivalent of javascripts "const obj = {}"
							let myglobals = v8::ObjectTemplate::new(scope);

							// broker_event()
							fn callback1( scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
								let path = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
								let args = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
								broker::event_with_path(&path,&args)
							}
							myglobals.set( v8::String::new(scope,"broker_event").unwrap().into(), v8::FunctionTemplate::new(scope,callback1).into() );

							// sleep()
							fn callback2( _scope: &mut v8::HandleScope, _args: v8::FunctionCallbackArguments, mut _retval: v8::ReturnValue ) {
								//let _args0 = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
								//let _args1 = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
								thread::sleep(std::time::Duration::from_millis(1000));
							}
							myglobals.set( v8::String::new(scope,"sleep").unwrap().into(), v8::FunctionTemplate::new(scope,callback2).into() );

							// promote this new object to be 'global' - TODO this may be sloppy
							let context = v8::Context::new_from_template(scope, myglobals);
							let scope = &mut v8::ContextScope::new(scope, context);

							// TODO soonish - probably want to start a separate context for this

							let file : String = v["file"].as_str().unwrap().to_string();
							println!("scripting: broker running a new script {}",file);

							//
							// set a couple of globals
							//

							let sourcecode = ["let LISTENPATH = \"",&listen,"\""].join("");
							let sourcecode = v8::String::new(scope,&sourcecode).unwrap();
							let script = v8::Script::compile(scope, sourcecode, None).unwrap();
							let _result = script.run(scope).unwrap();

							//
							// build some common helpers
							//

							let contents = std::fs::read_to_string("public/kernel/common.js").expect("Load error");
							let sourcecode = v8::String::new(scope,&contents).unwrap();
							let script = v8::Script::compile(scope, sourcecode, None).unwrap();
							let result = script.run(scope).unwrap();
							let _result = result.to_string(scope).unwrap();

							//
							// build a thing that helps run apps, loads manifests and so on
							//

							let contents = std::fs::read_to_string("public/kernel/runner.js").expect("Load error");
							let sourcecode = v8::String::new(scope,&contents).unwrap();
							let script = v8::Script::compile(scope, sourcecode, None).unwrap();
							let result = script.run(scope).unwrap();
							let _result = result.to_string(scope).unwrap();

							//
							// build a wrapper for the view service
							//

							let contents = std::fs::read_to_string("public/kernel/view.js").expect("Load error");
							let sourcecode = v8::String::new(scope,&contents).unwrap();
							let script = v8::Script::compile(scope, sourcecode, None).unwrap();
							let result = script.run(scope).unwrap();
							let _result = result.to_string(scope).unwrap();

							//
							// invoke userland javascript - hopefully it returns
							//

							let contents = std::fs::read_to_string(file).expect("Load error");
							let sourcecode = v8::String::new(scope,&contents).unwrap();
							let script = v8::Script::compile(scope, sourcecode, None).unwrap();
							let result = script.run(scope).unwrap();
							let _result = result.to_string(scope).unwrap();

							// start handling events

							while let Ok(blob) = recv2.recv() {
								match blob {
									broker::Message::Event(_path,args) => {

										// trying to call a method... this is just too complex
										//v8::Local<v8::Value> foo_value = context->Global()->Get(v8::String::NewFromUtf8(isolate, "foo"));
										//let special = context.get(v8::String::new(scope,"special").unwrap().into());
										//let ext = context.global(scope).get_internal_field(scope,0).unwrap();

										// this works... it's probably very slow TODO
										// it gets passed a json blob which it is naturally able to decode... magically due to js
										let sourcecode = ["special_callback(",&args,")"].join("");
										let sourcecode = v8::String::new(scope,&sourcecode).unwrap();
										let script = v8::Script::compile(scope, sourcecode, None).unwrap();
										let _result = script.run(scope).unwrap();

									},
									_ => {}
								}
							}
						});

						return;
					}

				},
				_ => {},
			}
		};

	});

}


