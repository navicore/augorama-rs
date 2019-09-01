use riker::actors::*;
use std::time::Duration;

struct MyActor;

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self, _ctx: &Context<String>, msg: String, _sender: Sender) {
        debug!("Received: {}", msg);
        warn!("want to do something with: {}", msg);
        // todo: something
    }
}

// provide factory and props methods
impl MyActor {
    fn actor() -> Self {
        MyActor
    }

    fn props() -> BoxActorProd<MyActor> {
        Props::new(MyActor::actor)
    }
}

// start the system and create an actor
pub fn main() {
    let sys = ActorSystem::new().unwrap();

    let props = MyActor::props();
    let my_actor = sys.actor_of(props, "my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    std::thread::sleep(Duration::from_millis(500));
}
