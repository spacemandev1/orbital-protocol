use anchor_lang::prelude::*;

#[error_code]
pub enum OrbitalError {
    #[msg("Fee is above the allowed maximum.")]
    FeeTooHigh,

    #[msg("Launch is not active.")]
    LaunchInactive,

    #[msg("Launch is already finalized.")]
    LaunchFinalized,

    #[msg("Invalid reserve configuration.")]
    InvalidReserves,

    #[msg("Slippage limit exceeded.")]
    SlippageExceeded,

    #[msg("Arithmetic overflow.")]
    MathOverflow,

    #[msg("Input amount must be greater than zero.")]
    ZeroAmount,

    #[msg("String field exceeds maximum length.")]
    StringTooLong,

    #[msg("Only the launch creator may perform this action.")]
    UnauthorizedCreator,
}
