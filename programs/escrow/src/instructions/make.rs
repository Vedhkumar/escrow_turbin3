use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    //? Since we have two token standars so we are passing the token program so that we know what token program we are using to get mint
    #[account(
        mint::token_program = token_program,
    )]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_b_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    //? So a ATA can be created for a PDA aswell
    #[account(
        init,
        payer = maker,
        associated_token::mint = token_a_mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        //? So the user should able to create multiple escrows so we gonna use some id while driving the seeds
        seeds = [b"escrow", maker.key().as_ref(), escrow_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    //? This program because it creates the associated token account
    pub associated_token_program: Program<'info, AssociatedToken>,
    //? New token program
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn make(
        &mut self,
        bumps: &MakeBumps,
        escrow_id: u64,
        amount_a: u64,
        amount_b_wanted: u64,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            escrow_id,
            maker: self.maker.key(),
            mint_a: self.token_a_mint.key(),
            mint_b: self.token_b_mint.key(),
            amount_a,
            amount_b_wanted,
            bump: bumps.escrow,
        });
        Ok(())
    }

    pub fn deposit(&self, amount_a: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.maker_token_a_account.to_account_info(),
                    mint: self.token_a_mint.to_account_info(),
                    to: self.token_a_vault.to_account_info(),
                    authority: self.maker.to_account_info(),
                },
            ),
            amount_a,
            self.token_a_mint.decimals,
        )
    }
}
