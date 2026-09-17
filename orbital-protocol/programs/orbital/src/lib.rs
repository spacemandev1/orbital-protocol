use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("Orbita1111111111111111111111111111111111111");

#[program]
pub mod orbital {
    use super::*;

    pub fn initialize_platform(
        ctx: Context<InitializePlatform>,
        protocol_fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_platform(ctx, protocol_fee_bps)
    }

    pub fn create_launch(
        ctx: Context<CreateLaunch>,
        args: CreateLaunchArgs,
    ) -> Result<()> {
        instructions::create_launch(ctx, args)
    }

    pub fn buy(
        ctx: Context<Trade>,
        sol_in: u64,
        min_tokens_out: u64,
    ) -> Result<()> {
        instructions::buy(ctx, sol_in, min_tokens_out)
    }

    pub fn sell(
        ctx: Context<Trade>,
        tokens_in: u64,
        min_sol_out: u64,
    ) -> Result<()> {
        instructions::sell(ctx, tokens_in, min_sol_out)
    }

    pub fn finalize_launch(ctx: Context<FinalizeLaunch>) -> Result<()> {
        instructions::finalize_launch(ctx)
    }
}
