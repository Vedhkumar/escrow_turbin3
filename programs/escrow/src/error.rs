use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The two mints cannot be the same.")]
    SameMint,
    #[msg("The amount must be greater than zero.")]
    InvalidAmount,
}
