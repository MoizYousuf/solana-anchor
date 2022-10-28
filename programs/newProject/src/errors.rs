use anchor_lang::prelude::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Instruction Overflowed!")]
    Overflow,
}