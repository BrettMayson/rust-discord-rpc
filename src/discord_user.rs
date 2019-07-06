#[derive(Default, Clone, Hash, PartialEq, Debug)]
pub struct DiscordUser {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}