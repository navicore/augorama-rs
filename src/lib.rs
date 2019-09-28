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

pub fn serve() {
    env_logger::init();
    info!("starting actor space");

    let sys = Arc::new(Mutex::new(ActorSystem::new().unwrap()));
    let sys_shared = sys.clone();

    let roots: Arc<Mutex<HashMap<String, ActorRef<AuMsg<String>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let roots_shared: Arc<Mutex<HashMap<String, ActorRef<AuMsg<String>>>>> = roots.clone();

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

        actor.tell(msg, None);

        //demo::main();
        format!("Hello {} {}!", typ, id)
    });

    let actor2_level = path!("actor" / String / String / String / String).map(
        |parent, parent_id, typ, id| -> String {
            debug!(
                "handling parent type {} id {} type {} id {} ",
                parent, parent_id, typ, id
            );
            //augieactor::main();
            format!("Hello {}'s {} {}!", parent_id, typ, id)
        },
    );

    let routes = actor2_level.or(actor1_level);
    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
