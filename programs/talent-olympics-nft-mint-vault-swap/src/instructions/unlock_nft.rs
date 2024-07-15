use anchor_lang::prelude::*;

use crate::{error::MyErrorCode, Locker, ProtocolConfig, CONFIG_SEED, LOCKER_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct UnlockNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      seeds = [CONFIG_SEED.as_ref()],
      bump,
      has_one = vault
    )]
    pub config: Account<'info, ProtocolConfig>,
    /// CHECK: it's ok to use
    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    pub vault: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [LOCKER_SEED.as_ref(), asset.key.as_ref() , signer.key.as_ref()],
        bump,
        close = signer,
        constraint = locker.owner == signer.key() @MyErrorCode::Unauthorized,
        constraint = locker.asset == asset.key() @MyErrorCode::InvalidLocker,
    )]
    pub locker: Account<'info, Locker>,

    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,
    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnlockNft<'info> {
    pub fn handler(&mut self, bumps: UnlockNftBumps) -> Result<()> {
        self.return_nft(bumps.vault)?;
        Ok(())
    }

    fn return_nft(&self, vault_bump: u8) -> Result<()> {
        mpl_core::instructions::TransferV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.signer.to_account_info(),
            authority: Some(&self.vault.to_account_info()),
            new_owner: &self.signer.to_account_info(),
            system_program: Some(&self.system_program.to_account_info()),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: mpl_core::instructions::TransferV1InstructionArgs {
                compression_proof: None,
            },
        }
        .invoke_signed(&[&[VAULT_SEED, &[vault_bump]]])?;

        Ok(())
    }
}
