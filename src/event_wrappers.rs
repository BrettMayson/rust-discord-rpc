use crate::{EventHandlers, discord_user::DiscordUser};
use sys;
use libc;
use std::ffi::CStr;

pub(crate) unsafe extern "C" fn ready_wrapper<EH: EventHandlers>(user: *const sys::DiscordUser) {
    let req = DiscordUser {
        user_id: CStr::from_ptr((*user).userId)
            .to_string_lossy()
            .into_owned(),
        username: CStr::from_ptr((*user).username)
            .to_string_lossy()
            .into_owned(),
        discriminator: CStr::from_ptr((*user).discriminator)
            .to_string_lossy()
            .into_owned(),
        avatar: CStr::from_ptr((*user).avatar)
            .to_string_lossy()
            .into_owned(),
    };
    
    EH::ready(req);
}

pub(crate) unsafe extern "C" fn disconnected_wrapper<EH: EventHandlers>(
    errcode: libc::c_int,
    message: *const libc::c_char,
) {
    EH::disconnected(errcode as i32, &CStr::from_ptr(message).to_string_lossy());
}

pub(crate) unsafe extern "C" fn errored_wrapper<EH: EventHandlers>(
    errcode: libc::c_int,
    message: *const libc::c_char,
) {
    EH::errored(errcode as i32, &CStr::from_ptr(message).to_string_lossy());
}

pub(crate) unsafe extern "C" fn join_game_wrapper<EH: EventHandlers>(secret: *const libc::c_char) {
    EH::join_game(&CStr::from_ptr(secret).to_string_lossy());
}

pub(crate) unsafe extern "C" fn spectate_game_wrapper<EH: EventHandlers>(
    secret: *const libc::c_char,
) {
    EH::spectate_game(&CStr::from_ptr(secret).to_string_lossy());
}

pub(crate) unsafe extern "C" fn join_request_wrapper<EH: EventHandlers>(
    user: *const sys::DiscordUser,
) {
    let req = DiscordUser {
        user_id: CStr::from_ptr((*user).userId)
            .to_string_lossy()
            .into_owned(),
        username: CStr::from_ptr((*user).username)
            .to_string_lossy()
            .into_owned(),
        discriminator: CStr::from_ptr((*user).discriminator)
            .to_string_lossy()
            .into_owned(),
        avatar: CStr::from_ptr((*user).avatar)
            .to_string_lossy()
            .into_owned(),
    };
    
    EH::join_request(req);
}
