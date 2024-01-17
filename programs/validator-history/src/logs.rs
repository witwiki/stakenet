use anchor_lang::prelude::*;

#[event]
pub struct LogInitializedValidatorHistoryAccount {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub signer: Pubkey
}

