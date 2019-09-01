#[macro_use]
extern crate log;
extern crate env_logger;

mod demo;

use warp::{self, path, Filter};

fn main() {
    env_logger::init();
    info!("starting server");
    let hello = path!("hello" / String).map(|name| {
        debug!("handling {}", name);
        demo::main();
        format!("Hello, {}!", name)
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030));
}
