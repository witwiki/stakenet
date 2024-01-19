use crate::{constants::MAX_ALLOC_BYTES, ClusterHistory};
<<<<<<< HEAD
<<<<<<< HEAD
use crate::logs::LogInitializeClusterHistoryAccount;
=======
use crate::logs::LogInitializedClusterHistoryAccount;
>>>>>>> 731f799 (completed logging the InitializeClusterHistoryAccount with LogInitializedClusterHistoryAccount)
=======
use crate::logs::LogInitializeClusterHistoryAccount;
>>>>>>> 16c23d6 (renamed Log events from InitializedValidatorHistoryAccount and InitializedClusterHistoryAccount to InitializeValidatorHistoryAccount and InitializeClusterHistoryAccount to keep consistency in function names with original code)
use anchor_lang::prelude::*;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeClusterHistoryAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = MAX_ALLOC_BYTES,
        seeds = [ClusterHistory::SEED],
        bump
    )]
    pub cluster_history_account: AccountLoader<'info, ClusterHistory>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

<<<<<<< HEAD
<<<<<<< HEAD
pub fn handler(ctx: Context<InitializeClusterHistoryAccount>) -> Result<()> {

    emit_cpi!(LogInitializeClusterHistoryAccount {
=======
pub fn handler(ctx: Context<InitializeClusterHistoryAccount>) -> Result<()> {

<<<<<<< HEAD
    emit_cpi!(LogInitializedClusterHistoryAccount {
>>>>>>> 731f799 (completed logging the InitializeClusterHistoryAccount with LogInitializedClusterHistoryAccount)
=======
    emit_cpi!(LogInitializeClusterHistoryAccount {
>>>>>>> 16c23d6 (renamed Log events from InitializedValidatorHistoryAccount and InitializedClusterHistoryAccount to InitializeValidatorHistoryAccount and InitializeClusterHistoryAccount to keep consistency in function names with original code)
        cluster_history_account: ctx.accounts.cluster_history_account.key(),
        signer: ctx.accounts.signer.owner.key()
    });

<<<<<<< HEAD
=======
pub fn handle_initialize_cluster_history_account(
    _: Context<InitializeClusterHistoryAccount>,
) -> Result<()> {
>>>>>>> upstream/master
=======
>>>>>>> 731f799 (completed logging the InitializeClusterHistoryAccount with LogInitializedClusterHistoryAccount)
    Ok(())
}
