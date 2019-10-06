//! # Integration Tests
//!
//! Starts up an Augorama space in a thread and accesses the actors via http client.
//!
extern crate augorama;

use std::{thread, time};

#[test]
fn actor_ask_works() {
    thread::spawn(move || augorama::serve());
    let two_seconds = time::Duration::from_millis(1000);
    thread::sleep(two_seconds);

    match reqwest::get("http://localhost:3030/actor/person/Mary") {
        Ok(mut result) => match result.text() {
            Ok(t) => assert_eq!(t, "[]"),
            Err(_) => assert!(false),
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn actor_tell_works() {
    thread::spawn(move || augorama::serve());
    let two_seconds = time::Duration::from_millis(1000);
    thread::sleep(two_seconds);

    let client = reqwest::Client::new();
    let result = client
        .post("http://localhost:3030/actor/person/Erdal/pet/Spot")
        .body(r#"[{"name": "my.name", "value": 1.3, "datetime": "2019-10-06T13:20:16Z"}]"#)
        .send();

    match result {
        Ok(mut response) => match response.text() {
            Ok(t) => assert_eq!(t, "Accepted"),
            Err(_) => assert!(false),
        },
        Err(_) => assert!(false),
    }

    match reqwest::get("http://localhost:3030/actor/person/Erdal/pet/Spot") {
        Ok(mut result) => match result.text() {
            Ok(t) => assert_eq!(
                t,
                r#"[{"datetime":"2019-10-06T13:20:16Z","name":"my.name","value":1.3}]"#
            ),
            Err(_) => assert!(false),
        },
        Err(_) => assert!(false),
    }
}
