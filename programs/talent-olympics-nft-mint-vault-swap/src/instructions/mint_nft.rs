use anchor_lang::prelude::*;
use mpl_core::types::{DataState, PluginAuthorityPair};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The address of the new asset.
    #[account(mut)]
    pub asset: Signer<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The owner of the new asset. Defaults to the authority if not present.
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,

    /// The authority on the new asset.
    /// CHECK: Checked in mpl-core.
    pub update_authority: Option<AccountInfo<'info>>,

    // The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl<'info> MintNft<'info> {
    pub fn handler(&mut self, args: CreateAssetArgs) -> Result<()> {
        mpl_core::instructions::CreateV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            authority: Some(&self.authority.to_account_info()),
            payer: &self.authority.to_account_info(),
            owner: self.owner.as_ref(),
            update_authority: self.update_authority.as_ref(),
            system_program: &self.system_program.to_account_info(),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: mpl_core::instructions::CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;
        Ok(())
    }
}
