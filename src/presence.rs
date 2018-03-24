use std::time::SystemTime;

pub struct RichPresence {
    /// The user's current party status. Maximum of 128 bytes.
    ///
    /// For example: `"Looking to Play"`, `"Playing Solo"`, `"In a Group"`...
    pub state: Option<String>,

    /// What the player is currently doing. Maximum of 128 bytes.
    ///
    /// For example: `"Competitive - Captain's Mode"`, `"In Queue"`, `"Unranked PvP"`...
    pub details: Option<String>,

    /// Time of game start. Including will show time as "elapsed".
    pub start_time: Option<SystemTime>,

    /// Time of game end. Including will show time as "remaining".
    pub end_time: Option<SystemTime>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub large_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub large_image_text: Option<String>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub small_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub small_image_text: Option<String>,

    /// ID of the player's party, lobby, or group. Maximum of 128 bytes.
    pub party_id: Option<String>,

    /// Current size of the player's party, lobby, or group.
    pub party_size: Option<u32>,

    /// Maximum size of the player's party, lobby, or group.
    pub party_max: Option<u32>,

    /// Unique hashed string for Spectate button. Maximum of 128 bytes.
    pub spectate_secret: Option<String>,

    /// Unique hashed string for chat invitations and Ask to Join. Maximum of 128 bytes.
    pub join_secret: Option<String>,
}
