use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use std::convert::Into;

declare_id!("6YHjQ62tRTRho5tkFdtMX9rQod2RkwZMz2VfSiGwK9v7");

#[program]
mod registry_realizor {
    use super::*;

    pub fn is_realized(
        ctx: Context<IsRealized>,
        v: Vesting,
        member_data: MemberData,
    ) -> Result<()> {
        // Secutiry Check
        assert!(member_data.balances_spt == *ctx.accounts.member_spt.to_account_info().key);
        assert!(
            member_data.balances_locked_spt
                == *ctx.accounts.member_spt_locked.to_account_info().key
        );

        if let Some(realizor) = &v.realizor {
            if &realizor.metadata != ctx.accounts.member.to_account_info().key {
                return Err(ErrorCode::InvalidRealizorMetadata.into());
            }
            // assert!(ctx.accounts.member.beneficiary == v.beneficiary);
            assert!(member_data.beneficiary == v.beneficiary);
            let total_staked =
                ctx.accounts.member_spt.amount + ctx.accounts.member_spt_locked.amount;
            if total_staked != 0 {
                return Err(ErrorCode::UnrealizedReward.into());
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IsRealized<'info> {
    // #[account(
    //     "&member.balances.spt == member_spt.to_account_info().key",
    //     "&member.balances_locked.spt == member_spt_locked.to_account_info().key"
    // )]
    // pub member: Box<Account<'info, Member>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub member: AccountInfo<'info>,
    pub member_spt: Account<'info, TokenAccount>,
    pub member_spt_locked: Account<'info, TokenAccount>,
}

// Member Data (Part)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MemberData {
    /// Registrar the member belongs to.
    pub registrar: Pubkey,
    /// The effective owner of the Member account.
    pub beneficiary: Pubkey,
    /// Arbitrary metadata account owned by any program.
    pub metadata: Pubkey,
    /// Sets of balances owned by the Member.
    pub balances_spt: Pubkey,
    /// Locked balances owned by the Member.
    pub balances_locked_spt: Pubkey,
}

// BalanceSandbox defines isolated funds that can only be deposited/withdrawn
// into the program.
//
// Once controlled by the program, the associated `Member` account's beneficiary
// can send funds to/from any of the accounts within the sandbox, e.g., to
// stake.
#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, Clone, PartialEq)]
pub struct BalanceSandbox {
    // Staking pool token.
    pub spt: Pubkey,
    // Free balance (deposit) vaults.
    pub vault: Pubkey,
    // Stake vaults.
    pub vault_stake: Pubkey,
    // Pending withdrawal vaults.
    pub vault_pw: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
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

#[error_code]
pub enum ErrorCode {
    #[msg("Locked rewards cannot be realized until one unstaked all tokens.")]
    UnrealizedReward,
    #[msg("The given member account does not match the realizor metadata.")]
    InvalidRealizorMetadata,
}
