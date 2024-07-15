use anchor_lang::prelude::*;

use crate::{ProtocolConfig, CONFIG_SEED, DISCRIMINATOR_SIZE, VAULT_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = DISCRIMINATOR_SIZE + ProtocolConfig::INIT_SPACE,
        seeds = [CONFIG_SEED.as_ref()],
        bump 
    )]
    pub config: Account<'info, ProtocolConfig>,
    /// CHECK: it's ok to use
    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, fee: u64) -> Result<()> {
        self.config.set_inner(
            ProtocolConfig {
                authority: self.signer.to_account_info().key(),
                vault: self.vault.to_account_info().key(),
                fee
            }
        );
        Ok(())
    }
}
