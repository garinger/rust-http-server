#![allow(dead_code, unused_imports, unused_variables)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;
use std::path::Path;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), Path::new(r"\public").display());
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}