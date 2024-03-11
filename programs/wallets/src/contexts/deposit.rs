use crate::Wallet;
use crate::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, address=wallet.authority)]
    pub payer: Signer<'info>,
    #[account(address=Wallet::pda(payer.key()).0)]
    pub wallet: Account<'info, Wallet>,
    #[account(address=WalletTokenAccount::pda(wallet.key(),wallet_token_account.mint.key()).0)]
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
      token::authority = payer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Deposit<'_> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let Self {
            token_program,
            payer,
            user_token_account,
            wallet_token_account_bank,
            ..
        } = self;
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    authority: payer.to_account_info(),
                    from: user_token_account.to_account_info(),
                    to: wallet_token_account_bank.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}
