use anchor_lang::prelude::*;

#[error_code]
pub enum MyErrorCode {
    Unauthorized,
    InvalidLocker,
}
