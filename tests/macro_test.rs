//! # learning macros
//!
//!
extern crate augorama_derive;
use augorama_derive::HelloMac;

#[derive(HelloMac)]
struct Pancakes;

#[test]
fn make_trait_works() {
    Pancakes::hiya();
}
