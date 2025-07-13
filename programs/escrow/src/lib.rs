#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("2HwNM48uAovyYCHMMvELDDdBBJA5DBEiBcim3PdLX4At");

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
            .make(&ctx.bumps, escrow_id, amount_a, amount_b_wanted)
    }

    // pub fn refund(ctx: Context<Refund>) -> Result<()> {
    //     Ok(())
    // }
    // pub fn take(ctx: Context<Take>, escrow_id: u64) -> Result<()> {
    //     ctx.accounts
    //         .taek(&ctx.bumps, escrow_id, amount_a, amount_b_wanted)?;
    //     ctx.accounts.deposit(amount_a)
    // }
}
