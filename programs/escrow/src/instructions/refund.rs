use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), escrow_id.to_le_bytes().as_ref()],
        bump,
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    fn refund(&self) -> Result<()> {
        let seeds = &[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.escrow_id.to_le_bytes()[..],
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    authority: self.escrow.to_account_info(),
                    from: self.vault.to_account_info(),
                    mint: self.mint_a.to_account_info(),
                    to: self.maker_token_a_account.to_account_info(),
                },
                signer_seeds,
            ),
            self.escrow.amount_a,
            self.mint_a.decimals,
        )
    }
}
