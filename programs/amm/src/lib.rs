pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D58puanNfoPFjurvZ1NqZ6ag2eJuCQZBdL565HvtYRG1");

#[program]
pub mod amm {
    use super::*;

    // pub fn initialize(
    //     ctx: Context<Initialize>,
    //     seed: u64,
    //     fee: u16,
    //     authority: Option<Pubkey>,
    // ) -> Result<()> {
    //     ctx.accounts.init(seed, fee, authority, ctx.bumps)
    // }
}
