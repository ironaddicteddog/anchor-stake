use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::token::{self, TokenAccount, Transfer};
use registry_realizor::IsRealized;
use std::collections::BTreeMap;

mod calculator;

declare_id!("DCrDvbtuqxFiAgTz7JrDy5c1PiuZkgfphyGUTpq2L2eW");

#[program]
pub mod lockup {
    use super::*;

    pub const WHITELIST_SIZE: usize = 10;

    pub fn whitelist_new(ctx: Context<WhitelistNew>, _lockup_nonce: u64) -> Result<()> {
        let mut whitelist = vec![];
        whitelist.resize(WHITELIST_SIZE, Default::default());
        ctx.accounts.lockup.authority = *ctx.accounts.authority.key;
        ctx.accounts.lockup.whitelist = whitelist;

        Ok(())
    }

    #[access_control(whitelist_auth(&ctx))]
    pub fn whitelist_add(ctx: Context<Auth>, _lockup_nonce: u64, entry: WhitelistEntry) -> Result<()> {
        if ctx.accounts.lockup.whitelist.len() == WHITELIST_SIZE {
            return Err(ErrorCode::WhitelistFull.into());
        }
        if ctx.accounts.lockup.whitelist.contains(&entry) {
            return Err(ErrorCode::WhitelistEntryAlreadyExists.into());
        }
        ctx.accounts.lockup.whitelist.push(entry);

        Ok(())
    }

    #[access_control(whitelist_auth(&ctx))]
    pub fn whitelist_delete(ctx: Context<Auth>, _lockup_nonce: u64, entry: WhitelistEntry) -> Result<()> {
        if !ctx.accounts.lockup.whitelist.contains(&entry) {
            return Err(ErrorCode::WhitelistEntryNotFound.into());
        }
        ctx.accounts.lockup.whitelist.retain(|e| e != &entry);

        Ok(())
    }

    #[access_control(whitelist_auth(&ctx))]
    pub fn set_authority(ctx: Context<Auth>, _lockup_nonce: u64, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.lockup.authority = new_authority;

        Ok(())
    }

    #[access_control(CreateVesting::accounts(&ctx, nonce))]
    pub fn create_vesting(
        ctx: Context<CreateVesting>,
        beneficiary: Pubkey,
        deposit_amount: u64,
        nonce: u8,
        start_ts: i64,
        end_ts: i64,
        period_count: u64,
        realizor: Option<Realizor>,
    ) -> Result<()> {
        if deposit_amount == 0 {
            return Err(ErrorCode::InvalidDepositAmount.into());
        }
        if !is_valid_schedule(start_ts, end_ts, period_count) {
            return Err(ErrorCode::InvalidSchedule.into());
        }
        let vesting = &mut ctx.accounts.vesting;
        vesting.beneficiary = beneficiary;
        vesting.mint = ctx.accounts.vault.mint;
        vesting.vault = *ctx.accounts.vault.to_account_info().key;
        vesting.period_count = period_count;
        vesting.start_balance = deposit_amount;
        vesting.end_ts = end_ts;
        vesting.start_ts = start_ts;
        vesting.created_ts = ctx.accounts.clock.unix_timestamp;
        vesting.outstanding = deposit_amount;
        vesting.whitelist_owned = 0;
        vesting.grantor = *ctx.accounts.depositor_authority.key;
        vesting.nonce = nonce;
        vesting.realizor = realizor;

        token::transfer(ctx.accounts.into(), deposit_amount)?;

        Ok(())
    }

