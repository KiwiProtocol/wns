use anchor_lang::prelude::*;

#[account]
pub struct NameRegistry {
    pub admin: Pubkey,
    pub records: Vec<NameRecord>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NameRecord {
    pub owner: Pubkey,
    pub name: String,
    pub expiration: i64, // Unix timestamp for expiration
}
