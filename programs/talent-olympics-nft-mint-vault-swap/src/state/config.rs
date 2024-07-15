use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    pub authority: Pubkey, // authority of the protocol
    pub vault: Pubkey,     // keeper of the protocol's fee
    pub fee: u64,          // how many lamports to charge for each lock
}
