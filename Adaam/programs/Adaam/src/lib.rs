use anchor_lang::prelude::*;

pub mod constant;
pub mod state;
use crate::{constant::*, state::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod adamm {

    use anchor_lang::system_program::CreateAccount;

    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String, profile: String) -> Result<()> {
        // init user wallet address

        let user_account = &mut ctx.account.user_account;
        let authority = &mut ctx.account.authority;

        // logic for getting users
        user_account.name = name;
        user_account.profile = profile;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        user_account.authority = authoriy.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let post_account = ctx.accounts.post_account;
        let user_account = ctx.accounts.user_account;
        let authority = ctx.accounts.authority;
        
        

        
    }
}

// accounts
#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    #[account(

        init,
        seeds = [USER_SEED, authority.key().as_ref()], // program address
        bump,
        payer = authority,    //
        space = 2312 + 8
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]

pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [POST_SEED, authority.key().as_ref(),&[user_account.last_post_id as u8 ].as_ref()],
        bump,   
        payer = authority,
        space = 2376 + 8 
        
        
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(

        mut,
        seeds = [USER_SEED, authorized.key().as.ref()],
        bump,
        has_one = authority 


    )]

    pub user_account: Account<'info,UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}
