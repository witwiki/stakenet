use anchor_lang::{
    prelude::*
    // , solana_program::{slot_history::Check}
};

use crate::{
    crds_value::{ContactInfo, LegacyContactInfo}
};
// use borsh::BorshSerialize;

#[event]
pub struct LogInitializeValidatorHistoryAccount {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub signer: Pubkey
}

<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> 5c2f60e (mostly finished with 'LogReallocValidatorHistoryAccount' but can't seem to get the 'history_idx' logged yet.)
#[event]
pub struct LogReallocValidatorHistoryAccount {
    pub validator_history_account: Pubkey,
    pub config: Pubkey,
<<<<<<< HEAD
    pub vote_account: Pubkey,
    pub system_program: Pubkey,
    pub signer: Pubkey,
    pub bump: u8,
    pub struct_version: u32,
    pub history_idx: u64
}

#[event]
pub struct LogReallocClusterHistoryAccount {
    pub cluster_history_account: Pubkey,
    pub system_program: Pubkey,
    pub signer: Pubkey,
    pub bump: u8,
    pub struct_version: u64,
    pub history_idx: u64
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


#[event]
pub struct LogUpdateMevCommission {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub config: Pubkey,
    pub tip_distribution_account: Pubkey,
    pub signer: Pubkey,
    pub mev_commission_bps: u16,
    pub epoch: u16
}

#[event]
pub struct LogInitializeConfig {
    pub config: Pubkey,
    pub signer: Pubkey,
    pub oracle_authority: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
    pub counter: u32
}

#[event]
pub struct LogSetNewTipDistributionProgram {
    pub config: Pubkey,
    pub new_tip_distribution_program: Pubkey,
    pub admin: Pubkey
}

#[event]
pub struct LogSetNewAdmin {
    pub config: Pubkey,
    pub new_admin: Pubkey,
    pub admin: Pubkey
}

#[event]
pub struct LogSetNewOracleAuthority {
    pub config: Pubkey,
    pub new_oracle_authority: Pubkey,
    pub admin: Pubkey
}

#[event]
pub struct LogBackfillTotalBlocks {
    pub cluster_history_account: Pubkey,
    pub config: Pubkey,
    pub epoch: u16,
    pub blocks_in_epoch: u32,
    pub signer: Pubkey
}

#[event]
pub struct LogCopyClusterInfo {
    pub cluster_history_account: Pubkey,
    // slot_history: Box<SlotHistory>,
    pub signer: Pubkey,
    //// Unsure to log params below due to its conditional nature
    // epoch: epoch,
    // blocks_in_epoch: 
    pub cluster_history_last_update_slot: u64
}

#[event]
pub struct LogUpdateStakeHistory {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub config: Pubkey,
    pub oracle_authority: Pubkey,
    pub epoch: u16,
    pub lamports: u64,
    pub rank: u32,
    pub is_superminority: bool
}

#[event]
pub struct LogCopyGossipContactInfo {
    pub validator_history_account: Pubkey,
    pub vote_account: Pubkey,
    pub instructions: Pubkey,
    pub signer: Pubkey,
    pub epoch: u16,
    // pub legacy_contact_info: LegacyContactInfo::default(),
    // pub last_signed_ts: ,
    // pub contact_info: &ContactInfo,
    // pub version: ,
    // pub legacy_version: CircBuf::default();
}

>>>>>>> master
=======
    pub bump: u8,
    pub struct_version: u32,
    // pub history_idx: u64,
    pub vote_account: Pubkey,
    pub signer: Pubkey
}
<<<<<<< HEAD
>>>>>>> 5c2f60e (mostly finished with 'LogReallocValidatorHistoryAccount' but can't seem to get the 'history_idx' logged yet.)
=======

#[event]
pub struct LogInitializeClusterHistoryAccount {
    pub cluster_history_account: Pubkey,
    pub signer: Pubkey
}
>>>>>>> 731f799 (completed logging the InitializeClusterHistoryAccount with LogInitializedClusterHistoryAccount)
