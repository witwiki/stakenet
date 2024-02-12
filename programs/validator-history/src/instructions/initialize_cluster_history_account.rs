use crate::{constants::MAX_ALLOC_BYTES, ClusterHistory};
use crate::logs::LogInitializeClusterHistoryAccount;
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
pub fn handler(ctx: Context<InitializeClusterHistoryAccount>) -> Result<()> {

    emit_cpi!(LogInitializeClusterHistoryAccount {
        cluster_history_account: ctx.accounts.cluster_history_account.key(),
        signer: ctx.accounts.signer.owner.key()
    });

=======
pub fn handle_initialize_cluster_history_account(
    _: Context<InitializeClusterHistoryAccount>,
) -> Result<()> {
>>>>>>> upstream/master
    Ok(())
}
