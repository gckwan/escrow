use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  /// Invalid instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
}

/// Implement the From trait for ProgramError so it can convert from EscrowError.
/// We do this because the entrypoint returns a Result of either nothing or a ProgramError,
/// so an EscrowError would not be valid, but the others would be.
impl From<EscrowError> for ProgramError {
  /// The function that carries out the conversion
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
