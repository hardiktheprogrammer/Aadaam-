use anchor_lang::prelude::*;

pub mod constant;
pub mod state;
use crate::{constant::*, state::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod adamm {

    use super::*;

    pub fn init_user(ctx: Context<InitUser>,name:String,profile::String) --> Result<()>  {   // init user wallet address

        // let user_account = &mut
        // let user = Lance
        // user = David 

        let user_account = &mut

        let user = len

         
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
        payer = authority,//
        space = 2312 + 8
    )]

    pub  users_account: Account<'info,Users Account>,



    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

}
