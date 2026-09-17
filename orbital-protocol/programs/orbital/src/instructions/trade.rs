use anchor_lang::prelude::*;

use crate::{
    errors::OrbitalError,
    math::{quote_buy, quote_sell},
    state::Launch,
};

#[derive(Accounts)]
pub struct Trade<'info> {
    pub user: Signer<'info>,

    #[account(mut)]
    pub launch: Account<'info, Launch>,
}

pub fn buy(
    ctx: Context<Trade>,
    sol_in: u64,
    min_tokens_out: u64,
) -> Result<()> {
    let launch = &mut ctx.accounts.launch;

    require!(launch.active, OrbitalError::LaunchInactive);
    require!(!launch.finalized, OrbitalError::LaunchFinalized);

    let tokens_out = quote_buy(
        launch.virtual_sol_reserve,
        launch.virtual_token_reserve,
        sol_in,
    )?;

    require!(
        tokens_out >= min_tokens_out,
        OrbitalError::SlippageExceeded
    );

    launch.virtual_sol_reserve = launch
        .virtual_sol_reserve
        .checked_add(sol_in)
        .ok_or(OrbitalError::MathOverflow)?;

    launch.virtual_token_reserve = launch
        .virtual_token_reserve
        .checked_sub(tokens_out)
        .ok_or(OrbitalError::MathOverflow)?;

    launch.total_tokens_sold = launch
        .total_tokens_sold
        .checked_add(tokens_out)
        .ok_or(OrbitalError::MathOverflow)?;

    launch.total_sol_volume = launch
        .total_sol_volume
        .checked_add(sol_in)
        .ok_or(OrbitalError::MathOverflow)?;

    emit!(BuyEvent {
        launch: launch.key(),
        user: ctx.accounts.user.key(),
        sol_in,
        tokens_out,
    });

    Ok(())
}

pub fn sell(
    ctx: Context<Trade>,
    tokens_in: u64,
    min_sol_out: u64,
) -> Result<()> {
    let launch = &mut ctx.accounts.launch;

    require!(launch.active, OrbitalError::LaunchInactive);
    require!(!launch.finalized, OrbitalError::LaunchFinalized);

    let sol_out = quote_sell(
        launch.virtual_sol_reserve,
        launch.virtual_token_reserve,
        tokens_in,
    )?;

    require!(
        sol_out >= min_sol_out,
        OrbitalError::SlippageExceeded
    );

    launch.virtual_token_reserve = launch
        .virtual_token_reserve
        .checked_add(tokens_in)
        .ok_or(OrbitalError::MathOverflow)?;

    launch.virtual_sol_reserve = launch
        .virtual_sol_reserve
        .checked_sub(sol_out)
        .ok_or(OrbitalError::MathOverflow)?;

    launch.total_tokens_sold = launch.total_tokens_sold.saturating_sub(tokens_in);

    launch.total_sol_volume = launch
        .total_sol_volume
        .checked_add(sol_out)
        .ok_or(OrbitalError::MathOverflow)?;

    emit!(SellEvent {
        launch: launch.key(),
        user: ctx.accounts.user.key(),
        tokens_in,
        sol_out,
    });

    Ok(())
}

#[event]
pub struct BuyEvent {
    pub launch: Pubkey,
    pub user: Pubkey,
    pub sol_in: u64,
    pub tokens_out: u64,
}

#[event]
pub struct SellEvent {
    pub launch: Pubkey,
    pub user: Pubkey,
    pub tokens_in: u64,
    pub sol_out: u64,
}
