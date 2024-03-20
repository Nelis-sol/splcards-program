use crate::*;

use self::error::ErrorCode;

pub const SEED_WALLET: &[u8] = b"wallet";

#[account]
pub struct Wallet {
    pub authority: Pubkey,
    pub signer1: Pubkey,
    pub required_signer1: bool,
    pub signer2: Pubkey,
    pub required_signer2: bool,
    pub allow_list: Option<Vec<Pubkey>>, // keep into account limitations of size
    pub block_list: Option<Vec<Pubkey>>, // point to PDA for limited use of pubkeys to keep size into account
    // use a enum (called list with items allow list and blocklist) instead of 2 fields (allow + block list) to XOR allow_list XOR block_list
    // trade-off: lower compute -> research / test it

    // consider using a look-up table only for block_list (allow_list is only for a few pubkeys).
    pub spending_window: Option<(i64, i64)>,
    // leave it with i64 for now, but if you want to optimize for compute/space use u32
    pub bump: u8,
}

impl Wallet {
    pub fn pda(authority: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_WALLET, authority.as_ref()], &crate::ID)
    }

    pub fn spend(
        &mut self,
        amount: u64,
        signer1: bool,
        signer2: bool,
        destination: Pubkey,
        current_timestamp: i64,
        wallet_token_account: &mut WalletTokenAccount,
    ) -> Result<()> {
        // requires both signers if requires_signers is true
        if (self.required_signer1 && !signer1) || (self.required_signer2 && !signer2) {
            return Err(ErrorCode::MissingRequiredSigners.into());
        }

        // block list
        if let Some(block_list) = &self.block_list {
            if block_list.contains(&destination) {
                return Err(ErrorCode::PubkeyInBlockList.into());
            }
        }

        // allow list
        if let Some(allow_list) = &self.allow_list {
            if !allow_list.contains(&destination) {
                return Err(ErrorCode::PubkeyNotInAllowList.into());
            }
        }

        // spending window
        if let Some((start, end)) = self.spending_window {
            if current_timestamp % 86400 < start % 86400 || current_timestamp % 86400 > end % 86400
            {
                return Err(ErrorCode::SpendingWindowViolation.into());
            }
        }

        wallet_token_account.spend(amount, signer1, signer2, current_timestamp)?;
        Ok(())
    }

    pub fn update(
        &mut self,
        signer1: Pubkey,
        signer2: Pubkey,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) {
        self.signer1 = signer1;
        self.signer2 = signer2;
        self.required_signer1 = required_signer1;
        self.required_signer2 = required_signer2;
        self.allow_list = allow_list;
        self.block_list = block_list;
        self.spending_window = spending_window;
    }
}

impl TryFrom<Vec<u8>> for Wallet {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Self::try_deserialize(&mut data.as_slice())
    }
}

pub trait WalletAccount {
    fn new(
        &mut self,
        authority: Pubkey,
        signer1: Pubkey,
        signer2: Pubkey,
        bump: u8,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()>;
}

impl WalletAccount for Account<'_, Wallet> {
    fn new(
        &mut self,
        authority: Pubkey,
        signer1: Pubkey,
        signer2: Pubkey,
        bump: u8,
        required_signer1: bool,
        required_signer2: bool,
        allow_list: Option<Vec<Pubkey>>,
        block_list: Option<Vec<Pubkey>>,
        spending_window: Option<(i64, i64)>,
    ) -> Result<()> {
        self.authority = authority;
        self.signer1 = signer1;
        self.signer2 = signer2;
        self.required_signer1 = required_signer1;
        self.required_signer2 = required_signer2;
        self.allow_list = allow_list;
        self.block_list = block_list;
        self.spending_window = spending_window;
        self.bump = bump;
        Ok(())
    }
}
