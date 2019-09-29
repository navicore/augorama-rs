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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::{debug, info};
use riker::actors::*;
use riker::system::ActorSystem;
use warp::{self, path, Filter};

use crate::au::actor::AugieActor;
use crate::au::msg::AuCmd::Get;
use crate::au::msg::{AuForwards, AuMsg};

pub mod au;

/// blocking call to run server.  server will open a port and expect http requests.
pub fn serve() {
    env_logger::init();
    info!("starting actor space");

    let sys = Arc::new(Mutex::new(ActorSystem::new().unwrap()));
    let sys_shared = sys.clone();

    let roots: Arc<Mutex<HashMap<String, ActorRef<AuMsg<String>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let roots_shared: Arc<Mutex<HashMap<String, ActorRef<AuMsg<String>>>>> = roots.clone();

    // route for root level actors, ie: an actor type and an instance id
    let actor1_level = path!("actor" / String / String).map(move |typ: String, id| -> String {
        let sys_shared = sys_shared.lock().unwrap();

        let msg = AuMsg {
            msg: "haha".to_string(),
            cmd: Get,
            forward: AuForwards::default(),
        };

        // Check for a specific one.
        let mut roots_shared = roots_shared.lock().unwrap();
        let actor = match roots_shared.get(&typ) {
            Some(actor) => actor.clone(),
            None => {
                debug!("creating root actor of type {}", typ);
                let props = AugieActor::props();
                let new_actor = sys_shared.actor_of(props, &typ).unwrap();
                roots_shared.insert(typ.to_string(), new_actor.clone());
                new_actor
            }
        };

        //ejs todo ask:
        actor.tell(msg, None);

        //ejs todo result:
        format!("Hello {} {}!", typ, id)
    });

    // 2nd level actors, ie: an actor type and an instance id that is the child of a root actor.
    let actor2_level = path!("actor" / String / String / String / String).map(
        |parent, parent_id, typ, id| -> String {
            debug!(
                "handling parent type {} id {} type {} id {} ",
                parent, parent_id, typ, id
            );
            //ejs todo ask:
            //ejs todo result:
            format!("Hello {}'s {} {}!", parent_id, typ, id)
        },
    );

    let routes = actor2_level.or(actor1_level);
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
