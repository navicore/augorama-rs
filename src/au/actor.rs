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
use riker::actors::*;

use crate::au::model::{AuMsg, AuState, AuTelemetry};

fn fwd(ctx: &Context<AuMsg<Vec<AuTelemetry>>>, msg: AuMsg<Vec<AuTelemetry>>, sender: Sender) {
    let fmsg = AuMsg {
        path: msg.path.clone().split_off(1),
        ..msg
    };

    match msg.path.get(0) {
        Some(next_id) => {
            debug!(
                "{} receiving msg addressed to child {}",
                ctx.myself.name(),
                next_id
            );
            // note: is there a faster way to look up a child?
            let child = ctx.myself.children().find(|x| x.name() == next_id);
            match child {
                Some(sel) => {
                    debug!(
                        "{} forwarding to existing child {}",
                        ctx.myself.name(),
                        next_id
                    );
                    match sel.try_tell(fmsg, sender) {
                        Ok(_) => {}
                        _ => error!("not sent"),
                    }
                }
                _ => {
                    debug!(
                        "{} forwarding to newly created child {}",
                        ctx.myself.name(),
                        next_id
                    );
                    let props = AugieActor::props();
                    let new_actor = ctx.actor_of(props, &next_id.to_string()).unwrap();
                    new_actor.tell(fmsg, sender);
                }
            };
        }
        None => error!("path error: {}", ctx.myself.name()),
    }
}

pub struct AugieActor {
    state: AuState,
}

impl Actor for AugieActor {
    type Msg = AuMsg<Vec<AuTelemetry>>;

    fn recv(&mut self, ctx: &Context<AuMsg<Vec<AuTelemetry>>>, msg: AuMsg<Vec<AuTelemetry>>, sender: Sender) {
        if !msg.path.is_empty() {
            // it is a msg to a child
            fwd(ctx, msg, sender);
        } else {
            // else it is mine
            debug!("{} received msg", ctx.myself.name());

            let response = AuMsg { msg: Some(self.state.state.clone()), ..msg };
            let result = sender
                .unwrap()
                .try_tell(response, Some(ctx.myself().into()));
            match result {
                Ok(_) => debug!("{} sent reply", ctx.myself.name()),
                Err(_) => error!("NOT sent"),
            }

//            match serde_json::to_string(&self.state) {
//                Ok(json) => {
//                    let response = AuMsg { msg: Some(self.state.state.clone()), ..msg };
//                    let result = sender
//                        .unwrap()
//                        .try_tell(response, Some(ctx.myself().into()));
//                    match result {
//                        Ok(_) => debug!("{} sent reply", ctx.myself.name()),
//                        Err(_) => error!("NOT sent"),
//                    }
//                }
//                Err(_) => {
//                    error!("can not serialize state");
//                }
//            }
        }
    }
}

impl AugieActor {
    fn actor() -> Self {
        AugieActor {
            state: AuState { state: Vec::new() },
        }
    }
    pub fn props() -> BoxActorProd<AugieActor> {
        Props::new(AugieActor::actor)
    }
}
