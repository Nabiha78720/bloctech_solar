use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

declare_id!("Fjk21FDQXLuYq4STKn3aF15KyGgTnmGkjxo6BqvAn4uq");

#[program]
pub mod new_solar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, tiers: Vec<Tier>) -> Result<()> {
        let initialize_account = &mut ctx.accounts.initialize_account;

        initialize_account.tiers= tiers;
        initialize_account.mint = ctx.accounts.mint.key();
        initialize_account.vault_account= ctx.accounts.vault_account.key();

        token:: transfer(ctx.accounts.transfer_to_vault_account(),amount)?;
        
        Ok(())
    }
}

#[derive(Debug,Clone, AnchorSerialize, AnchorDeserialize)]
pub enum TierType {
    Tier1,
    Tier2,
    Tier3,
}

#[derive(Debug,Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Tier {
    pub apr: u8,
    pub duration: u64,
    pub min_amount: u64,
}

#[derive(Debug,Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserData {
    pub user: Pubkey,
    pub tier: TierType,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer = owner, space = 8 + 296)]
    pub initialize_account: Account<'info,InitializeAccount>,

    #[account(init, payer = owner, token::mint = mint, token::authority = vault_authority,)]

    pub vault_account: Account<'info,TokenAccount>,

    pub mint: Account<'info,Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub owner_token_account: Account<'info,TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info,Rent>,
    pub system_program: Program<'info,System>,
}

impl<'info> Initialize<'info> {
    fn transfer_to_vault_account(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.owner_token_account.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.owner.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}


#[account]
pub struct InitializeAccount {
    pub mint: Pubkey,
    pub vault_account: Pubkey,
    pub tiers: Vec<Tier>,
    pub users: Vec<UserData>,
}

