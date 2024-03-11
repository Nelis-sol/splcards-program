use crate::Wallet;
use crate::*;

#[derive(Accounts)]
pub struct UpdateWallet<'info> {
    #[account(mut, address=wallet.authority)]
    pub payer: Signer<'info>,
    #[account(mut, address=Wallet::pda(payer.key()).0)]
    pub wallet: Account<'info, Wallet>,
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}

impl<'info> UpdateWallet<'_> {
    pub fn process(
        &mut self,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()> {
        let Self {
            wallet,
            signer1,
            signer2,
            ..
        } = self;
        wallet.update(
            signer1.key(),
            signer2.key(),
            required_signer1,
            required_signer2,
            allow_list,
            block_list,
            spending_window,
        );
        Ok(())
    }
}
