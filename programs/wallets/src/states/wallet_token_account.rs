use crate::*;

use self::error::ErrorCode;

pub const SEED_WALLET_TOKEN_ACCOUNT: &[u8] = b"wallet_token_account";

#[account]
pub struct WalletTokenAccount {
    pub wallet: Pubkey,
    pub bank: Pubkey,
    pub mint: Pubkey,
    pub spent_last_24: (i64, u64), // (timestamp, amount)
    pub spend_limit_amount: Option<u64>,
    pub bump: u8,
}

impl WalletTokenAccount {
    pub fn pda(wallet: Pubkey, mint: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_WALLET_TOKEN_ACCOUNT, wallet.as_ref(), mint.as_ref()],
            &crate::ID,
        )
    }

    pub fn spend(
        &mut self,
        amount: u64,
        signer1: bool,
        signer2: bool,
        current_timestamp: i64,
    ) -> Result<()> {
        // spend limit
        if let Some(spend_limit_amount) = self.spend_limit_amount {
            // get start of today timestamp
            let today_timestamp = current_timestamp - (current_timestamp % 86400);

            // check if the amount spent in the last 24 hours is greater than the spend limit
            if self.spent_last_24.0 == today_timestamp {
                if self.spent_last_24.1 + amount > spend_limit_amount {
                    if !signer1 && !signer2 {
                        return Err(ErrorCode::SpendLimitExceeded.into());
                    }
                }
                self.spent_last_24.1 += amount;
            } else {
                if amount > spend_limit_amount {
                    if !signer1 && !signer2 {
                        return Err(ErrorCode::SpendLimitExceeded.into());
                    }
                }
                self.spent_last_24.0 = today_timestamp;
                self.spent_last_24.1 = amount;
            }
        }

        Ok(())
    }

    pub fn update(&mut self, spend_limit_amount: Option<u64>) {
        self.spend_limit_amount = spend_limit_amount;
    }
}

impl TryFrom<Vec<u8>> for WalletTokenAccount {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Self::try_deserialize(&mut data.as_slice())
    }
}

pub trait WalletTokenAccountAccount {
    fn new(
        &mut self,
        wallet: Pubkey,
        mint: Pubkey,
        bank: Pubkey,
        spend_limit_amount: Option<u64>,
        bump: u8,
    ) -> Result<()>;
}

impl WalletTokenAccountAccount for Account<'_, WalletTokenAccount> {
    fn new(
        &mut self,
        wallet: Pubkey,
        mint: Pubkey,
        bank: Pubkey,
        spend_limit_amount: Option<u64>,
        bump: u8,
    ) -> Result<()> {
        self.wallet = wallet;
        self.mint = mint;
        self.spend_limit_amount = spend_limit_amount;
        self.bump = bump;
        self.bank = bank;
        self.spent_last_24 = (0, 0);
        Ok(())
    }
}
