use riker::actors::*;
use std::collections::LinkedList;

pub struct AugieActor;

#[derive(Clone)]
pub struct AugieMsg<T> {
    pub to: String,
    pub msg: T,
    pub forward: Option<LinkedList<String>>,
}

impl std::fmt::Display for AugieMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.forward.is_none() {
            write!(f, "(msg: {} to: {})", self.msg, self.to)
        } else {
            write!(f, "(msg: {} to: {} forward: {})", self.msg, self.to, "self.forward")
        }
    }
}

impl std::fmt::Debug for AugieMsg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.forward.is_none() {
            write!(f, "(msg: {} to: {})", self.msg, self.to)
        } else {
            write!(f, "(msg: {} to: {} forward: {})", self.msg, self.to, "self.forward")
        }
    }
}

impl Actor for AugieActor {
    type Msg = AugieMsg<String>;

    fn recv(&mut self, _ctx: &Context<AugieMsg<String>>, msg: AugieMsg<String>, _sender: Sender) {
        debug!("Received: {}", msg);
        for x in _ctx.myself.children() {
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
