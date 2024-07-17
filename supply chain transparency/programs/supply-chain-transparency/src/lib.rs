use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock::Clock, sysvar::Sysvar};

declare_id!("FycGESQEzfzKwXk8RHoZCZUGTkkcxSidDTgACWRevrsX");

#[program]
pub mod supply_chain_transparency {
    use super::*;

    /// Initializes a new traceability account with the given product name.
    pub fn initialize_traceability(ctx: Context<InitializeTraceability>, product_name: String) -> Result<()> {
        let traceability_account = &mut ctx.accounts.traceability_account;
        traceability_account.product_name = product_name.clone();
        traceability_account.origin = "Farm".to_string();
        traceability_account.stages = vec![];
        traceability_account.certifications = vec![];
        traceability_account.last_update_time = Clock::get()?.unix_timestamp;
        traceability_account.owner = ctx.accounts.user.key();
        emit!(TraceabilityInitialized {
            product_name: product_name.clone(),
        });
        Ok(())
    }

    /// Updates the traceability stages for an existing traceability account.
    pub fn update_traceability(ctx: Context<UpdateTraceability>, stage: String) -> Result<()> {
        let traceability_account = &mut ctx.accounts.traceability_account;
        
        // Rate limiting: ensure updates are not too frequent
        let current_time = Clock::get()?.unix_timestamp;
        if current_time - traceability_account.last_update_time < 60 {
            return Err(CustomError::RateLimitExceeded.into());
        }
        
        traceability_account.stages.push(stage.clone());
        traceability_account.last_update_time = current_time;
        emit!(TraceabilityStageUpdated {
            product_name: traceability_account.product_name.clone(),
            stage,
        });
        Ok(())
    }

    /// Adds a certification to an existing traceability account.
    pub fn add_certification(ctx: Context<AddCertification>, certification: String) -> Result<()> {
        let traceability_account = &mut ctx.accounts.traceability_account;
        
        // Rate limiting: ensure updates are not too frequent
        let current_time = Clock::get()?.unix_timestamp;
        if current_time - traceability_account.last_update_time < 60 {
            return Err(CustomError::RateLimitExceeded.into());
        }
        
        traceability_account.certifications.push(certification.clone());
        traceability_account.last_update_time = current_time;
        emit!(CertificationAdded {
            product_name: traceability_account.product_name.clone(),
            certification,
        });
        Ok(())
    }

    /// Closes the traceability account and reclaims storage.
    pub fn close_traceability(ctx: Context<CloseTraceability>) -> Result<()> {
        let traceability_account = &mut ctx.accounts.traceability_account;
        let recipient = &mut ctx.accounts.recipient;
        **recipient.to_account_info().try_borrow_mut_lamports()? += traceability_account.to_account_info().lamports();
        **traceability_account.to_account_info().try_borrow_mut_lamports()? = 0;
        traceability_account.to_account_info().data.borrow_mut().fill(0);
        emit!(TraceabilityClosed {
            product_name: traceability_account.product_name.clone(),
        });
        Ok(())
    }

    /// Fetches a paginated list of stages for the traceability account.
    pub fn get_stages(ctx: Context<GetStages>, start: u64, end: u64) -> Result<Vec<String>> {
        let traceability_account = &ctx.accounts.traceability_account;
        if start >= end || end as usize > traceability_account.stages.len() {
            return Err(CustomError::InvalidPagination.into());
        }
        Ok(traceability_account.stages[start as usize..end as usize].to_vec())
    }

    /// Fetches a paginated list of certifications for the traceability account.
    pub fn get_certifications(ctx: Context<GetCertifications>, start: u64, end: u64) -> Result<Vec<String>> {
        let traceability_account = &ctx.accounts.traceability_account;
        if start >= end || end as usize > traceability_account.certifications.len() {
            return Err(CustomError::InvalidPagination.into());
        }
        Ok(traceability_account.certifications[start as usize..end as usize].to_vec())
    }

    /// Adds a new user with a specified role.
    pub fn add_user(ctx: Context<AddUser>, role: UserRole) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.role = role.clone();
        emit!(UserAdded {
            user: ctx.accounts.admin.key(),
            role,
        });
        Ok(())
    }

    /// Updates the role of an existing user.
    pub fn update_user_role(ctx: Context<UpdateUserRole>, role: UserRole) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.role = role.clone();
        emit!(UserRoleUpdated {
            user: ctx.accounts.user_account.key(),
            role,
        });
        Ok(())
    }
}

#[account]
pub struct Traceability {
    pub product_name: String,
    pub origin: String,
    pub stages: Vec<String>,
    pub certifications: Vec<String>,
    pub last_update_time: i64, // Timestamp of the last update for rate limiting
    pub owner: Pubkey,         // Owner of the account
}

#[account]
pub struct UserAccount {
    pub role: UserRole,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Verifier,
    User,
}

#[derive(Accounts)]
pub struct InitializeTraceability<'info> {
    #[account(init, payer = user, space = 8 + Traceability::MAX_SIZE)]
    pub traceability_account: Account<'info, Traceability>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTraceability<'info> {
    #[account(mut, has_one = owner @ CustomError::Unauthorized)]
    pub traceability_account: Account<'info, Traceability>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddCertification<'info> {
    #[account(mut, has_one = owner @ CustomError::Unauthorized)]
    pub traceability_account: Account<'info, Traceability>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseTraceability<'info> {
    #[account(mut, has_one = owner @ CustomError::Unauthorized, close = recipient)]
    pub traceability_account: Account<'info, Traceability>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetStages<'info> {
    pub traceability_account: Account<'info, Traceability>,
}

#[derive(Accounts)]
pub struct GetCertifications<'info> {
    pub traceability_account: Account<'info, Traceability>,
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(init, payer = admin, space = 8 + UserAccount::MAX_SIZE)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUserRole<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Rate limit exceeded. Please wait before making another update.")]
    RateLimitExceeded,
    #[msg("Invalid pagination parameters.")]
    InvalidPagination,
}

#[event]
pub struct TraceabilityInitialized {
    pub product_name: String,
}

#[event]
pub struct TraceabilityStageUpdated {
    pub product_name: String,
    pub stage: String,
}

#[event]
pub struct CertificationAdded {
    pub product_name: String,
    pub certification: String,
}

#[event]
pub struct TraceabilityClosed {
    pub product_name: String,
}

#[event]
pub struct UserAdded {
    pub user: Pubkey,
    pub role: UserRole,
}

#[event]
pub struct UserRoleUpdated {
    pub user: Pubkey,
    pub role: UserRole,
}

impl Traceability {
    pub const MAX_SIZE: usize = 32 + 32 + 4 + 100 * 32 + 4 + 20 * 32 + 8 + 32;
}

impl UserAccount {
    pub const MAX_SIZE: usize = 1;
}
