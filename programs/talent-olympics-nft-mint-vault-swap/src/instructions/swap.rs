use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer},
};

use crate::{error::MyErrorCode, Locker, ProtocolConfig, CONFIG_SEED, LOCKER_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct Swap<'info> {
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
    #[account(mut)]
    pub old_owner: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [LOCKER_SEED.as_ref(), asset.key.as_ref() , old_owner.key.as_ref()],
        bump,
        close = old_owner,
        constraint = locker.owner == old_owner.key() @MyErrorCode::Unauthorized,
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

impl<'info> Swap<'info> {
    pub fn handler(&mut self, bumps: SwapBumps) -> Result<()> {
        self.tranfer_sol_to_old_owner()?;
        self.transfer_nft(bumps.vault)?;
        Ok(())
    }

    fn tranfer_sol_to_old_owner(&mut self) -> Result<()> {
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.old_owner.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        system_program::transfer(ctx, self.locker.lamports)?;
        Ok(())
    }

    fn transfer_nft(&mut self, vault_bump: u8) -> Result<()> {
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
        // .invoke_signed(signer_seeds)?;
        .invoke_signed(&[&[VAULT_SEED, &[vault_bump]]])?;
        Ok(())
    }
}
