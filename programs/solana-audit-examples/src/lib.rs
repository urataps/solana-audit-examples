use anchor_lang::prelude::*;

declare_id!("HFi8ekN1E6Pyt2kTbnX55JhPoDCbvBcaM9GmKT4AaPAv");

#[program]
pub mod solana_audit_examples {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
