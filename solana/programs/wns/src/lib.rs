use anchor_lang::prelude::*;
use crate::instruction::*;
use crate::state::*;
use crate::cross_chain::*;

pub mod instruction;
pub mod state;
pub mod error;
pub mod utils;
pub mod cross_chain;

declare_id!("YourProgramID");

#[program]
pub mod wns {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize(ctx, bump)
    }

    pub fn register_name(ctx: Context<RegisterName>, name: String, owner: Pubkey) -> Result<()> {
        instructions::register_name(ctx, name, owner)
    }

    pub fn renew_name(ctx: Context<RenewName>, name: String) -> Result<()> {
        instructions::renew_name(ctx, name)
    }

    pub fn resolve_name(ctx: Context<ResolveName>, name: String) -> Result<Pubkey> {
        instructions::resolve_name(ctx, name)
    }

    pub fn post_message(ctx: Context<PostMessage>, name: String) -> Result<()> {
        cross_chain::post_message(ctx, name)
    }
    
    pub fn register_wns_from_sns(ctx: Context<RegisterWnsFromSns>, domain: String) -> Result<()> {
        instructions::register_wns_from_sns(ctx, domain)
    }
}
