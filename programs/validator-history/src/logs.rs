use anchor_lang::prelude::*;

#[event]
pub struct LogInitializeValidatorHistoryAccount {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub signer: Pubkey
}

#[event]
pub struct LogReallocValidatorHistoryAccount {
    pub validator_history_account: Pubkey,
    pub config: Pubkey,
    pub bump: u8,
    pub struct_version: u32,
    // pub history_idx: u64,
    pub vote_account: Pubkey,
    pub signer: Pubkey
}

#[event]
pub struct LogInitializeClusterHistoryAccount {
    pub cluster_history_account: Pubkey,
    pub signer: Pubkey
}

#[event]
pub struct LogCopyVoteAccount {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub signer: Pubkey,
    pub epoch: u16,
    pub commission: u8,
    pub slot: u64,
    pub epoch_credits: Vec<(u64, u64, u64)>
}