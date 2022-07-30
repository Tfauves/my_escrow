use anchor_lang::prelude::*;
use anchor_spl::token::{CloseAccount, Mint, Token, TokenAccount, Transfer};

#[account]
pub struct EscrowInfo {
    pub init_user: Pubkey,
    pub token1_mint: Pubkey,
    pub token2_mint: Pubkey,
    pub init_user_token1: Pubkey,
    pub init_user_token2: Pubkey,
    pub token1_amt: u64,
    pub token2_amt: u64,
}

#[derive(Accounts)]
pub struct StartEscrow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + 8 + 8 + (6 * 32))]
    pub escrow_account: Box<Account<'info, EscrowInfo>>,

    pub token1_mint: Box<Account<'info, Mint>>,

    pub token2_mint: Box<Account<'info, Mint>>,

    #[account(mut, constraint = init_user_token1.mint == token1_mint.key())]
    pub init_user_token1: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = init_user_token2.mint == token2_mint.key())]
    pub init_user_token2: Box<Account<'info, TokenAccount>>,

    #[account(init, payer = payer,
        seeds = [b"token1_vault".as_ref(), &payer.to_account_info().key().clone().to_bytes(), &token1_mint.to_account_info().key().clone().to_bytes()],
        bump, token::mint = token1_mint, token::authority = vault_owner)]
    pub token1_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: none
    #[account(seeds = [b"vault_owner".as_ref()],bump)]
    pub vault_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub escrow_account: Box<Account<'info, EscrowInfo>>,

    #[account(mut)]
    pub init_user_token1: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub token1_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: none
    #[account(mut)]
    pub vault_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = escrow_account.token2_amt <= user2_token2.amount)]
    pub escrow_account: Box<Account<'info, EscrowInfo>>,

    #[account(mut)]
    pub init_user_token1: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub init_user_token2: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = user2_token1.mint == escrow_account.token1_mint.key())]
    pub user2_token1: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = user2_token2.mint == escrow_account.token2_mint.key())]
    pub user2_token2: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub token1_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: none
    #[account(mut)]
    pub vault_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> StartEscrow<'info> {
    pub fn transfer_user1_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer{
                from: self.init_user_token1.to_account_info().clone(),
                to: self.token1_vault.to_account_info().clone(),
                authority: self.payer.to_account_info().clone(),
            },
        )
    }
}

impl<'info> CancelEscrow<'info> {
    pub fn transfer_vault_to_user1(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer{
                from: self.token1_vault.to_account_info().clone(),
                to: self.init_user_token1.to_account_info().clone(),
                authority: self.vault_owner.to_account_info(),
            },
        )
    }

    pub fn close_vault_account(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount{
                account: self.token1_vault.to_account_info().clone(),
                destination: self.payer.to_account_info().clone(),
                authority: self.vault_owner.to_account_info().clone(),
            },
        )
    }
}

impl<'info> Exchange<'info> {
    pub fn transfer_token(
        &self,
        from: Account<'info, TokenAccount>,
        to: Account<'info, TokenAccount>,
        authority: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: from.to_account_info().clone(),
                to: to.to_account_info().clone(),
                authority: authority.clone(),
            },
        )
    }

    pub fn close_vault_account(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.token1_vault.to_account_info().clone(),
                destination: self.payer.to_account_info().clone(),
                authority: self.vault_owner.clone(),
            },
        )
    }
}
