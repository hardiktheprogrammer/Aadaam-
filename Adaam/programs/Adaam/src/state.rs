use anchor::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub name: String, // 4 +256
    pub profile: String,
    pub authority: PubKey,
    pub last_post_id: u8,
    pub post_count: u8,
}

// let user = {
//     name:
//     profile:

// }
