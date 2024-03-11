use crate::Wallet;
use crate::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

// Spending is basically transfering

#[derive(Accounts)]
pub struct Spend<'info> {
    #[account(mut, address=wallet.authority)]
    pub payer: Signer<'info>,
    #[account(address=Wallet::pda(payer.key()).0)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut,address=WalletTokenAccount::pda(wallet.key(),wallet_token_account.mint.key()).0)]
    pub wallet_token_account: Account<'info, WalletTokenAccount>,
    #[account(
      mut,
      associated_token::mint = wallet_token_account.mint,
      associated_token::authority = wallet_token_account,
    )]
    pub wallet_token_account_bank: Account<'info, TokenAccount>,
    #[account(
      mut,
      token::mint = wallet_token_account.mint,
    )]
    pub spend_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we need to check if this account is passed as signer or not
    #[account(address=wallet.signer1)]
    pub signer1: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we need to check if this account is passed as signer or not
    #[account(address=wallet.signer2)]
    pub signer2: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Spend<'_> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let Self {
            wallet_token_account,
            token_program,
            spend_token_account,
            wallet_token_account_bank,
            signer1,
            signer2,
            wallet,
            ..
        } = self;
        wallet.spend(
            amount,
            signer1.is_signer,
            signer2.is_signer,
            spend_token_account.key(),
            Clock::get()?.unix_timestamp,
            wallet_token_account,
        )?;
        token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    authority: wallet_token_account.to_account_info(),
                    from: wallet_token_account_bank.to_account_info(),
                    to: spend_token_account.to_account_info(),
                },
                &[&[
                    SEED_WALLET_TOKEN_ACCOUNT,
                    wallet_token_account.wallet.key().as_ref(),
                    wallet_token_account.mint.key().as_ref(),
                    &[wallet_token_account.bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }
}
