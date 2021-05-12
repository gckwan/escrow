use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

  /*
  Starts the trade by creating and populating an escrow account and transferring control to the PDA

  Accounts:
  0. [signer] The account of the person initializing the escrow
  1. [writable] Temporary token account created prior, owned by initializer
  2. [] Initializer's token account for received token
  3. [writable] Escrow account holding all trade info
  4. [] Rent sysvar
  5. [] Token program

  */
  InitEscrow {
    // The amount party A expects to receive of token Y
    amount: u64
  }
}

  impl EscrowInstruction {
    // Unpack a byte buffer into an [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
      // The first byte is the tag, which indicates how to decode the rest of the slice.
      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

      Ok(match tag {
        0 => Self::InitEscrow {
          amount: Self::unpack_amount(rest?)
        },
        _ => return Err(InvalidInstruction.into())
      })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
      let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;
      Ok(amount)
    }

  }
