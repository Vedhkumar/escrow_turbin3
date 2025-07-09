#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("Eh9LgkBpxCFQ4yKr15FUwuL2U4QJzVUxGYNPUzT1vj3n");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        escrow_id: u64,
        amount_a: u64,
        amount_b_wanted: u64,
    ) -> Result<()> {
        ctx.accounts
            .make(&ctx.bumps, escrow_id, amount_a, amount_b_wanted)?;
        ctx.accounts.deposit(amount_a)
    }
    pub fn take(ctx: Context<Take>, escrow_id: u64) -> Result<()> {
        ctx.accounts
            .make(&ctx.bumps, escrow_id, amount_a, amount_b_wanted)?;
        ctx.accounts.deposit(amount_a)
    }
}
