use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use anchor_lang::solana_program::{system_instruction, program::invoke};
use crate::sns_integration::*;

const REGISTRATION_FEE: u64 = 0.2 * LAMPORTS_PER_SOL;
const RENEWAL_FEE: u64 = 0.2 * LAMPORTS_PER_SOL;

pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    registry.admin = ctx.accounts.admin.key();
    registry.bump = bump;
    Ok(())
}

pub fn register_name(ctx: Context<RegisterName>, name: String, owner: Pubkey) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    let record = NameRecord {
        owner,
        name: name.clone(),
        expiration: Clock::get()?.unix_timestamp + 365 * 24 * 60 * 60, // 1 year
    };
    registry.records.push(record);

    // Collect registration fee
    let ix = system_instruction::transfer(&ctx.accounts.payer.key(), &ctx.accounts.treasury.key(), REGISTRATION_FEE);
    invoke(&ix, &[ctx.accounts.payer.to_account_info(), ctx.accounts.treasury.to_account_info()])?;

    Ok(())
}

pub fn renew_name(ctx: Context<RenewName>, name: String) -> Result<()> {
    let registry = &mut ctx.accounts.registry;
    for record in registry.records.iter_mut() {
        if record.name == name {
            record.expiration += 365 * 24 * 60 * 60; // Extend by 1 year

            // Collect renewal fee
            let ix = system_instruction::transfer(&ctx.accounts.payer.key(), &ctx.accounts.treasury.key(), RENEWAL_FEE);
            invoke(&ix, &[ctx.accounts.payer.to_account_info(), ctx.accounts.treasury.to_account_info()])?;

            return Ok(());
        }
    }
    Err(ErrorCode::NameNotFound.into())
}

pub fn register_wns_from_sns(ctx: Context<RegisterWnsFromSns>, domain: String) -> Result<()> {
    sns_integration::register_wns_from_sns(ctx, domain)
}

