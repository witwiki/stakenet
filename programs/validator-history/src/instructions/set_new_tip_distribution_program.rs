use anchor_lang::prelude::*;

use crate::state::Config;
use crate::logs::LogSetNewTipDistributionProgram;

#[event_cpi]
#[derive(Accounts)]
pub struct SetNewTipDistributionProgram<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = admin,
    )]
    pub config: Account<'info, Config>,
    /// CHECK: fine since we are not deserializing account
    #[account(executable)]
    pub new_tip_distribution_program: AccountInfo<'info>,
    pub admin: Signer<'info>,
}

pub fn handle_set_new_tip_distribution_program(
    ctx: Context<SetNewTipDistributionProgram>,
) -> Result<()> {
    ctx.accounts.config.tip_distribution_program = ctx.accounts.new_tip_distribution_program.key();

    emit_cpi!(LogSetNewTipDistributionProgram {
        config: ctx.accounts.config.key(),
        new_tip_distribution_program: ctx.accounts.new_tip_distribution_program.key(),
        admin: ctx.accounts.admin.owner.key()
    });

    Ok(())
}
