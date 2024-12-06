use anchor_lang::prelude::*;

declare_id!("EtjK1Bo1P4QEkM7xRfGduehuPxgHL5FunuLprksT7mFc");

#[program]
pub mod meme_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