    #[access_control(is_realized(&ctx))]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let available_for_withdrawal = calculator::available_for_withdrawal(
            &ctx.accounts.vesting,
            ctx.accounts.clock.unix_timestamp,
        );

        // Has the given amount vested?
        if amount > available_for_withdrawal
        {
            return Err(ErrorCode::InsufficientWithdrawalBalance.into());
        }

        // Transfer funds out.
        let seeds = &[
            ctx.accounts.vesting.to_account_info().key.as_ref(),
            &[ctx.accounts.vesting.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::transfer(cpi_ctx, amount)?;

        // Bookeeping.
        let vesting = &mut ctx.accounts.vesting;
        vesting.outstanding -= amount;

        Ok(())
    }

    // Sends funds from the lockup program to a whitelisted program.
    pub fn whitelist_withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WhitelistWithdraw<'info>>,
        instruction_data: Vec<u8>,
        amount: u64,
    ) -> Result<()> {
        let before_amount = ctx.accounts.transfer.vault.amount;
        whitelist_relay_cpi(
            &ctx.accounts.transfer,
            ctx.remaining_accounts,
            instruction_data,
        )?;
        let after_amount = ctx.accounts.transfer.vault.amount;

        // CPI safety checks.
        let withdraw_amount = before_amount - after_amount;
        if withdraw_amount > amount {
            return Err(ErrorCode::WhitelistWithdrawLimit)?;
        }

        // Bookeeping.
        ctx.accounts.transfer.vesting.whitelist_owned += withdraw_amount;

        Ok(())
    }

    // Sends funds from a whitelisted program back to the lockup program.
    pub fn whitelist_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WhitelistDeposit<'info>>,
        instruction_data: Vec<u8>,
    ) -> Result<()> {
        let before_amount = ctx.accounts.transfer.vault.amount;
        whitelist_relay_cpi(
            &ctx.accounts.transfer,
            ctx.remaining_accounts,
            instruction_data,
        )?;
        let after_amount = ctx.accounts.transfer.vault.amount;

        // CPI safety checks.
        let deposit_amount = after_amount - before_amount;
        if deposit_amount <= 0 {
            return Err(ErrorCode::InsufficientWhitelistDepositAmount)?;
        }
        if deposit_amount > ctx.accounts.transfer.vesting.whitelist_owned {
            return Err(ErrorCode::WhitelistDepositOverflow)?;
        }

        // Bookkeeping.
        ctx.accounts.transfer.vesting.whitelist_owned -= deposit_amount;

        Ok(())
    }

    // Convenience function for UI's to calculate the withdrawable amount.
    pub fn available_for_withdrawal(ctx: Context<AvailableForWithdrawal>) -> Result<()> {
        let available = calculator::available_for_withdrawal(
            &ctx.accounts.vesting,
            ctx.accounts.clock.unix_timestamp,
        );
        // Log as string so that JS can read as a BN.
        msg!(&format!("{{ \"result\": \"{}\" }}", available));
        Ok(())
    }
}

#[account]
pub struct Lockup {
    /// The key with the ability to change the whitelist.
    pub authority: Pubkey,
    /// List of programs locked tokens can be sent to. These programs
    /// are completely trusted to maintain the locked property.
    pub whitelist: Vec<WhitelistEntry>,
}

#[derive(Accounts)]
#[instruction(lockup_nonce: u64)]
pub struct WhitelistNew<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"lockup".as_ref(), &lockup_nonce.to_le_bytes()],
        bump,
        payer = authority,
        space = 1000
    )]
    pub lockup: Box<Account<'info, Lockup>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(lockup_nonce: u64)]
pub struct Auth<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"lockup".as_ref(), &lockup_nonce.to_le_bytes()],
        bump
    )]
    pub lockup: Box<Account<'info, Lockup>>,
}

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    // Vesting.
    #[account(zero)]
    pub vesting: Box<Account<'info, Vesting>>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    // Depositor.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    pub depositor_authority: Signer<'info>,
    // Misc.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> CreateVesting<'info> {
    fn accounts(ctx: &Context<CreateVesting>, nonce: u8) -> Result<()> {
        let vault_authority = Pubkey::create_program_address(
            &[
                ctx.accounts.vesting.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| ErrorCode::InvalidProgramAddress)?;
        if ctx.accounts.vault.owner != vault_authority {
            return Err(ErrorCode::InvalidVaultOwner)?;
        }

        Ok(())
    }
}

