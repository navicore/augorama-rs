//! The main building block of the Augorama system.  The actor ingests
//! `AuMsg` messages.
//!
//! These messages may be:
//!   * updates containing new telemetry to advance state.
//!   * queries for state information.
//!   * queries for journal records.

extern crate env_logger;
extern crate log;

use log::{debug, warn};
use riker::actors::*;

use crate::au::msg::AuMsg;

pub struct AugieActor;

impl Actor for AugieActor {
    type Msg = AuMsg<String>;

    fn recv(&mut self, _ctx: &Context<AuMsg<String>>, msg: AuMsg<String>, _sender: Sender) {
        debug!("Received: {}", msg);
        // todo: 0 ejs make forwards vector? for slicing.
        // todo: 1 ejs check to see if this is a fwd msg
        // todo: 2 if no, echo it with world
        // todo: 3 if yes, pop top off of forward, make new msg, lookup or create child, send...
        for _x in _ctx.myself.children() {
            //x.
        }
        warn!("want to do something with: {}", msg);
        // todo: something
    }
}

impl AugieActor {
    fn actor() -> Self {
        AugieActor
    }
    pub fn props() -> BoxActorProd<AugieActor> {
        Props::new(AugieActor::actor)
    }
}
