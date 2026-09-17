use anchor_lang::prelude::*;
use crate::errors::OrbitalError;

/// Constant-product quote:
///
/// k = x * y
/// token_out = y - k / (x + sol_in)
pub fn quote_buy(
    virtual_sol: u64,
    virtual_tokens: u64,
    sol_in: u64,
) -> Result<u64> {
    require!(sol_in > 0, OrbitalError::ZeroAmount);
    require!(
        virtual_sol > 0 && virtual_tokens > 0,
        OrbitalError::InvalidReserves
    );

    let x = virtual_sol as u128;
    let y = virtual_tokens as u128;
    let dx = sol_in as u128;

    let k = x
        .checked_mul(y)
        .ok_or(OrbitalError::MathOverflow)?;

    let new_x = x
        .checked_add(dx)
        .ok_or(OrbitalError::MathOverflow)?;

    let new_y = k / new_x;

    let out = y
        .checked_sub(new_y)
        .ok_or(OrbitalError::MathOverflow)?;

    u64::try_from(out).map_err(|_| OrbitalError::MathOverflow.into())
}

/// Reverse constant-product quote:
///
/// sol_out = x - k / (y + token_in)
pub fn quote_sell(
    virtual_sol: u64,
    virtual_tokens: u64,
    token_in: u64,
) -> Result<u64> {
    require!(token_in > 0, OrbitalError::ZeroAmount);
    require!(
        virtual_sol > 0 && virtual_tokens > 0,
        OrbitalError::InvalidReserves
    );

    let x = virtual_sol as u128;
    let y = virtual_tokens as u128;
    let dy = token_in as u128;

    let k = x
        .checked_mul(y)
        .ok_or(OrbitalError::MathOverflow)?;

    let new_y = y
        .checked_add(dy)
        .ok_or(OrbitalError::MathOverflow)?;

    let new_x = k / new_y;

    let out = x
        .checked_sub(new_x)
        .ok_or(OrbitalError::MathOverflow)?;

    u64::try_from(out).map_err(|_| OrbitalError::MathOverflow.into())
}
