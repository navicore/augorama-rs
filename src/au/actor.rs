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

use crate::au::msg::AuMsg;

pub struct AugieActor;

impl Actor for AugieActor {
    type Msg = AuMsg<String>;

    fn recv(&mut self, ctx: &Context<AuMsg<String>>, msg: AuMsg<String>, sender: Sender) {
        if !msg.path.is_empty() {
            let fmsg = AuMsg {
                msg: msg.msg,
                cmd: msg.cmd,
                path: msg.path.clone().split_off(1),
            };

            match msg.path.get(0) {
                Some(typ) => {
                    debug!(
                        "{} received msg addressed to child named {}",
                        ctx.myself.name(),
                        typ
                    );
                    let child = ctx.myself.children().find(|x| x.name() == typ);
                    match child {
                        Some(sel) => {
                            debug!("forwarding to existing child of type {}", typ);
                            match sel.try_tell(fmsg, sender) {
                                Ok(_) => debug!("sent"),
                                _ => debug!("not sent"),
                            }
                        }
                        _ => {
                            debug!("creating child actor of type {}", typ);
                            let props = AugieActor::props();
                            let new_actor = ctx.actor_of(props, &typ.to_string()).unwrap();
                            new_actor.tell(fmsg, sender);
                        }
                    };
                }
                None => error!("path error: {}", ctx.myself.name()),
            }
        } else {
            debug!("{} received msg addressed to itself", ctx.myself.name());
            // else it is mine
            let result = sender.unwrap().try_tell(msg, Some(ctx.myself().into()));
            match result {
                Ok(_) => debug!("{} sent reply", ctx.myself.name()),
                Err(_) => error!("NOT sent"),
            }
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
