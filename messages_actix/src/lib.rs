#[macro_use]
extern crate actix_web;

// use actix_web::{middleware, web, App, HttpServer, Result};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;


static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct AppState {
	server_id: usize,
	request_count: Cell<usize>,
	messages: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    fn new(server_id: usize, request_count: Cell<usize>, messages: Arc<Mutex<Vec<String>>>) -> Self { Self { server_id, request_count, messages } }
}

#[derive(Serialize)]
struct  IndexResponse {
	server_id: usize,
	request_count: usize,
	messages: Vec<String>,
}

pub struct MessageApp {
	port: u16,	
}
 
impl MessageApp {
	pub fn new(port: u16) -> Self {
			MessageApp {port}
	}

	pub fn run(&self) -> std::io::Result<()> {
		println!("Starting http server: 127.0.0.1:{}", self.port);
		HttpServer::new(move || {
			App::new()
					.wrap(middleware::Logger::default())
					.service(index)
		})
		.bind(("127.0.0.1", self.port))?
		.workers(8)
		.run()
	}
}


#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
	let request_count = state.request_count.get() + 1;
	state.request_count.set(request_count);
	let ms = state.messages.lock().unwrap();
	
	Ok(web::Json(IndexResponse {
		server_id: state.server_id,
		request_count,
		messages: ms.clone(),
	}))
}

// `and_then` needs to return an `Option`, but `to_str` returned a `Result`. `ok` which takes data from the `Ok` variant of a `Result` and puts it inside the `Some` variant of an `Option`, otherwise it turns the `Err` variant of the `Result` into the `None` variant of `Option` and discards the error. 