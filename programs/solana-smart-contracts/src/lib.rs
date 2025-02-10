use anchor_lang::prelude::*;

declare_id!("6iPxdjHimmNrLemHX7RX6NwvEJiBAaChVLA9ivQtUy3b");

#[program]
pub mod my_first_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.number = 0;
        msg!("Initialized with number: {}", base_account.number);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_number: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.number = new_number;
        msg!("Updated number to: {}", base_account.number);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub number: u64,
}
