use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub escrow_id: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub amount_b_wanted: u64,
    pub bump: u8,
}
