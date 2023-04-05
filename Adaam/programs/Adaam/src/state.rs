use anchor::prelude::*;

#[account]
#[derive(Default)]
pub struct UsersAccount {
    pub name: String,      // 4 +256
    pub profile: String,   // 4 + 2864
    pub authority: PubKey, //32

    pub last_post_id: u8, //1
    pub post_count: u8,
}

/* let user =
//     name:
//     profile:

*/