// All accounts not included here, i.e., the "remaining accounts" should be
// ordered according to the realization interface.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    // Vesting.
    #[account(mut, has_one = beneficiary, has_one = vault)]
    vesting: Box<Account<'info, Vesting>>,
    beneficiary: Signer<'info>,
    #[account(mut)]
    vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [vesting.to_account_info().key.as_ref()], bump = vesting.nonce)]
    vesting_signer: AccountInfo<'info>,
    // Withdraw receiving target..
    #[account(mut)]
    token: Account<'info, TokenAccount>,
    // Misc.
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct WhitelistWithdraw<'info> {
    transfer: WhitelistTransfer<'info>,
}

#[derive(Accounts)]
pub struct WhitelistDeposit<'info> {
    transfer: WhitelistTransfer<'info>,
}

#[derive(Accounts)]
#[instruction(lockup_nonce: u64)]
pub struct WhitelistTransfer<'info> {
    #[account(
        seeds = [b"lockup".as_ref(), &lockup_nonce.to_le_bytes()],
        bump
    )]
    lockup: Box<Account<'info, Lockup>>,
    // lockup: ProgramState<'info, Lockup>,
    beneficiary: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    whitelisted_program: AccountInfo<'info>,

    // Whitelist interface.
    #[account(mut, has_one = beneficiary, has_one = vault)]
    vesting: Box<Account<'info, Vesting>>,
    #[account(mut, "&vault.owner == vesting_signer.key")]
    vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [vesting.to_account_info().key.as_ref()], bump = vesting.nonce)]
    vesting_signer: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    whitelisted_program_vault: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    whitelisted_program_vault_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AvailableForWithdrawal<'info> {
    vesting: Box<Account<'info, Vesting>>,
    clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Vesting {
    /// The owner of this Vesting account.
    pub beneficiary: Pubkey,
    /// The mint of the SPL token locked up.
    pub mint: Pubkey,
    /// Address of the account's token vault.
    pub vault: Pubkey,
    /// The owner of the token account funding this account.
    pub grantor: Pubkey,
    /// The outstanding SRM deposit backing this vesting account. All
    /// withdrawals will deduct this balance.
    pub outstanding: u64,
    /// The starting balance of this vesting account, i.e., how much was
    /// originally deposited.
    pub start_balance: u64,
    /// The unix timestamp at which this vesting account was created.
    pub created_ts: i64,
    /// The time at which vesting begins.
    pub start_ts: i64,
    /// The time at which all tokens are vested.
    pub end_ts: i64,
    /// The number of times vesting will occur. For example, if vesting
    /// is once a year over seven years, this will be 7.
    pub period_count: u64,
    /// The amount of tokens in custody of whitelisted programs.
    pub whitelist_owned: u64,
    /// Signer nonce.
    pub nonce: u8,
    /// The program that determines when the locked account is **realized**.
    /// In addition to the lockup schedule, the program provides the ability
    /// for applications to determine when locked tokens are considered earned.
    /// For example, when earning locked tokens via the staking program, one
    /// cannot receive the tokens until unstaking. As a result, if one never
    /// unstakes, one would never actually receive the locked tokens.
    pub realizor: Option<Realizor>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Realizor {
    /// Program to invoke to check a realization condition. This program must
    /// implement the `RealizeLock` trait.
    pub program: Pubkey,
    /// Address of an arbitrary piece of metadata interpretable by the realizor
    /// program. For example, when a vesting account is allocated, the program
    /// can define its realization condition as a function of some account
    /// state. The metadata is the address of that account.
    ///
    /// In the case of staking, the metadata is a `Member` account address. When
    /// the realization condition is checked, the staking program will check the
    /// `Member` account defined by the `metadata` has no staked tokens.
    pub metadata: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Copy, Clone)]
