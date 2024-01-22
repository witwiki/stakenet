use anchor_lang::prelude::*;

use crate::{errors::ValidatorHistoryError, utils::cast_epoch, ClusterHistory, Config};
use crate::logs::LogBackfillTotalBlocks;

#[event_cpi]
#[derive(Accounts)]
pub struct BackfillTotalBlocks<'info> {
    #[account(
        mut,
        seeds = [ClusterHistory::SEED],
        bump,
    )]
    pub cluster_history_account: AccountLoader<'info, ClusterHistory>,
    pub config: Account<'info, Config>,
    #[account(mut, address = config.oracle_authority )]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<BackfillTotalBlocks>, epoch: u64, blocks_in_epoch: u32) -> Result<()> {
    let mut cluster_history_account = ctx.accounts.cluster_history_account.load_mut()?;

    let epoch = cast_epoch(epoch);

    // Need to backfill in ascending order when initially filling in the account otherwise entries will be out of order
    if !cluster_history_account.history.is_empty()
        && epoch != cluster_history_account.history.last().unwrap().epoch + 1
    {
        return Err(ValidatorHistoryError::EpochOutOfRange.into());
    }
    cluster_history_account.set_blocks(epoch, blocks_in_epoch)?;

    emit_cpi!(LogBackfillTotalBlocks {
        cluster_history_account: ctx.accounts.cluster_history_account.key(),
        config: ctx.accounts.config.key(),
        epoch: epoch,
        blocks_in_epoch: blocks_in_epoch,
        signer: ctx.accounts.signer.owner.key()
    });

    Ok(())
}
