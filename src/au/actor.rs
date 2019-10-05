//! The main building block of the Augorama system.  The actor ingests
//! `AuMsg` messages.
//!
//! These messages may be:
//!   * updates containing new telemetry to advance state.
//!   * queries for state information.
//!   * queries for journal records.

extern crate env_logger;
extern crate log;

use log::{debug, error};

use crate::au::msg::AuMsg;
use riker::actors::{Actor, ActorReference, BoxActorProd, Context, Props, Sender};

pub struct AugieActor;

impl Actor for AugieActor {
    type Msg = AuMsg<String>;

    fn recv(&mut self, ctx: &Context<AuMsg<String>>, msg: AuMsg<String>, sender: Sender) {
        // todo: 0 ejs make forwards vector? for slicing.
        // todo: 1 ejs check to see if this is a fwd msg
        // todo: 2 if no, echo it with world
        // todo: 3 if yes, pop top off of forward, make new msg, lookup or create child, send...

        // ejs todo: if (msg.forward.)
        //if (msg.forward.) {} else {}
        // if forwards len > 0
        for x in ctx.myself.children() {
            debug!("child found named {}", x.name())
        }

        // else it is mine
        let result = sender.unwrap().try_tell(msg, Some(ctx.myself().into()));
        match result {
            Ok(_) => debug!("sent"),
            Err(_) => error!("NOT sent"),
        }
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
