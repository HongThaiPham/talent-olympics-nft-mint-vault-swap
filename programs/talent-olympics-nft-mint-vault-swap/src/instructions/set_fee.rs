use anchor_lang::prelude::*;

use crate::{error::MyErrorCode, ProtocolConfig, CONFIG_SEED};

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_ref()],
        bump ,
        constraint = config.authority == signer.key() @MyErrorCode::Unauthorized
    )]
    pub config: Account<'info, ProtocolConfig>,
}

impl<'info> SetFee<'info> {
    pub fn handler(&mut self, fee: u64) -> Result<()> {
        self.config.fee = fee;
        Ok(())
    }
}
