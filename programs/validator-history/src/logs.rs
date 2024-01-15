use anchor_lang::prelude::*;

// pub struct ValidatorHistory {
//     // Cannot be enum due to Pod and Zeroable trait limitations
//     pub struct_version: u32,

//     pub vote_account: Pubkey,
//     // Index of validator of all ValidatorHistory accounts
//     pub index: u32,

//     pub bump: u8,

//     pub _padding0: [u8; 7],

//     // These Crds gossip values are only signed and dated once upon startup and then never updated
//     // so we track latest time on-chain to make sure old messages aren't uploaded
//     pub last_ip_timestamp: u64,
//     pub last_version_timestamp: u64,

//     pub _padding1: [u8; 232],

//     pub history: CircBuf,
// }

// Any change in the Valido

#[event]
pub struct DepositLog {
    pub open_orders_account: Pubkey,
    pub signer: Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}

