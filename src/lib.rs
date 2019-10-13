#![doc(html_root_url = "https://github.com/navicore/augorama-rs")]
#![doc(html_favicon_url = "https://onextent.com/favicon.ico")]
#![doc(html_logo_url = "https://onextent.com/OnExtentLogo_RGB.png")]

//! The Augorama module implements the entry point to the Augorama actor system.
//!
//! Augorama is a server hosting a graph of digital twins (Augorama actors) of real and virtual
//! things in the world.  An actor can answer questions about its twin.  Answers can include
//! facts about its twin's current state, explanations about current state referencing past state
//! (explainability), and predictions about the twin's future state (AI).
//!
//! The actors maintain their state by watching telemetry collected about their twins.  Their state
//! consists of counts and sums and statistics.  Their state is backed by event sourcing
//! persistence.
//!
//! The telemetry driving the advancement over time of actor state is collected from an HTTP(S)
//! endpoint.  The data posted to the endpoint is structured - json by default - of any structure.
//! Telemetry is extracted from the structure with path specifications - `jsonpath` by default.
//! The separation of incoming data schema from telemetry via path means any structured data can be
//! posted to the endpoint, that there is no "Augorama data schema".  If a source can emit json, it
//! can be used to create telemetry necessary to maintain a digital twin (Augorama actor).

extern crate env_logger;
extern crate log;

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

use futures::executor::block_on;
use futures::future::RemoteHandle;
use log::{debug, info};
use riker::actors::*;
use riker::system::ActorSystem;
use riker_patterns::ask::*;
use warp::{self, Filter};

use crate::au::actor::AugieActor;
use crate::au::model::AuOperator;
use crate::au::model::AuOperator::*;
use crate::au::model::{AuMsg, AuTelemetry};

pub mod au;

type AuActorRef = ActorRef<AuMsg<Vec<AuTelemetry>>>;

fn safe_path(path: Vec<String>) -> Vec<String> {
    let mut p: Vec<String> = Vec::new();
    for x in path.iter() {
        p.push(x.clone().to_lowercase().to_string())
    }
    p
}

fn tell_actor(
    root: String,
    path: Vec<String>,
    op: AuOperator,
    data: Option<Vec<AuTelemetry>>,
    sys_shared: MutexGuard<ActorSystem>,
    mut roots_shared: MutexGuard<HashMap<String, AuActorRef, RandomState>>,
) -> String {
    debug!("handling {} {} {:?}", op, root, path);

    let aumsg: AuMsg<Vec<AuTelemetry>> = AuMsg {
        data,
        op,
        path: safe_path(path),
    };

    let actor = match roots_shared.get(&root) {
        Some(actor) => {
            debug!("found existing root {}", root);
            actor.clone()
        }
        None => {
            debug!("creating root {}", root);
            let props = AugieActor::props();
            let new_actor: ActorRef<AuMsg<Vec<AuTelemetry>>> =
                sys_shared.actor_of(props, &root).unwrap();
            roots_shared.insert(root.to_string(), new_actor.clone());
            new_actor
        }
    };

    actor.tell(aumsg, None);
    String::from("Accepted")
}

fn ls_actor(
    root: String,
    path: Vec<String>,
    cmd: AuOperator,
    msg: Option<Vec<AuTelemetry>>,
    sys_shared: MutexGuard<ActorSystem>,
    mut roots_shared: MutexGuard<HashMap<String, AuActorRef, RandomState>>,
) -> Vec<String> {
    debug!("handling {} {} {:?}", cmd, root, path);
    let aumsg: AuMsg<Vec<AuTelemetry>> = AuMsg {
        data: msg,
        op: cmd,
        path: safe_path(path),
    };

    let actor = match roots_shared.get(&root) {
        Some(actor) => {
            debug!("found existing root {}", root);
            actor.clone()
        }
        None => {
            debug!("creating root {}", root);
            let props = AugieActor::props();
            let new_actor = sys_shared.actor_of(props, &root).unwrap();
            roots_shared.insert(root.to_string(), new_actor.clone());
            new_actor
        }
    };

    let sys = sys_shared.borrow().deref();
    let res: RemoteHandle<AuMsg<Vec<AuTelemetry>>> = ask(sys, &actor, aumsg);
    let response = block_on(res);

    response.path
}

