use anchor_lang::prelude::*;

use crate::{
    constants::{MAX_FEE_BPS, MAX_NAME_LEN, MAX_SYMBOL_LEN, MAX_URI_LEN},
    errors::OrbitalError,
    state::{Launch, PlatformConfig},
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateLaunchArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub virtual_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub creator_fee_bps: u16,
}

#[derive(Accounts)]
pub struct CreateLaunch<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: In the starter, mint is only used as a unique launch identifier.
    /// Production code should validate a real SPL/Token-2022 mint.
    pub mint: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, PlatformConfig>,

    #[account(
        init,
        payer = creator,
        space = 8 + Launch::INIT_SPACE,
        seeds = [b"launch", creator.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub launch: Account<'info, Launch>,

    pub system_program: Program<'info, System>,
}

pub fn create_launch(
    ctx: Context<CreateLaunch>,
    args: CreateLaunchArgs,
) -> Result<()> {
    require!(
        args.name.len() <= MAX_NAME_LEN
            && args.symbol.len() <= MAX_SYMBOL_LEN
            && args.uri.len() <= MAX_URI_LEN,
        OrbitalError::StringTooLong
    );

    require!(
        args.creator_fee_bps <= MAX_FEE_BPS,
        OrbitalError::FeeTooHigh
    );

    require!(
        args.virtual_sol_reserve > 0 && args.virtual_token_reserve > 0,
        OrbitalError::InvalidReserves
    );

    let launch = &mut ctx.accounts.launch;

    launch.creator = ctx.accounts.creator.key();
    launch.mint = ctx.accounts.mint.key();
    launch.name = args.name;
    launch.symbol = args.symbol;
    launch.uri = args.uri;
    launch.virtual_sol_reserve = args.virtual_sol_reserve;
    launch.virtual_token_reserve = args.virtual_token_reserve;
    launch.total_tokens_sold = 0;
    launch.total_sol_volume = 0;
    launch.creator_fee_bps = args.creator_fee_bps;
    launch.active = true;
    launch.finalized = false;
    launch.bump = ctx.bumps.launch;

    ctx.accounts.platform.launch_count = ctx
        .accounts
        .platform
        .launch_count
        .checked_add(1)
        .ok_or(OrbitalError::MathOverflow)?;

    Ok(())
}
