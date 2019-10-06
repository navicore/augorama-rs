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
use crate::au::model::AuOperator::Ask;
use crate::au::model::{AuMsg, AuTelemetry};

pub mod au;

type AuActorRef = ActorRef<AuMsg<Vec<AuTelemetry>>>;

fn tell_actor(
    root: String,
    path: Vec<String>,
    cmd: AuOperator,
    msg: Option<Vec<AuTelemetry>>,
    sys_shared: MutexGuard<ActorSystem>,
    mut roots_shared: MutexGuard<HashMap<String, AuActorRef, RandomState>>,
) -> String {
    debug!("handling Tell {} {} {:?}", cmd, root, path);
    let aumsg: AuMsg<Vec<AuTelemetry>> = AuMsg {
        data: msg,
        op: cmd,
        path,
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

fn ask_actor(
    root: String,
    path: Vec<String>,
    cmd: AuOperator,
    msg: Option<Vec<AuTelemetry>>,
    sys_shared: MutexGuard<ActorSystem>,
    mut roots_shared: MutexGuard<HashMap<String, AuActorRef, RandomState>>,
) -> Option<Vec<AuTelemetry>> {
    debug!("handling Ask {} {} {:?}", cmd, root, path);
    let aumsg: AuMsg<Vec<AuTelemetry>> = AuMsg {
        data: msg,
        op: cmd,
        path,
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
    let sys_shared1 = sys.clone();
    let sys_shared1p = sys.clone();
    let sys_shared2 = sys.clone();
    let sys_shared2p = sys.clone();
    let sys_shared3 = sys.clone();
    let sys_shared3p = sys.clone();
    let sys_shared4 = sys.clone();
    let sys_shared4p = sys.clone();
    let sys_shared5 = sys.clone();
    let sys_shared5p = sys.clone();

    let roots: ActorRoots = Arc::new(Mutex::new(HashMap::new()));
    let roots_shared1 = roots.clone();
    let roots_shared1p = roots.clone();
    let roots_shared2 = roots.clone();
    let roots_shared2p = roots.clone();
    let roots_shared3 = roots.clone();
    let roots_shared3p = roots.clone();
    let roots_shared4 = roots.clone();
    let roots_shared4p = roots.clone();
    let roots_shared5 = roots.clone();
    let roots_shared5p = roots.clone();

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
                    sys_shared1p.lock().unwrap(),
                    roots_shared1p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

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
                    vec![root_id.clone(), child_typ.clone(), id.clone()],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared2p.lock().unwrap(),
                    roots_shared2p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

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
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child2_typ.clone(),
                        child2_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared3p.lock().unwrap(),
                    roots_shared3p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

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
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child2_typ.clone(),
                        child2_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared4p.lock().unwrap(),
                    roots_shared4p.lock().unwrap(),
                )
            },
        )
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::ACCEPTED));

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
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child2_typ.clone(),
                        child2_id.clone(),
                        child3_typ.clone(),
                        child3_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    AuOperator::Tell,
                    Some(json),
                    sys_shared5p.lock().unwrap(),
                    roots_shared5p.lock().unwrap(),
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
                    sys_shared1.lock().unwrap(),
                    roots_shared1.lock().unwrap(),
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
                    vec![root_id.clone(), child_typ.clone(), id.clone()],
                    Ask,
                    None,
                    sys_shared2.lock().unwrap(),
                    roots_shared2.lock().unwrap(),
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
                    vec![
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    Ask,
                    None,
                    sys_shared3.lock().unwrap(),
                    roots_shared3.lock().unwrap(),
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
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child2_typ.clone(),
                        child2_id.clone(),
                        child3_typ.clone(),
                        child3_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    Ask,
                    None,
                    sys_shared4.lock().unwrap(),
                    roots_shared4.lock().unwrap(),
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
                        root_id.clone(),
                        child1_typ.clone(),
                        child1_id.clone(),
                        child2_typ.clone(),
                        child2_id.clone(),
                        child3_typ.clone(),
                        child3_id.clone(),
                        child4_typ.clone(),
                        child4_id.clone(),
                        child_typ.clone(),
                        id.clone(),
                    ],
                    Ask,
                    None,
                    sys_shared5.lock().unwrap(),
                    roots_shared5.lock().unwrap(),
                )
            },
        )
        .map(|reply: Option<std::vec::Vec<au::model::AuTelemetry>>| warp::reply::json(&reply));

    let post_routes =
        post_route_10.or(post_route_8.or(post_route_6.or(post_route_4.or(post_route_2))));
    let get_routes = get_route_10.or(get_route_8.or(get_route_6.or(get_route_4.or(get_route_2))));
    warp::serve(get_routes.or(post_routes)).run(([127, 0, 0, 1], 3030));
}
