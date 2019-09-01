#[macro_use]
extern crate log;
extern crate env_logger;

use warp::{self, path, Filter};

fn main() {
    env_logger::init();
    debug!("starting server");
    let hello = path!("hello" / String).map(|name| {
        debug!("handling {}", name);
        format!("Hello, {}!", name)
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030));
}
