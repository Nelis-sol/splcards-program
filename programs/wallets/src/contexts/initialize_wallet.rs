use crate::*;
use crate::{Wallet, SEED_WALLET};
use std::mem;

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init,
      payer = payer,
      space = 8 + mem::size_of::<Wallet>(),
      seeds = [SEED_WALLET.as_ref(), payer.key().as_ref()],
      bump)]
    pub wallet: Account<'info, Wallet>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWallet<'_> {
    pub fn process(
        &mut self,
        signer1: Pubkey,
        signer2: Pubkey,
        bump: u8,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()> {
        let Self { payer, wallet, .. } = self;
        wallet.new(
            payer.key(),
            signer1,
            signer2,
            bump,
            required_signer1,
            required_signer2,
            allow_list,
            block_list,
            spending_window,
        )?;
        Ok(())
    }
}
