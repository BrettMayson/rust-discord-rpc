use super::DiscordUser;

pub trait EventHandlers {
    fn ready(_user: DiscordUser) {}

    fn errored(_errcode: i32, _message: &str) {}

    fn disconnected(_errcode: i32, _message: &str) {}

    fn join_game(_secret: &str) {}

    fn spectate_game(_secret: &str) {}

    fn join_request(_request: DiscordUser) {}
}

use sys;

pub(crate) fn wrap<EH: EventHandlers>() -> sys::DiscordEventHandlers {
    use crate::event_wrappers::*;

    sys::DiscordEventHandlers {
        ready: Some(ready_wrapper::<EH>),
        disconnected: Some(disconnected_wrapper::<EH>),
        errored: Some(errored_wrapper::<EH>),
        joinGame: Some(join_game_wrapper::<EH>),
        spectateGame: Some(spectate_game_wrapper::<EH>),
        joinRequest: Some(join_request_wrapper::<EH>),
    }
}