pub struct WhitelistEntry {
    pub program_id: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Vesting end must be greater than the current unix timestamp.")]
    InvalidTimestamp,
    #[msg("The number of vesting periods must be greater than zero.")]
    InvalidPeriod,
    #[msg("The vesting deposit amount must be greater than zero.")]
    InvalidDepositAmount,
    #[msg("The Whitelist entry is not a valid program address.")]
    InvalidWhitelistEntry,
    #[msg("Invalid program address. Did you provide the correct nonce?")]
    InvalidProgramAddress,
    #[msg("Invalid vault owner.")]
    InvalidVaultOwner,
    #[msg("Vault amount must be zero.")]
    InvalidVaultAmount,
    #[msg("Insufficient withdrawal balance.")]
    InsufficientWithdrawalBalance,
    #[msg("Whitelist is full")]
    WhitelistFull,
    #[msg("Whitelist entry already exists")]
    WhitelistEntryAlreadyExists,
    #[msg("Balance must go up when performing a whitelist deposit")]
    InsufficientWhitelistDepositAmount,
    #[msg("Cannot deposit more than withdrawn")]
    WhitelistDepositOverflow,
    #[msg("Tried to withdraw over the specified limit")]
    WhitelistWithdrawLimit,
    #[msg("Whitelist entry not found.")]
    WhitelistEntryNotFound,
    #[msg("You do not have sufficient permissions to perform this action.")]
    Unauthorized,
    #[msg("You are unable to realize projected rewards until unstaking.")]
    UnableToWithdrawWhileStaked,
    #[msg("The given lock realizor doesn't match the vesting account.")]
    InvalidLockRealizor,
    #[msg("You have not realized this vesting account.")]
    UnrealizedVesting,
    #[msg("Invalid vesting schedule given.")]
    InvalidSchedule,
}

impl<'a, 'b, 'c, 'info> From<&mut CreateVesting<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut CreateVesting<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vault.to_account_info(),
            authority: accounts.depositor_authority.to_account_info().clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'a, 'b, 'c, 'info> From<&Withdraw<'info>> for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    fn from(accounts: &Withdraw<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.vault.to_account_info(),
            to: accounts.token.to_account_info(),
            authority: accounts.vesting_signer.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[access_control(is_whitelisted(transfer))]
pub fn whitelist_relay_cpi<'info>(
    transfer: &WhitelistTransfer<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    instruction_data: Vec<u8>,
) -> Result<()> {
    let mut meta_accounts = vec![
        AccountMeta::new_readonly(*transfer.vesting.to_account_info().key, false),
        AccountMeta::new(*transfer.vault.to_account_info().key, false),
        AccountMeta::new_readonly(*transfer.vesting_signer.to_account_info().key, true),
        AccountMeta::new_readonly(*transfer.token_program.to_account_info().key, false),
        AccountMeta::new(
            *transfer.whitelisted_program_vault.to_account_info().key,
            false,
        ),
        AccountMeta::new_readonly(
            *transfer
                .whitelisted_program_vault_authority
                .to_account_info()
                .key,
            false,
        ),
    ];
    meta_accounts.extend(remaining_accounts.iter().map(|a| {
        if a.is_writable {
            AccountMeta::new(*a.key, a.is_signer)
        } else {
            AccountMeta::new_readonly(*a.key, a.is_signer)
        }
    }));
    let relay_instruction = Instruction {
        program_id: *transfer.whitelisted_program.to_account_info().key,
        accounts: meta_accounts,
        data: instruction_data.to_vec(),
    };

    let seeds = &[
        transfer.vesting.to_account_info().key.as_ref(),
        &[transfer.vesting.nonce],
    ];
    let signer = &[&seeds[..]];
    let mut accounts = transfer.to_account_infos();
    accounts.extend_from_slice(&remaining_accounts);
    invoke_signed(&relay_instruction, &accounts, signer).map_err(Into::into)
}

pub fn is_whitelisted<'info>(transfer: &WhitelistTransfer<'info>) -> Result<()> {
    if !transfer.lockup.whitelist.contains(&WhitelistEntry {
        program_id: *transfer.whitelisted_program.key,
    }) {
        return Err(ErrorCode::WhitelistEntryNotFound.into());
    }
    Ok(())
}

