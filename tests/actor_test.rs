extern crate augorama;

use std::{thread, time};

#[test]
fn actor_messaging_works() {
    thread::spawn(move || {
        augorama::serve()
    });
    let two_seconds = time::Duration::from_millis(1000);
    thread::sleep(two_seconds);

    match reqwest::get("http://localhost:3030/actor/person/Mary") {
        Ok(mut result) => {
            match result.text() {
                Ok(t) => assert_eq!(t, "Hello person Mary!"),
                Err(_) => assert!(false)
            }
        }
        Err(_) => assert!(false)
    }
}
