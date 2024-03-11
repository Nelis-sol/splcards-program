use crate::Wallet;
use crate::*;

#[derive(Accounts)]
pub struct UpdateWalletTokenAccount<'info> {
    #[account(mut, address=wallet.authority)]
    pub payer: Signer<'info>,
    #[account(address=Wallet::pda(payer.key()).0)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut,address=WalletTokenAccount::pda(wallet.key(),wallet_token_account.mint.key()).0)]
    pub wallet_token_account: Account<'info, WalletTokenAccount>,
}

impl<'info> UpdateWalletTokenAccount<'_> {
    pub fn process(&mut self, spend_limit_amount: Option<u64>) -> Result<()> {
        let Self {
            wallet_token_account,
            ..
        } = self;
        wallet_token_account.update(spend_limit_amount);
        Ok(())
    }
}
