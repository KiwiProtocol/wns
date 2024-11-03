use solana_name_service::resolution::*;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use anchor_lang::prelude::*;
use solana_client::rpc_client::RpcClient;

#[derive(Accounts)]
pub struct RegisterWnsFromSns<'info> {
    #[account(mut)]
    pub registry: Account<'info, NameRegistry>,
    pub sns_program: AccountInfo<'info>,
}

pub fn resolve_sns_owner(domain: &str) -> Result<Pubkey> {
    // Replace "RPC_URL" with the RPC endpoint to Solana Devnet or Mainnet
    let client = RpcClient::new(std::env::var("RPC_URL").unwrap());
    let owner = resolve_owner(&client, domain)
        .map_err(|_| ErrorCode::SnsDomainNotFound)?;

    Ok(owner)
}

pub fn register_wns_from_sns(ctx: Context<RegisterWnsFromSns>, domain: String) -> Result<()> {
    // Resolve the SNS owner for the given `.sol` domain
    let sns_owner = resolve_sns_owner(&domain)?;

    // Check if the owner already has a `.wns` domain
    let registry = &mut ctx.accounts.registry;
    for record in registry.records.iter() {
        if record.name == format!("{}.wns", domain) {
            return Err(ErrorCode::DomainAlreadyRegistered.into());
        }
    }

    // Register a new `.wns` domain for the SNS owner
    let new_record = NameRecord {
        owner: sns_owner,
        name: format!("{}.wns", domain),
        expiration: Clock::get()?.unix_timestamp + 365 * 24 * 60 * 60, // 1 year
    };
    registry.records.push(new_record);

    Ok(())
}
