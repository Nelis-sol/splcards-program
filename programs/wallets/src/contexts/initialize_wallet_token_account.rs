use crate::Wallet;
use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use std::mem;

#[derive(Accounts)]
pub struct InitializeWalletTokenAccount<'info> {
    #[account(mut, address=wallet.authority)]
    pub payer: Signer<'info>,
    #[account(address=Wallet::pda(payer.key()).0)]
    pub wallet: Account<'info, Wallet>,
    #[account(init,
      payer = payer,
      space = 8 + mem::size_of::<WalletTokenAccount>(),
      seeds = [SEED_WALLET_TOKEN_ACCOUNT.as_ref(), wallet.key().as_ref(), mint.key().as_ref()],
      bump)]
    pub wallet_token_account: Account<'info, WalletTokenAccount>,
    #[account(
      init,
      payer = payer,
      associated_token::mint = mint,
      associated_token::authority = wallet_token_account,
  )]
    pub wallet_token_account_bank: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeWalletTokenAccount<'_> {
    pub fn process(&mut self, spend_limit_amount: Option<u64>, bump: u8) -> Result<()> {
        let Self {
            wallet,
            wallet_token_account,
            mint,
            wallet_token_account_bank,
            ..
        } = self;
        wallet_token_account.new(
            wallet.key(),
            mint.key(),
            wallet_token_account_bank.key(),
            spend_limit_amount,
            bump,
        )?;
        Ok(())
    }
}
