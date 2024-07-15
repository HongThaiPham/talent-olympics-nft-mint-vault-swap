use anchor_lang::prelude::*;
pub const DISCRIMINATOR_SIZE: usize = std::mem::size_of::<u64>();

#[constant]
pub const CONFIG_SEED: &[u8] = b"config";

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";
