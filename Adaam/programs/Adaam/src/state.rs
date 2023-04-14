use anchor::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub name: String,      // 4 +256
    pub profile: String,   // 4 + 2864
    pub authority: PubKey, //32
    pub last_post_id: u8,  //1
    pub post_count: u8,
    // pub authority: u8;
}
#[account]
#[derive(Default)]
pub struct PostAccount {
    pub id: u8,
    pub title: String,     //
    pub user: PubKey,      //
    pub authority: PubKey, //
    pub authoriy: Pubkey,  //
}
