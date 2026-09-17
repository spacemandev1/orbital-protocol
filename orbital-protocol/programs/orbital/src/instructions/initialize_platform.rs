use anchor_lang::prelude::*;

use crate::{
    constants::MAX_FEE_BPS,
    errors::OrbitalError,
    state::PlatformConfig,
};

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + PlatformConfig::INIT_SPACE,
        seeds = [b"platform"],
        bump
    )]
    pub platform: Account<'info, PlatformConfig>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_platform(
    ctx: Context<InitializePlatform>,
    protocol_fee_bps: u16,
) -> Result<()> {
    require!(
        protocol_fee_bps <= MAX_FEE_BPS,
        OrbitalError::FeeTooHigh
    );

    let platform = &mut ctx.accounts.platform;
    platform.authority = ctx.accounts.authority.key();
    platform.protocol_fee_bps = protocol_fee_bps;
    platform.launch_count = 0;
    platform.bump = ctx.bumps.platform;

    Ok(())
}
