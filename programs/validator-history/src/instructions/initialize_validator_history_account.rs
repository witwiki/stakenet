use crate::{
    constants::{MAX_ALLOC_BYTES, MIN_VOTE_EPOCHS},
    errors::ValidatorHistoryError,
    state::ValidatorHistory,
<<<<<<< HEAD
<<<<<<< HEAD
    logs::LogInitializedValidatorHistoryAccount,
=======
    logs::LogInitializeValidatorHistoryAccount,
>>>>>>> master
=======
    logs::LogInitializeValidatorHistoryAccount,
>>>>>>> 16c23d6 (renamed Log events from InitializedValidatorHistoryAccount and InitializedClusterHistoryAccount to InitializeValidatorHistoryAccount and InitializeClusterHistoryAccount to keep consistency in function names with original code)
};

use anchor_lang::{prelude::*, solana_program::vote};
use validator_history_vote_state::VoteStateVersions;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeValidatorHistoryAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = MAX_ALLOC_BYTES,
        seeds = [ValidatorHistory::SEED, vote_account.key().as_ref()],
        bump
    )]
    pub validator_history_account: AccountLoader<'info, ValidatorHistory>,
    /// CHECK: Safe because we check the vote program is the owner before deserialization.
    #[account(owner = vote::program::ID.key())]
    pub vote_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handle_initialize_validator_history_account(
    ctx: Context<InitializeValidatorHistoryAccount>,
) -> Result<()> {
    // Need minimum 5 epochs of vote credits to be valid
    let epoch_credits = VoteStateVersions::deserialize_epoch_credits(&ctx.accounts.vote_account)?;
    if epoch_credits.len() < MIN_VOTE_EPOCHS {
        return Err(ValidatorHistoryError::NotEnoughVotingHistory.into());
    }

<<<<<<< HEAD
<<<<<<< HEAD
    emit_cpi!(LogInitializedValidatorHistoryAccount {
=======
    emit_cpi!(LogInitializeValidatorHistoryAccount {
>>>>>>> master
=======
    emit_cpi!(LogInitializeValidatorHistoryAccount {
>>>>>>> 16c23d6 (renamed Log events from InitializedValidatorHistoryAccount and InitializedClusterHistoryAccount to InitializeValidatorHistoryAccount and InitializeClusterHistoryAccount to keep consistency in function names with original code)
        validator_history_account: ctx.accounts.validator_history_account.key(),
        vote_account: ctx.accounts.vote_account.key(),
        signer: ctx.accounts.signer.owner.key()
    });

    Ok(())
}

