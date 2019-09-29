#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://doc.rust-lang.org/"
)]
extern crate augorama;
extern crate env_logger;
extern crate log;

use augorama::serve;

/// entry point to start the server
///
fn main() {
    serve();
}
