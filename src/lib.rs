extern crate discord_rpc_sys as sys;
extern crate libc;

mod presence;
pub use presence::RichPresence;

#[derive(Default, Clone, Hash, PartialEq, Debug)]
pub struct JoinRequest {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

#[derive(Clone, Hash, PartialEq, Debug)]
pub enum JoinRequestReply {
    No,
    Yes,
    Ignore,
}

pub trait EventHandlers {
    fn ready() {}

    fn errored(_errcode: i32, _message: &str) {}

    fn disconnected(_errcode: i32, _message: &str) {}

    fn join_game(_secret: &str) {}

    fn spectate_game(_secret: &str) {}

    fn join_request<R: FnOnce(JoinRequestReply)>(_request: JoinRequest, _respond: R) {}
}

pub struct RPC;

use std::ffi::{CStr, CString, NulError};
use std::ptr;

impl RPC {
    pub fn init<EH: EventHandlers>(
        app_id: &str,
        auto_register: bool,
        steam_id: Option<&str>,
    ) -> Result<RPC, NulError> {
        extern "C" fn ready_wrapper<EH: EventHandlers>() {
            EH::ready();
        }
        unsafe extern "C" fn disconnected_wrapper<EH: EventHandlers>(
            errcode: libc::c_int,
            message: *const libc::c_char,
        ) {
            EH::disconnected(errcode as i32, &CStr::from_ptr(message).to_string_lossy());
        }
        unsafe extern "C" fn errored_wrapper<EH: EventHandlers>(
            errcode: libc::c_int,
            message: *const libc::c_char,
        ) {
            EH::errored(errcode as i32, &CStr::from_ptr(message).to_string_lossy());
        }
        unsafe extern "C" fn join_game_wrapper<EH: EventHandlers>(secret: *const libc::c_char) {
            EH::join_game(&CStr::from_ptr(secret).to_string_lossy());
        }
        unsafe extern "C" fn spectate_game_wrapper<EH: EventHandlers>(secret: *const libc::c_char) {
            EH::spectate_game(&CStr::from_ptr(secret).to_string_lossy());
        }
        unsafe extern "C" fn join_request_wrapper<EH: EventHandlers>(
            join_request: *const sys::DiscordJoinRequest,
        ) {
            let req = JoinRequest {
                user_id: CStr::from_ptr((*join_request).userId)
                    .to_string_lossy()
                    .into_owned(),
                username: CStr::from_ptr((*join_request).username)
                    .to_string_lossy()
                    .into_owned(),
                discriminator: CStr::from_ptr((*join_request).discriminator)
                    .to_string_lossy()
                    .into_owned(),
                avatar: CStr::from_ptr((*join_request).avatar)
                    .to_string_lossy()
                    .into_owned(),
            };
            EH::join_request(req, |reply| {
                sys::Discord_Respond(
                    (*join_request).userId,
                    match reply {
                        JoinRequestReply::No => sys::DISCORD_REPLY_NO,
                        JoinRequestReply::Yes => sys::DISCORD_REPLY_YES,
                        JoinRequestReply::Ignore => sys::DISCORD_REPLY_IGNORE,
                    } as libc::c_int,
                );
            });
        }

        let mut sys_handlers = sys::DiscordEventHandlers {
            ready: Some(ready_wrapper::<EH>),
            disconnected: Some(disconnected_wrapper::<EH>),
            errored: Some(errored_wrapper::<EH>),
            joinGame: Some(join_game_wrapper::<EH>),
            spectateGame: Some(spectate_game_wrapper::<EH>),
            joinRequest: Some(join_request_wrapper::<EH>),
        };

        unsafe {
            sys::Discord_Initialize(
                CString::new(app_id)?.into_raw(),
                &mut sys_handlers,
                auto_register as libc::c_int,
                match steam_id {
                    None => ptr::null(),
                    Some(id) => CString::new(id)?.into_raw(),
                },
            );
        }

        Ok(RPC)
    }

    pub fn update_presence(&self, presence: RichPresence) -> Result<(), NulError> {
        let sys_presence = presence.to_sys()?;

        unsafe {
            sys::Discord_UpdatePresence(&sys_presence);
        }

        Ok(())
    }

    pub fn clear_presence(&self) {
        unsafe {
            sys::Discord_ClearPresence();
        }
    }

    /// Invokes any pending callbacks from Discord on the calling thread. This
    /// function is allegedly thread safe.
    pub fn run_callbacks(&self) {
        unsafe {
            sys::Discord_RunCallbacks();
        }
    }
}

impl Drop for RPC {
    fn drop(&mut self) {
        unsafe {
            sys::Discord_Shutdown();
        }
    }
}
