use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

#[derive(Accounts)]
pub struct PostMessage<'info> {
    #[account(mut)]
    pub core_bridge: AccountInfo<'info>,
    pub wormhole_config: AccountInfo<'info>,
    pub wormhole_program: AccountInfo<'info>,
}

pub fn post_message(ctx: Context<PostMessage>, name: String) -> Result<()> {
    // Post message with the name information using Wormhole SDK
    let msg = name.as_bytes().to_vec();
    wormhole::post_message(
        &ctx.accounts.wormhole_program,
        &ctx.accounts.core_bridge,
        &ctx.accounts.wormhole_config,
        &ctx.accounts.sender.to_account_info(),
        &msg,
        0, // nonce for the message
    )?;
    Ok(())
}
