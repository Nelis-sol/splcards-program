use anchor_lang::prelude::*;

pub mod contexts;
pub mod error;
pub mod states;

pub use contexts::*;
pub use states::*;

declare_id!("AVMA24bYvmoq1CiaGM8v8wdhzj2K5u5vFzstS9pHztLv");

#[program]
pub mod wallets {
    use super::*;

    pub fn initialize_wallet(
        ctx: Context<InitializeWallet>,
        signer1: Pubkey,
        signer2: Pubkey,
        bump: u8,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()> {
        ctx.accounts.process(
            signer1,
            signer2,
            bump,
            required_signer1,
            required_signer2,
            allow_list,
            block_list,
            spending_window,
        )
    }

    pub fn update_wallet(
        ctx: Context<UpdateWallet>,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()> {
        ctx.accounts.process(
            required_signer1,
            required_signer2,
            allow_list,
            block_list,
            spending_window,
        )
    }

    pub fn initialize_wallet_token_account(
        ctx: Context<InitializeWalletTokenAccount>,
        spend_limit_amount: Option<u64>,
        bump: u8,
    ) -> Result<()> {
        ctx.accounts.process(spend_limit_amount, bump)
    }

    pub fn update_wallet_token_account(
        ctx: Context<UpdateWalletTokenAccount>,
        spend_limit_amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.process(spend_limit_amount)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn spend(ctx: Context<Spend>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }
}