fn ask_actor(
    root: String,
    path: Vec<String>,
    cmd: AuOperator,
    msg: Option<Vec<AuTelemetry>>,
    sys_shared: MutexGuard<ActorSystem>,
    mut roots_shared: MutexGuard<HashMap<String, AuActorRef, RandomState>>,
) -> Option<Vec<AuTelemetry>> {
    debug!("handling {} {} {:?}", cmd, root, path);
    let aumsg: AuMsg<Vec<AuTelemetry>> = AuMsg {
        data: msg,
        op: cmd,
        path: safe_path(path),
    };

    let actor = match roots_shared.get(&root) {
        Some(actor) => {
            debug!("found existing root {}", root);
            actor.clone()
        }
        None => {
            debug!("creating root {}", root);
            let props = AugieActor::props();
            let new_actor = sys_shared.actor_of(props, &root).unwrap();
            roots_shared.insert(root.to_string(), new_actor.clone());
            new_actor
        }
    };

    let sys = sys_shared.borrow().deref();
    let res: RemoteHandle<AuMsg<Vec<AuTelemetry>>> = ask(sys, &actor, aumsg);
    let response = block_on(res);

    response.data
}

/// blocking call to run server.  server will open a port and expect http requests.
pub fn serve() {
    type ActorRoots = Arc<Mutex<HashMap<String, ActorRef<AuMsg<Vec<AuTelemetry>>>>>>;

    env_logger::init();
    info!("starting actor space");

    let sys = Arc::new(Mutex::new(ActorSystem::new().unwrap()));
    let roots: ActorRoots = Arc::new(Mutex::new(HashMap::new()));

    //let sys_shared0 = sys.clone();
    //let sys_shared0p = sys.clone();
    //let sys_shared0c = sys.clone();
    //let roots_shared0 = roots.clone();
    //let roots_shared0p = roots.clone();
    let roots_shared0c = roots.clone();
    // todo: route 0

    //let sys_shared1 = sys.clone();
    //let sys_shared1p = sys.clone();
    //let sys_shared1c = sys.clone();
    //let roots_shared1 = roots.clone();
    //let roots_shared1p = roots.clone();
    //let roots_shared1c = roots.clone();
    // todo: route 1

    let sys_shared2 = sys.clone();
    let sys_shared2p = sys.clone();
    let sys_shared2c = sys.clone();
    let roots_shared2 = roots.clone();
    let roots_shared2p = roots.clone();
    let roots_shared2c = roots.clone();
    let post_route_2 = warp::path("actor")
        .and(warp::post2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(
            move |root_typ: String, id: String, json: Vec<AuTelemetry>| -> String {
                tell_actor(
                    root_typ,
                    vec![id],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared2p.lock().unwrap(),
                    roots_shared2p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

    //let sys_shared3 = sys.clone();
    //let sys_shared3p = sys.clone();
    let sys_shared3c = sys.clone();
    //let roots_shared3 = roots.clone();
    //let roots_shared3p = roots.clone();
    let roots_shared3c = roots.clone();
    // todo: route 3

    let sys_shared4 = sys.clone();
    let sys_shared4p = sys.clone();
    let sys_shared4c = sys.clone();
    let roots_shared4 = roots.clone();
    let roots_shared4p = roots.clone();
    let roots_shared4c = roots.clone();
    let post_route_4 = warp::path("actor")
        .and(warp::post2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child_typ: String,
                  id: String,
                  json: Vec<AuTelemetry>|
                  -> String {
                tell_actor(
                    root_typ,
                    vec![root_id, child_typ, id],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared4p.lock().unwrap(),
                    roots_shared4p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

    //let sys_shared5 = sys.clone();
    //let sys_shared5p = sys.clone();
    let sys_shared5c = sys.clone();
    //let roots_shared5 = roots.clone();
    //let roots_shared5p = roots.clone();
    let roots_shared5c = roots.clone();
    // todo: route 5

    let sys_shared6 = sys.clone();
    let sys_shared6p = sys.clone();
    let sys_shared6c = sys.clone();
    let roots_shared6 = roots.clone();
    let roots_shared6p = roots.clone();
    let roots_shared6c = roots.clone();
    let post_route_6 = warp::path("actor")
        .and(warp::post2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child2_id: String,
                  child_typ: String,
                  id: String,
                  json: Vec<AuTelemetry>|
                  -> String {
                tell_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child_typ, id,
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared6p.lock().unwrap(),
                    roots_shared6p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

    //let sys_shared7 = sys.clone();
    //let sys_shared7p = sys.clone();
    let sys_shared7c = sys.clone();
    //let roots_shared7 = roots.clone();
    //let roots_shared7p = roots.clone();
    let roots_shared7c = roots.clone();
    // todo: route 7

    let sys_shared8 = sys.clone();
    let sys_shared8p = sys.clone();
    let sys_shared8c = sys.clone();
    let roots_shared8 = roots.clone();
    let roots_shared8p = roots.clone();
    let roots_shared8c = roots.clone();
    let post_route_8 = warp::path("actor")
        .and(warp::post2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child_typ: String,
                  child2_id: String,
                  child2_typ: String,
                  id: String,
                  json: Vec<AuTelemetry>|
                  -> String {
                tell_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child_typ, id,
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared8p.lock().unwrap(),
                    roots_shared8p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

    //let sys_shared9 = sys.clone();
    //let sys_shared9p = sys.clone();
    let sys_shared9c = sys.clone();
    //let roots_shared9 = roots.clone();
    //let roots_shared9p = roots.clone();
    let roots_shared9c = roots.clone();
    // todo: route 9

    let sys_shared10 = sys.clone();
    let sys_shared10p = sys.clone();
    let sys_shared10c = sys.clone();
    let roots_shared10 = roots.clone();
    let roots_shared10p = roots.clone();
    let roots_shared10c = roots.clone();
    let post_route_10 = warp::path("actor")
        .and(warp::post2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child_typ: String,
                  child2_id: String,
                  child2_typ: String,
                  child3_id: String,
                  child3_typ: String,
                  id: String,
                  json: Vec<AuTelemetry>|
                  -> String {
                tell_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child3_typ,
                        child3_id, child_typ, id,
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared10p.lock().unwrap(),
                    roots_shared10p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

    let get_route_2 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .map(
            move |root_typ: String, id: String| -> Option<Vec<AuTelemetry>> {
                ask_actor(
                    root_typ,
                    vec![id],
                    Ask,
                    None,
                    sys_shared2.lock().unwrap(),
                    roots_shared2.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let get_route_4 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child_typ: String,
                  id: String|
                  -> Option<Vec<AuTelemetry>> {
                ask_actor(
                    root_typ,
                    vec![root_id, child_typ, id],
                    Ask,
                    None,
                    sys_shared4.lock().unwrap(),
                    roots_shared4.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let get_route_6 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child_typ: String,
                  id: String|
                  -> Option<Vec<AuTelemetry>> {
                ask_actor(
                    root_typ,
                    vec![root_id, child1_typ, child1_id, child_typ, id],
                    Ask,
                    None,
                    sys_shared6.lock().unwrap(),
                    roots_shared6.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let get_route_8 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child2_id: String,
                  child3_typ: String,
                  child3_id: String,
                  child_typ: String,
                  id: String|
                  -> Option<Vec<AuTelemetry>> {
                ask_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child3_typ,
                        child3_id, child_typ, id,
                    ],
                    Ask,
                    None,
                    sys_shared8.lock().unwrap(),
                    roots_shared8.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let get_route_10 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child2_id: String,
                  child3_typ: String,
                  child3_id: String,
                  child4_typ: String,
                  child4_id: String,
                  child_typ: String,
                  id: String|
                  -> Option<Vec<AuTelemetry>> {
                ask_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child3_typ,
                        child3_id, child4_typ, child4_id, child_typ, id,
                    ],
                    Ask,
                    None,
                    sys_shared10.lock().unwrap(),
                    roots_shared10.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let child_route_0 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path("children"))
        .map(move || -> Vec<String> {
            let mut child_names: Vec<String> = Vec::new();
            for x in roots_shared0c.lock().unwrap().keys() {
                //child_names.push(x.name().to_string());
                child_names.push(x.clone().to_string())
            }
            child_names
        })
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_1 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(move |root_typ: String| -> Vec<String> {
            ls_actor(
                root_typ,
                Vec::new(),
                Ls,
                None,
                sys.lock().unwrap(),
                roots.lock().unwrap(),
            )
        })
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_2 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(move |root_typ: String, id: String| -> Vec<String> {
            ls_actor(
                root_typ,
                vec![id],
                Ls,
                None,
                sys_shared2c.lock().unwrap(),
                roots_shared2c.lock().unwrap(),
            )
        })
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_3 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String, root_id: String, id: String| -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, id],
                    Ls,
                    None,
                    sys_shared3c.lock().unwrap(),
                    roots_shared3c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_4 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, child_typ, id],
                    Ls,
                    None,
                    sys_shared4c.lock().unwrap(),
                    roots_shared4c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_5 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, child1_typ, child_typ, id],
                    Ls,
                    None,
                    sys_shared5c.lock().unwrap(),
                    roots_shared5c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_6 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, child1_typ, child1_id, child_typ, id],
                    Ls,
                    None,
                    sys_shared6c.lock().unwrap(),
                    roots_shared6c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_7 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, child1_typ, child1_id, child_typ, id],
                    Ls,
                    None,
                    sys_shared7c.lock().unwrap(),
                    roots_shared7c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_8 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child2_id: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child_typ, id,
                    ],
                    Ls,
                    None,
                    sys_shared8c.lock().unwrap(),
                    roots_shared8c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_9 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![root_id, child1_typ, child1_id, child2_typ, child_typ, id],
                    Ls,
                    None,
                    sys_shared9c.lock().unwrap(),
                    roots_shared9c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    let child_route_10 = warp::path("actor")
        .and(warp::get2())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("children"))
        .map(
            move |root_typ: String,
                  root_id: String,
                  child1_typ: String,
                  child1_id: String,
                  child2_typ: String,
                  child2_id: String,
                  child3_typ: String,
                  child3_id: String,
                  child_typ: String,
                  id: String|
                  -> Vec<String> {
                ls_actor(
                    root_typ,
                    vec![
                        root_id, child1_typ, child1_id, child2_typ, child2_id, child3_typ,
                        child3_id, child_typ, id,
                    ],
                    Ls,
                    None,
                    sys_shared10c.lock().unwrap(),
                    roots_shared10c.lock().unwrap(),
                )
            },
        )
        .map(|reply: std::vec::Vec<String>| warp::reply::json(&reply));

    // ejs todo: create macros to tersely manage arbitrarily long paths - manage all routes with DRY
    // ejs todo: create macros to tersely manage arbitrarily long paths - manage all routes with DRY
    // ejs todo: create macros to tersely manage arbitrarily long paths - manage all routes with DRY
    let routes = child_route_10
        .or(child_route_9)
        .or(child_route_8)
        .or(child_route_7)
        .or(child_route_6)
        .or(child_route_5)
        .or(child_route_4)
        .or(child_route_3)
        .or(child_route_2)
        .or(child_route_1)
        .or(child_route_0)
        .or(post_route_10)
        .or(post_route_8)
        .or(post_route_6)
        .or(post_route_4)
        .or(post_route_2)
        .or(get_route_10)
        .or(get_route_8)
        .or(get_route_6)
        .or(get_route_4)
        .or(get_route_2);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
