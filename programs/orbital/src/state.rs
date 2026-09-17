use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub launch_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Launch {
    pub creator: Pubkey,
    pub mint: Pubkey,

    #[max_len(32)]
    pub name: String,

    #[max_len(10)]
    pub symbol: String,

    #[max_len(200)]
    pub uri: String,

    pub virtual_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub total_tokens_sold: u64,
    pub total_sol_volume: u64,

    pub creator_fee_bps: u16,
    pub active: bool,
    pub finalized: bool,

    pub bump: u8,
}