// fn whitelist_auth(lockup: &Lockup, ctx: &Context<Auth>) -> ProgramResult {
//     if &lockup.authority != ctx.accounts.authority.key {
//         return Err(ErrorCode::Unauthorized.into());
//     }
//     Ok(())
// }

fn whitelist_auth(ctx: &Context<Auth>) -> Result<()> {
    if ctx.accounts.lockup.authority != *ctx.accounts.authority.key {
        return Err(ErrorCode::Unauthorized.into());
    }
    Ok(())
}

pub fn is_valid_schedule(start_ts: i64, end_ts: i64, period_count: u64) -> bool {
    if end_ts <= start_ts {
        return false;
    }
    if period_count > (end_ts - start_ts) as u64 {
        return false;
    }
    if period_count == 0 {
        return false;
    }
    true
}

// Returns Ok if the locked vesting account has been "realized". Realization
// is application dependent. For example, in the case of staking, one must first
// unstake before being able to earn locked tokens.
fn is_realized(ctx: &Context<Withdraw>) -> Result<()> {
    if let Some(realizor) = &ctx.accounts.vesting.realizor {
        let cpi_program = {
            let p = ctx.remaining_accounts[0].clone();
            if p.key != &realizor.program {
                return Err(ErrorCode::InvalidLockRealizor.into());
            }
            p
        };
        let registry_program = ctx.remaining_accounts[1].clone();
        // let cpi_accounts = ctx.remaining_accounts.to_vec()[1..].to_vec();
        let mut bumps = BTreeMap::new();
        let cpi_accounts = {
            let accs = IsRealized::try_accounts(
                // cpi_program.key,
                registry_program.key,
                &mut &ctx.remaining_accounts.to_vec()[2..=4],
                &[],
                &mut bumps,
            )?;
            registry_realizor::cpi::accounts::IsRealized {
                member: accs.member.to_account_info(),
                member_spt: accs.member_spt.to_account_info(),
                member_spt_locked: accs.member_spt_locked.to_account_info(),
            }
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let vesting_account = (*ctx.accounts.vesting).clone();
        let realizor = registry_realizor::Realizor {
            program: vesting_account.realizor.as_ref().unwrap().program,
            metadata: vesting_account.realizor.as_ref().unwrap().metadata,
        };
        let vesting = registry_realizor::Vesting {
            beneficiary: vesting_account.beneficiary,
            mint: vesting_account.mint,
            vault: vesting_account.vault,
            grantor: vesting_account.grantor,
            outstanding: vesting_account.outstanding,
            start_balance: vesting_account.start_balance,
            created_ts: vesting_account.created_ts,
            start_ts: vesting_account.start_ts,
            end_ts: vesting_account.end_ts,
            period_count: vesting_account.period_count,
            whitelist_owned: vesting_account.whitelist_owned,
            nonce: vesting_account.nonce,
            // realizor: vesting_account.realizor.clone(),
            realizor: Some(realizor),
        };

        let member_data = registry_realizor::MemberData {
            registrar: ctx.remaining_accounts[5].key(),
            beneficiary: ctx.remaining_accounts[6].key(),
            metadata: ctx.remaining_accounts[7].key(),
            balances_spt: ctx.remaining_accounts[8].key(),
            balances_locked_spt: ctx.remaining_accounts[9].key(),
        };

        // realize_lock::is_realized(cpi_ctx, vesting).map_err(|_| ErrorCode::UnrealizedVesting)?;

        registry_realizor::cpi::is_realized(cpi_ctx, vesting, member_data)
            .map_err(|_| ErrorCode::UnrealizedVesting)?;
    }
    Ok(())
}

/// RealizeLock defines the interface an external program must implement if
/// they want to define a "realization condition" on a locked vesting account.
/// This condition must be satisfied *even if a vesting schedule has
/// completed*. Otherwise the user can never earn the locked funds. For example,
/// in the case of the staking program, one cannot received a locked reward
/// until one has completely unstaked.
#[interface]
pub trait RealizeLock<'info, T: Accounts<'info>> {
    fn is_realized(ctx: Context<T>, v: Vesting) -> Result<()>;
}
