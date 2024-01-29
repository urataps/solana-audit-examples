use anchor_lang::prelude::*;

pub mod signer_authorization_insecure {

    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,
}