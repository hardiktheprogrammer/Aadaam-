use anchor_lang::prelude::*;

pub mod constant;
pub mod state;
use crate::{constant::*, state::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod adamm {

    use super::*;

    pub fn init_user(ctx: Context<InitUser>,name:String,profile::String) --> Result<()>  {   // init user wallet address 

        
    } 
}
