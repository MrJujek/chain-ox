use anchor_lang::prelude::*;

declare_id!("3hFYYYFMpcemDqLinR933Q7LuAiZtrcDsg2ZJjYdURWr");

#[program]
pub mod chain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
