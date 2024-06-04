use anchor_lang::prelude::*;

declare_id!("EL3zioyKuHaox1ra3PzKKjKvdNtMHh71paufgkYYPr5V");

#[program]
pub mod hello_world_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
