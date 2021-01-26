#[path = "./events.rs"]
pub mod events;

use crossbeam_channel as cc;
use events::*;
use node_bindgen::derive::node_bindgen;
use serde::Serialize;
use std::sync::{Arc, Mutex};

type RequestChannel = (cc::Sender<Request>, cc::Receiver<Request>);

struct Session {
    val: f64,
    state: Arc<Mutex<SessionState>>,
    request_channel: RequestChannel,
}
impl Session {
    fn debug(&self) {
        if let Ok(mut s) = self.state.lock() {
            let js_string = serde_json::to_string(&*s).expect("Serialization failed");
            println!("s: {}", js_string);
        }
    }

    pub fn send_event<T: Serialize>(&self, event: T) {
        // send_js_event_queue(self.queue.clone(), self.callback.clone(), event);
    }
    fn consume_event(&mut self, event: Event) {
        self.state.lock().unwrap().cached_result = Some("event".to_owned());
    }
}

fn long_operation() -> String {
    std::thread::sleep(std::time::Duration::from_millis(100));
    "long_operation()".to_owned()
}

enum Request {
    One,
    Two,
    PrintSession,
    Shutdown,
}

#[derive(Debug, Serialize)]
pub enum Event {
    Tick(u64),
    Done,
}

#[derive(Debug, Serialize)]
struct SessionState {
    cached_result: Option<String>,
    is_running: bool,
}

#[node_bindgen]
impl Session {
    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        println!("create new object");
        Self {
            val,
            state: Arc::new(Mutex::new(SessionState {
                cached_result: None,
                is_running: false,
            })),
            request_channel: cc::unbounded(),
        }
    }

    #[node_bindgen]
    fn doRequestOne(&self) -> bool {
        println!("Rust: do_request");
        let _ = self.request_channel.0.send(Request::One);
        true
    }


    #[node_bindgen]
    fn doRequestTwo(&self) -> bool {
        println!("Rust: do_request");
        let _ = self.request_channel.0.send(Request::Two);
        true
    }

    #[node_bindgen]
    fn plus_one(&self) -> f64 {
        println!("plus 1");
        self.val + 1.0
    }

    #[node_bindgen(getter)]
    fn value(&self) -> f64 {
        println!("value");
        self.val
    }

    #[node_bindgen(mt)]
    fn start<F: Fn(Events) + Send + 'static>(&self, cb: F) {
        let protected_state = self.state.clone();
        let request_rx = self.request_channel.1.clone();
        std::thread::spawn(move || {
            println!("rust thread running....");
            if let Ok(mut state) = protected_state.lock() {
                state.is_running = true;
                loop {
                    let request = request_rx.recv();
                    println!("Rust: request arrived in thread");

                    match request {
                        Ok(Request::PrintSession) => {
                            println!("PrintSession event");
                            // self.debug();
                        }
                        Ok(Request::One) => {
                            println!("Rust: handle One: {:?}", state.cached_result);
                            let res = long_operation();
                            state.cached_result = Some(res);
                            println!("we have the mutable self");
                            let event = StreamUpdated {
                                signature: "StreamUpdated".to_string(),
                                bytes: 1,
                                rows: 2,
                            };
                            cb(Events::StreamUpdated(event))
                            // self.send_event(Event::Tick(1u64));
                        }
                        Ok(Request::Two) => {
                            let event = StreamUpdated {
                                signature: "StreamUpdated".to_string(),
                                bytes: 3,
                                rows: 4,
                            };
                            cb(Events::StreamUpdated(event));
                            println!("Two: {:?}", state.cached_result);
                            // self.send_event(Event::Tick(2u64));
                        }
                        Ok(Request::Shutdown) => {
                            let event = StreamUpdated {
                                signature: "StreamUpdated".to_string(),
                                bytes: 5,
                                rows: 6,
                            };
                            cb(Events::StreamUpdated(event));
                            println!("Shutdown: {:?}", state.cached_result);
                            break;
                        }
                        Err(e) => {
                            println!("error: {}", e);
                        }
                    }
                }
            }

            // std::thread::sleep(std::time::Duration::from_millis(100));
            // session.lock().unwrap().send_event(Event::Done);
            // std::thread::sleep(std::time::Duration::from_millis(100));
        });
        println!("exit ...");
    }
}
