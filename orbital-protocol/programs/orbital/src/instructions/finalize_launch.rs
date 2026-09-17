use anchor_lang::prelude::*;

use crate::{
    errors::OrbitalError,
    state::Launch,
};

#[derive(Accounts)]
pub struct FinalizeLaunch<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        constraint = launch.creator == creator.key() @ OrbitalError::UnauthorizedCreator
    )]
    pub launch: Account<'info, Launch>,
}

pub fn finalize_launch(
    ctx: Context<FinalizeLaunch>,
) -> Result<()> {
    let launch = &mut ctx.accounts.launch;

    require!(!launch.finalized, OrbitalError::LaunchFinalized);

    launch.active = false;
    launch.finalized = true;

    emit!(LaunchFinalizedEvent {
        launch: launch.key(),
        creator: ctx.accounts.creator.key(),
    });

    Ok(())
}

#[event]
pub struct LaunchFinalizedEvent {
    pub launch: Pubkey,
    pub creator: Pubkey,
}
