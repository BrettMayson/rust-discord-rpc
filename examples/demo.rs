extern crate discord_rpc;

use discord_rpc::{RPC, EventHandlers, RichPresence};

const APPLICATION_ID: &'static str = "378906438590005272";

struct Handlers;

impl EventHandlers for Handlers {
    fn ready() {
        println!("We're ready!");
    }
}

fn main() {
    let rpc = RPC::init::<Handlers>(APPLICATION_ID, false, None).unwrap();

    let presence = RichPresence {
        state: Some("IT motherFUCKING".to_string()),
        details: Some("works im CRYING".to_string()),
        ..Default::default()
    };
    rpc.update_presence(presence).unwrap();

    loop {
        rpc.run_callbacks();
    }
}
