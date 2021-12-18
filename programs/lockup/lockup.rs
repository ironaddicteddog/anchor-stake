#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::token::{self, TokenAccount, Transfer};
mod calculator {
    //! Utility functions for calculating unlock schedules for a vesting account.
    use crate::Vesting;
    pub fn available_for_withdrawal(vesting: &Vesting, current_ts: i64) -> u64 {
        std::cmp::min(outstanding_vested(vesting, current_ts), balance(vesting))
    }
    fn balance(vesting: &Vesting) -> u64 {
        vesting
            .outstanding
            .checked_sub(vesting.whitelist_owned)
            .unwrap()
    }
    fn outstanding_vested(vesting: &Vesting, current_ts: i64) -> u64 {
        total_vested(vesting, current_ts)
            .checked_sub(withdrawn_amount(vesting))
            .unwrap()
    }
    fn withdrawn_amount(vesting: &Vesting) -> u64 {
        vesting
            .start_balance
            .checked_sub(vesting.outstanding)
            .unwrap()
    }
    fn total_vested(vesting: &Vesting, current_ts: i64) -> u64 {
        if current_ts < vesting.start_ts {
            0
        } else if current_ts >= vesting.end_ts {
            vesting.start_balance
        } else {
            linear_unlock(vesting, current_ts).unwrap()
        }
    }
    fn linear_unlock(vesting: &Vesting, current_ts: i64) -> Option<u64> {
        let current_ts = current_ts as u64;
        let start_ts = vesting.start_ts as u64;
        let end_ts = vesting.end_ts as u64;
        let shifted_start_ts =
            start_ts.checked_sub(end_ts.checked_sub(start_ts)? % vesting.period_count)?;
        let reward_overflow = vesting.start_balance % vesting.period_count;
        let reward_per_period = (vesting.start_balance.checked_sub(reward_overflow)?)
            .checked_div(vesting.period_count)?;
        let current_period = {
            let period_secs =
                (end_ts.checked_sub(shifted_start_ts)?).checked_div(vesting.period_count)?;
            let current_period_count =
                (current_ts.checked_sub(shifted_start_ts)?).checked_div(period_secs)?;
            std::cmp::min(current_period_count, vesting.period_count)
        };
        if current_period == 0 {
            return Some(0);
        }
        current_period
            .checked_mul(reward_per_period)?
            .checked_add(reward_overflow)
    }
}
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        170u8, 199u8, 120u8, 40u8, 75u8, 101u8, 67u8, 255u8, 45u8, 118u8, 36u8, 227u8, 207u8,
        119u8, 92u8, 139u8, 220u8, 195u8, 12u8, 189u8, 177u8, 120u8, 233u8, 255u8, 194u8, 203u8,
        127u8, 225u8, 140u8, 234u8, 160u8, 31u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use lockup::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
#[cfg(not(feature = "no-entrypoint"))]
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if data.len() < 8 {
        return Err(anchor_lang::__private::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data).map_err(|e| {
        ::solana_program::log::sol_log(&e.to_string());
        e
    })
}
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct Lockup;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Lockup {
        #[inline]
        fn clone(&self) -> Lockup {
            match *self {
                Lockup => Lockup,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for Lockup {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Lockup::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(Lockup)
        }
    }
    impl anchor_lang::Id for Lockup {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [100, 40, 230, 207, 4, 208, 186, 204] => {
            __private::__global::whitelist_new(program_id, accounts, ix_data)
        }
        [200, 159, 194, 141, 100, 114, 107, 154] => {
            __private::__global::whitelist_add(program_id, accounts, ix_data)
        }
        [68, 105, 246, 32, 218, 241, 220, 192] => {
            __private::__global::whitelist_delete(program_id, accounts, ix_data)
        }
        [133, 250, 37, 21, 110, 163, 26, 121] => {
            __private::__global::set_authority(program_id, accounts, ix_data)
        }
        [135, 184, 171, 156, 197, 162, 246, 44] => {
            __private::__global::create_vesting(program_id, accounts, ix_data)
        }
        [183, 18, 70, 156, 148, 109, 161, 34] => {
            __private::__global::withdraw(program_id, accounts, ix_data)
        }
        [225, 1, 148, 72, 133, 32, 188, 143] => {
            __private::__global::whitelist_withdraw(program_id, accounts, ix_data)
        }
        [143, 172, 171, 182, 132, 5, 52, 70] => {
            __private::__global::whitelist_deposit(program_id, accounts, ix_data)
        }
        [176, 181, 67, 9, 120, 172, 226, 206] => {
            __private::__global::available_for_withdrawal(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::__private::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> ProgramResult {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::__private::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn whitelist_new(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WhitelistNew");
            let ix = instruction::WhitelistNew::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WhitelistNew { _bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WhitelistNew::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::whitelist_new(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn whitelist_add(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WhitelistAdd");
            let ix = instruction::WhitelistAdd::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WhitelistAdd { _bump, entry } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Auth::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::whitelist_add(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
                entry,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn whitelist_delete(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WhitelistDelete");
            let ix = instruction::WhitelistDelete::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WhitelistDelete { _bump, entry } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Auth::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::whitelist_delete(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
                entry,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn set_authority(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: SetAuthority");
            let ix = instruction::SetAuthority::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetAuthority {
                _bump,
                new_authority,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Auth::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::set_authority(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
                new_authority,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_vesting(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: CreateVesting");
            let ix = instruction::CreateVesting::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateVesting {
                beneficiary,
                deposit_amount,
                nonce,
                start_ts,
                end_ts,
                period_count,
                realizor,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CreateVesting::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::create_vesting(
                Context::new(program_id, &mut accounts, remaining_accounts),
                beneficiary,
                deposit_amount,
                nonce,
                start_ts,
                end_ts,
                period_count,
                realizor,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: Withdraw");
            let ix = instruction::Withdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Withdraw { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Withdraw::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::withdraw(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn whitelist_withdraw(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WhitelistWithdraw");
            let ix = instruction::WhitelistWithdraw::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WhitelistWithdraw {
                instruction_data,
                amount,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WhitelistWithdraw::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::whitelist_withdraw(
                Context::new(program_id, &mut accounts, remaining_accounts),
                instruction_data,
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn whitelist_deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WhitelistDeposit");
            let ix = instruction::WhitelistDeposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WhitelistDeposit { instruction_data } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WhitelistDeposit::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::whitelist_deposit(
                Context::new(program_id, &mut accounts, remaining_accounts),
                instruction_data,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn available_for_withdrawal(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: AvailableForWithdrawal");
            let ix = instruction::AvailableForWithdrawal::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::AvailableForWithdrawal = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                AvailableForWithdrawal::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            lockup::available_for_withdrawal(Context::new(
                program_id,
                &mut accounts,
                remaining_accounts,
            ))?;
            accounts.exit(program_id)
        }
    }
}
pub mod lockup {
    use super::*;
    pub const WHITELIST_SIZE: usize = 10;
    pub fn whitelist_new(ctx: Context<WhitelistNew>, _bump: u8) -> ProgramResult {
        let mut whitelist = ::alloc::vec::Vec::new();
        whitelist.resize(WHITELIST_SIZE, Default::default());
        ctx.accounts.lockup.authority = *ctx.accounts.authority.key;
        ctx.accounts.lockup.whitelist = whitelist;
        Ok(())
    }
    pub fn whitelist_add(ctx: Context<Auth>, _bump: u8, entry: WhitelistEntry) -> ProgramResult {
        whitelist_auth(&ctx)?;
        if ctx.accounts.lockup.whitelist.len() == WHITELIST_SIZE {
            return Err(ErrorCode::WhitelistFull.into());
        }
        if ctx.accounts.lockup.whitelist.contains(&entry) {
            return Err(ErrorCode::WhitelistEntryAlreadyExists.into());
        }
        ctx.accounts.lockup.whitelist.push(entry);
        Ok(())
    }
    pub fn whitelist_delete(ctx: Context<Auth>, _bump: u8, entry: WhitelistEntry) -> ProgramResult {
        whitelist_auth(&ctx)?;
        if !ctx.accounts.lockup.whitelist.contains(&entry) {
            return Err(ErrorCode::WhitelistEntryNotFound.into());
        }
        ctx.accounts.lockup.whitelist.retain(|e| e != &entry);
        Ok(())
    }
    pub fn set_authority(ctx: Context<Auth>, _bump: u8, new_authority: Pubkey) -> ProgramResult {
        whitelist_auth(&ctx)?;
        ctx.accounts.lockup.authority = new_authority;
        Ok(())
    }
    pub fn create_vesting(
        ctx: Context<CreateVesting>,
        beneficiary: Pubkey,
        deposit_amount: u64,
        nonce: u8,
        start_ts: i64,
        end_ts: i64,
        period_count: u64,
        realizor: Option<Realizor>,
    ) -> ProgramResult {
        CreateVesting::accounts(&ctx, nonce)?;
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
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        is_realized(&ctx)?;
        if amount
            > calculator::available_for_withdrawal(
                &ctx.accounts.vesting,
                ctx.accounts.clock.unix_timestamp,
            )
        {
            return Err(ErrorCode::InsufficientWithdrawalBalance.into());
        }
        let seeds = &[
            ctx.accounts.vesting.to_account_info().key.as_ref(),
            &[ctx.accounts.vesting.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::from(&*ctx.accounts).with_signer(signer);
        token::transfer(cpi_ctx, amount)?;
        let vesting = &mut ctx.accounts.vesting;
        vesting.outstanding -= amount;
        Ok(())
    }
    pub fn whitelist_withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WhitelistWithdraw<'info>>,
        instruction_data: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {
        let before_amount = ctx.accounts.transfer.vault.amount;
        whitelist_relay_cpi(
            &ctx.accounts.transfer,
            ctx.remaining_accounts,
            instruction_data,
        )?;
        let after_amount = ctx.accounts.transfer.vault.amount;
        let withdraw_amount = before_amount - after_amount;
        if withdraw_amount > amount {
            return Err(ErrorCode::WhitelistWithdrawLimit)?;
        }
        ctx.accounts.transfer.vesting.whitelist_owned += withdraw_amount;
        Ok(())
    }
    pub fn whitelist_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WhitelistDeposit<'info>>,
        instruction_data: Vec<u8>,
    ) -> ProgramResult {
        let before_amount = ctx.accounts.transfer.vault.amount;
        whitelist_relay_cpi(
            &ctx.accounts.transfer,
            ctx.remaining_accounts,
            instruction_data,
        )?;
        let after_amount = ctx.accounts.transfer.vault.amount;
        let deposit_amount = after_amount - before_amount;
        if deposit_amount <= 0 {
            return Err(ErrorCode::InsufficientWhitelistDepositAmount)?;
        }
        if deposit_amount > ctx.accounts.transfer.vesting.whitelist_owned {
            return Err(ErrorCode::WhitelistDepositOverflow)?;
        }
        ctx.accounts.transfer.vesting.whitelist_owned -= deposit_amount;
        Ok(())
    }
    pub fn available_for_withdrawal(ctx: Context<AvailableForWithdrawal>) -> ProgramResult {
        let available = calculator::available_for_withdrawal(
            &ctx.accounts.vesting,
            ctx.accounts.clock.unix_timestamp,
        );
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["{ \"result\": \"", "\" }"],
                &match (&available,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct WhitelistNew {
        pub _bump: u8,
    }
    impl borsh::ser::BorshSerialize for WhitelistNew
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WhitelistNew
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WhitelistNew {
        fn data(&self) -> Vec<u8> {
            let mut d = [100, 40, 230, 207, 4, 208, 186, 204].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WhitelistAdd {
        pub _bump: u8,
        pub entry: WhitelistEntry,
    }
    impl borsh::ser::BorshSerialize for WhitelistAdd
    where
        u8: borsh::ser::BorshSerialize,
        WhitelistEntry: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            borsh::BorshSerialize::serialize(&self.entry, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WhitelistAdd
    where
        u8: borsh::BorshDeserialize,
        WhitelistEntry: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
                entry: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WhitelistAdd {
        fn data(&self) -> Vec<u8> {
            let mut d = [200, 159, 194, 141, 100, 114, 107, 154].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WhitelistDelete {
        pub _bump: u8,
        pub entry: WhitelistEntry,
    }
    impl borsh::ser::BorshSerialize for WhitelistDelete
    where
        u8: borsh::ser::BorshSerialize,
        WhitelistEntry: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            borsh::BorshSerialize::serialize(&self.entry, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WhitelistDelete
    where
        u8: borsh::BorshDeserialize,
        WhitelistEntry: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
                entry: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WhitelistDelete {
        fn data(&self) -> Vec<u8> {
            let mut d = [68, 105, 246, 32, 218, 241, 220, 192].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SetAuthority {
        pub _bump: u8,
        pub new_authority: Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetAuthority
    where
        u8: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            borsh::BorshSerialize::serialize(&self.new_authority, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetAuthority
    where
        u8: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
                new_authority: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetAuthority {
        fn data(&self) -> Vec<u8> {
            let mut d = [133, 250, 37, 21, 110, 163, 26, 121].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateVesting {
        pub beneficiary: Pubkey,
        pub deposit_amount: u64,
        pub nonce: u8,
        pub start_ts: i64,
        pub end_ts: i64,
        pub period_count: u64,
        pub realizor: Option<Realizor>,
    }
    impl borsh::ser::BorshSerialize for CreateVesting
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Option<Realizor>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.deposit_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
            borsh::BorshSerialize::serialize(&self.end_ts, writer)?;
            borsh::BorshSerialize::serialize(&self.period_count, writer)?;
            borsh::BorshSerialize::serialize(&self.realizor, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateVesting
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Option<Realizor>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                beneficiary: borsh::BorshDeserialize::deserialize(buf)?,
                deposit_amount: borsh::BorshDeserialize::deserialize(buf)?,
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
                start_ts: borsh::BorshDeserialize::deserialize(buf)?,
                end_ts: borsh::BorshDeserialize::deserialize(buf)?,
                period_count: borsh::BorshDeserialize::deserialize(buf)?,
                realizor: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CreateVesting {
        fn data(&self) -> Vec<u8> {
            let mut d = [135, 184, 171, 156, 197, 162, 246, 44].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Withdraw {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for Withdraw
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Withdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [183, 18, 70, 156, 148, 109, 161, 34].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WhitelistWithdraw {
        pub instruction_data: Vec<u8>,
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for WhitelistWithdraw
    where
        Vec<u8>: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.instruction_data, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WhitelistWithdraw
    where
        Vec<u8>: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                instruction_data: borsh::BorshDeserialize::deserialize(buf)?,
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WhitelistWithdraw {
        fn data(&self) -> Vec<u8> {
            let mut d = [225, 1, 148, 72, 133, 32, 188, 143].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WhitelistDeposit {
        pub instruction_data: Vec<u8>,
    }
    impl borsh::ser::BorshSerialize for WhitelistDeposit
    where
        Vec<u8>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.instruction_data, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WhitelistDeposit
    where
        Vec<u8>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                instruction_data: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WhitelistDeposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [143, 172, 171, 182, 132, 5, 52, 70].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct AvailableForWithdrawal;
    impl borsh::ser::BorshSerialize for AvailableForWithdrawal {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AvailableForWithdrawal {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for AvailableForWithdrawal {
        fn data(&self) -> Vec<u8> {
            let mut d = [176, 181, 67, 9, 120, 172, 226, 206].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_whitelist_new::*;
    pub use crate::__client_accounts_available_for_withdrawal::*;
    pub use crate::__client_accounts_auth::*;
    pub use crate::__client_accounts_create_vesting::*;
    pub use crate::__client_accounts_withdraw::*;
    pub use crate::__client_accounts_whitelist_withdraw::*;
    pub use crate::__client_accounts_whitelist_deposit::*;
}
pub struct Lockup {
    /// The key with the ability to change the whitelist.
    pub authority: Pubkey,
    /// List of programs locked tokens can be sent to. These programs
    /// are completely trusted to maintain the locked property.
    pub whitelist: Vec<WhitelistEntry>,
}
impl borsh::ser::BorshSerialize for Lockup
where
    Pubkey: borsh::ser::BorshSerialize,
    Vec<WhitelistEntry>: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Lockup
where
    Pubkey: borsh::BorshDeserialize,
    Vec<WhitelistEntry>: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            authority: borsh::BorshDeserialize::deserialize(buf)?,
            whitelist: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Lockup {
    #[inline]
    fn clone(&self) -> Lockup {
        match *self {
            Lockup {
                authority: ref __self_0_0,
                whitelist: ref __self_0_1,
            } => Lockup {
                authority: ::core::clone::Clone::clone(&(*__self_0_0)),
                whitelist: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Lockup {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[1, 45, 32, 32, 57, 81, 88, 67])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Lockup {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [1, 45, 32, 32, 57, 81, 88, 67].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[1, 45, 32, 32, 57, 81, 88, 67] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Lockup {
    fn discriminator() -> [u8; 8] {
        [1, 45, 32, 32, 57, 81, 88, 67]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Lockup {
    fn owner() -> Pubkey {
        crate::ID
    }
}
# [instruction (lockup_bump : u8)]
pub struct WhitelistNew<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    # [account (init , seeds = [b"lockup" . as_ref ()] , bump = lockup_bump , payer = authority , space = 1000)]
    pub lockup: Box<Account<'info, Lockup>>,
    pub system_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for WhitelistNew<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            lockup_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.lockup_bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    lockup_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { lockup_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let lockup = &accounts[0];
        *accounts = &accounts[1..];
        let system_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let lockup = {
            let actual_field = lockup.to_account_info();
            let actual_owner = actual_field.owner;
            let space = 1000;
            if !false || actual_owner == &anchor_lang::solana_program::system_program::ID {
                let payer = authority.to_account_info();
                let __current_lamports = lockup.to_account_info().lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::create_account(
                            payer.to_account_info().key,
                            lockup.to_account_info().key,
                            lamports,
                            space as u64,
                            program_id,
                        ),
                        &[
                            payer.to_account_info(),
                            lockup.to_account_info(),
                            system_program.to_account_info(),
                        ],
                        &[&[b"lockup".as_ref(), &[lockup_bump][..]][..]],
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        anchor_lang::solana_program::program::invoke(
                            &anchor_lang::solana_program::system_instruction::transfer(
                                payer.to_account_info().key,
                                lockup.to_account_info().key,
                                required_lamports,
                            ),
                            &[
                                payer.to_account_info(),
                                lockup.to_account_info(),
                                system_program.to_account_info(),
                            ],
                        )?;
                    }
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::allocate(
                            lockup.to_account_info().key,
                            space as u64,
                        ),
                        &[lockup.to_account_info(), system_program.to_account_info()],
                        &[&[b"lockup".as_ref(), &[lockup_bump][..]][..]],
                    )?;
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::assign(
                            lockup.to_account_info().key,
                            program_id,
                        ),
                        &[lockup.to_account_info(), system_program.to_account_info()],
                        &[&[b"lockup".as_ref(), &[lockup_bump][..]][..]],
                    )?;
                }
            }
            let pa: Box<anchor_lang::Account<Lockup>> =
                Box::new(anchor_lang::Account::try_from_unchecked(&lockup)?);
            if !(!false || actual_owner == &anchor_lang::solana_program::system_program::ID) {
                if space != actual_field.data_len() {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSpace.into());
                }
                if actual_owner != program_id {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                let expected_key = anchor_lang::prelude::Pubkey::create_program_address(
                    &[b"lockup".as_ref(), &[lockup_bump][..]][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if expected_key != lockup.key() {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
            }
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[b"lockup".as_ref()],
                program_id,
            );
        if lockup.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != lockup_bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !lockup.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            lockup.to_account_info().lamports(),
            lockup.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(WhitelistNew {
            authority,
            lockup,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistNew<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.lockup.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for WhitelistNew<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas.extend(self.lockup.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for WhitelistNew<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.lockup, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_whitelist_new {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct WhitelistNew {
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub lockup: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for WhitelistNew
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.lockup, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for WhitelistNew {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.lockup,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.system_program,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_whitelist_new {
    use super::*;
    pub struct WhitelistNew<'info> {
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub lockup: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WhitelistNew<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.lockup),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.system_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistNew<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.lockup));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
# [instruction (lockup_bump : u8)]
pub struct Auth<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    # [account (mut , seeds = [b"lockup" . as_ref ()] , bump = lockup_bump)]
    pub lockup: Box<Account<'info, Lockup>>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Auth<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            lockup_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.lockup_bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    lockup_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { lockup_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let lockup: Box<anchor_lang::Account<Lockup>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[b"lockup".as_ref(), &[lockup_bump][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if lockup.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !lockup.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(Auth { authority, lockup })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Auth<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.lockup.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Auth<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas.extend(self.lockup.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Auth<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.lockup, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_auth {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Auth {
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub lockup: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Auth
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.lockup, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Auth {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.lockup,
                false,
            ));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_auth {
    use super::*;
    pub struct Auth<'info> {
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub lockup: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Auth<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.lockup),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Auth<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.lockup));
            account_infos
        }
    }
}
pub struct CreateVesting<'info> {
    #[account(zero)]
    pub vesting: Box<Account<'info, Vesting>>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    #[account(signer)]
    pub depositor_authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CreateVesting<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let vesting = &accounts[0];
        *accounts = &accounts[1..];
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let vesting: Box<anchor_lang::Account<Vesting>> = {
            let mut __data: &[u8] = &vesting.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(&vesting)?)
        };
        if !vesting.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            vesting.to_account_info().lamports(),
            vesting.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !depositor.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !depositor_authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(CreateVesting {
            vesting,
            vault,
            depositor,
            depositor_authority,
            token_program,
            rent,
            clock,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVesting<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.depositor.to_account_infos());
        account_infos.extend(self.depositor_authority.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateVesting<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.depositor.to_account_metas(None));
        account_metas.extend(self.depositor_authority.to_account_metas(Some(true)));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateVesting<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vesting, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.depositor, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create_vesting {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct CreateVesting {
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor_authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateVesting
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateVesting {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vesting,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.depositor,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.depositor_authority,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create_vesting {
    use super::*;
    pub struct CreateVesting<'info> {
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateVesting<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vesting),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.depositor),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.depositor_authority),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVesting<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.depositor_authority,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
impl<'info> CreateVesting<'info> {
    fn accounts(ctx: &Context<CreateVesting>, nonce: u8) -> ProgramResult {
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
pub struct Withdraw<'info> {
    # [account (mut , has_one = beneficiary , has_one = vault)]
    vesting: Box<Account<'info, Vesting>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut)]
    vault: Account<'info, TokenAccount>,
    # [account (seeds = [vesting . to_account_info () . key . as_ref ()] , bump = vesting . nonce)]
    vesting_signer: AccountInfo<'info>,
    #[account(mut)]
    token: Account<'info, TokenAccount>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Withdraw<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let vesting: Box<anchor_lang::Account<Vesting>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !vesting.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &vesting.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &vesting.vault != vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[vesting.to_account_info().key.as_ref(), &[vesting.nonce][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if vesting_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !token.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(Withdraw {
            vesting,
            beneficiary,
            vault,
            vesting_signer,
            token,
            token_program,
            clock,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vesting_signer.to_account_infos());
        account_infos.extend(self.token.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vesting_signer.to_account_metas(None));
        account_metas.extend(self.token.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Withdraw<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vesting, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.token, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_withdraw {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Withdraw {
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Withdraw
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Withdraw {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vesting,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vesting_signer,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.token, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_withdraw {
    use super::*;
    pub struct Withdraw<'info> {
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vesting),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vesting_signer),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.token),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Withdraw<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vesting_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.token));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
pub struct WhitelistWithdraw<'info> {
    transfer: WhitelistTransfer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for WhitelistWithdraw<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let transfer: WhitelistTransfer<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        Ok(WhitelistWithdraw { transfer })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistWithdraw<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.transfer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for WhitelistWithdraw<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.transfer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for WhitelistWithdraw<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.transfer, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_whitelist_withdraw {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_whitelist_transfer::WhitelistTransfer;
    pub struct WhitelistWithdraw {
        pub transfer: __client_accounts_whitelist_transfer::WhitelistTransfer,
    }
    impl borsh::ser::BorshSerialize for WhitelistWithdraw
    where
        __client_accounts_whitelist_transfer::WhitelistTransfer: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.transfer, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for WhitelistWithdraw {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.transfer.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_whitelist_withdraw {
    use super::*;
    pub use __cpi_client_accounts_whitelist_transfer::WhitelistTransfer;
    pub struct WhitelistWithdraw<'info> {
        pub transfer: __cpi_client_accounts_whitelist_transfer::WhitelistTransfer<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WhitelistWithdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.transfer.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistWithdraw<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.transfer,
            ));
            account_infos
        }
    }
}
pub struct WhitelistDeposit<'info> {
    transfer: WhitelistTransfer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for WhitelistDeposit<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let transfer: WhitelistTransfer<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        Ok(WhitelistDeposit { transfer })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistDeposit<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.transfer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for WhitelistDeposit<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.transfer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for WhitelistDeposit<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.transfer, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_whitelist_deposit {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_whitelist_transfer::WhitelistTransfer;
    pub struct WhitelistDeposit {
        pub transfer: __client_accounts_whitelist_transfer::WhitelistTransfer,
    }
    impl borsh::ser::BorshSerialize for WhitelistDeposit
    where
        __client_accounts_whitelist_transfer::WhitelistTransfer: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.transfer, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for WhitelistDeposit {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.transfer.to_account_metas(None));
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_whitelist_deposit {
    use super::*;
    pub use __cpi_client_accounts_whitelist_transfer::WhitelistTransfer;
    pub struct WhitelistDeposit<'info> {
        pub transfer: __cpi_client_accounts_whitelist_transfer::WhitelistTransfer<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WhitelistDeposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.transfer.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistDeposit<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.transfer,
            ));
            account_infos
        }
    }
}
# [instruction (lockup_bump : u8)]
pub struct WhitelistTransfer<'info> {
    # [account (seeds = [b"lockup" . as_ref ()] , bump = lockup_bump)]
    lockup: Box<Account<'info, Lockup>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    whitelisted_program: AccountInfo<'info>,
    # [account (mut , has_one = beneficiary , has_one = vault)]
    vesting: Box<Account<'info, Vesting>>,
    #[account(mut, "&vault.owner == vesting_signer.key")]
    vault: Account<'info, TokenAccount>,
    # [account (seeds = [vesting . to_account_info () . key . as_ref ()] , bump = vesting . nonce)]
    vesting_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    #[account(mut)]
    whitelisted_program_vault: AccountInfo<'info>,
    whitelisted_program_vault_authority: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for WhitelistTransfer<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let mut ix_data = ix_data;
        struct __Args {
            lockup_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.lockup_bump, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    lockup_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { lockup_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let lockup: Box<anchor_lang::Account<Lockup>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let whitelisted_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting: Box<anchor_lang::Account<Vesting>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let whitelisted_program_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let whitelisted_program_vault_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __program_signer = Pubkey::create_program_address(
            &[b"lockup".as_ref(), &[lockup_bump][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if lockup.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !vesting.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &vesting.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &vesting.vault != vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(&vault.owner == vesting_signer.key) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[vesting.to_account_info().key.as_ref(), &[vesting.nonce][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if vesting_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !whitelisted_program_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(WhitelistTransfer {
            lockup,
            beneficiary,
            whitelisted_program,
            vesting,
            vault,
            vesting_signer,
            token_program,
            whitelisted_program_vault,
            whitelisted_program_vault_authority,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistTransfer<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.lockup.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.whitelisted_program.to_account_infos());
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vesting_signer.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.whitelisted_program_vault.to_account_infos());
        account_infos.extend(self.whitelisted_program_vault_authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for WhitelistTransfer<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.lockup.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.whitelisted_program.to_account_metas(None));
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vesting_signer.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.whitelisted_program_vault.to_account_metas(None));
        account_metas.extend(
            self.whitelisted_program_vault_authority
                .to_account_metas(None),
        );
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for WhitelistTransfer<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vesting, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.whitelisted_program_vault, program_id)?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_whitelist_transfer {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct WhitelistTransfer {
        pub lockup: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub whitelisted_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub whitelisted_program_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub whitelisted_program_vault_authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for WhitelistTransfer
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lockup, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.whitelisted_program, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.whitelisted_program_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.whitelisted_program_vault_authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for WhitelistTransfer {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.lockup,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.whitelisted_program,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vesting,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vesting_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.whitelisted_program_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.whitelisted_program_vault_authority,
                    false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_whitelist_transfer {
    use super::*;
    pub struct WhitelistTransfer<'info> {
        pub lockup: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub whitelisted_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub whitelisted_program_vault:
            anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub whitelisted_program_vault_authority:
            anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WhitelistTransfer<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.lockup),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.whitelisted_program),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vesting),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vesting_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.whitelisted_program_vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.whitelisted_program_vault_authority),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WhitelistTransfer<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.lockup));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.whitelisted_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vesting_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.whitelisted_program_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.whitelisted_program_vault_authority,
            ));
            account_infos
        }
    }
}
pub struct AvailableForWithdrawal<'info> {
    vesting: Box<Account<'info, Vesting>>,
    clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for AvailableForWithdrawal<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let vesting: Box<anchor_lang::Account<Vesting>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        Ok(AvailableForWithdrawal { vesting, clock })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for AvailableForWithdrawal<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for AvailableForWithdrawal<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for AvailableForWithdrawal<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_available_for_withdrawal {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct AvailableForWithdrawal {
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for AvailableForWithdrawal
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for AvailableForWithdrawal {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vesting,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// `cpi::accounts` module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_available_for_withdrawal {
    use super::*;
    pub struct AvailableForWithdrawal<'info> {
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for AvailableForWithdrawal<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vesting),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for AvailableForWithdrawal<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
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
impl borsh::ser::BorshSerialize for Vesting
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    Option<Realizor>: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
        borsh::BorshSerialize::serialize(&self.mint, writer)?;
        borsh::BorshSerialize::serialize(&self.vault, writer)?;
        borsh::BorshSerialize::serialize(&self.grantor, writer)?;
        borsh::BorshSerialize::serialize(&self.outstanding, writer)?;
        borsh::BorshSerialize::serialize(&self.start_balance, writer)?;
        borsh::BorshSerialize::serialize(&self.created_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.end_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.period_count, writer)?;
        borsh::BorshSerialize::serialize(&self.whitelist_owned, writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, writer)?;
        borsh::BorshSerialize::serialize(&self.realizor, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Vesting
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    Option<Realizor>: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            beneficiary: borsh::BorshDeserialize::deserialize(buf)?,
            mint: borsh::BorshDeserialize::deserialize(buf)?,
            vault: borsh::BorshDeserialize::deserialize(buf)?,
            grantor: borsh::BorshDeserialize::deserialize(buf)?,
            outstanding: borsh::BorshDeserialize::deserialize(buf)?,
            start_balance: borsh::BorshDeserialize::deserialize(buf)?,
            created_ts: borsh::BorshDeserialize::deserialize(buf)?,
            start_ts: borsh::BorshDeserialize::deserialize(buf)?,
            end_ts: borsh::BorshDeserialize::deserialize(buf)?,
            period_count: borsh::BorshDeserialize::deserialize(buf)?,
            whitelist_owned: borsh::BorshDeserialize::deserialize(buf)?,
            nonce: borsh::BorshDeserialize::deserialize(buf)?,
            realizor: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Vesting {
    #[inline]
    fn clone(&self) -> Vesting {
        match *self {
            Vesting {
                beneficiary: ref __self_0_0,
                mint: ref __self_0_1,
                vault: ref __self_0_2,
                grantor: ref __self_0_3,
                outstanding: ref __self_0_4,
                start_balance: ref __self_0_5,
                created_ts: ref __self_0_6,
                start_ts: ref __self_0_7,
                end_ts: ref __self_0_8,
                period_count: ref __self_0_9,
                whitelist_owned: ref __self_0_10,
                nonce: ref __self_0_11,
                realizor: ref __self_0_12,
            } => Vesting {
                beneficiary: ::core::clone::Clone::clone(&(*__self_0_0)),
                mint: ::core::clone::Clone::clone(&(*__self_0_1)),
                vault: ::core::clone::Clone::clone(&(*__self_0_2)),
                grantor: ::core::clone::Clone::clone(&(*__self_0_3)),
                outstanding: ::core::clone::Clone::clone(&(*__self_0_4)),
                start_balance: ::core::clone::Clone::clone(&(*__self_0_5)),
                created_ts: ::core::clone::Clone::clone(&(*__self_0_6)),
                start_ts: ::core::clone::Clone::clone(&(*__self_0_7)),
                end_ts: ::core::clone::Clone::clone(&(*__self_0_8)),
                period_count: ::core::clone::Clone::clone(&(*__self_0_9)),
                whitelist_owned: ::core::clone::Clone::clone(&(*__self_0_10)),
                nonce: ::core::clone::Clone::clone(&(*__self_0_11)),
                realizor: ::core::clone::Clone::clone(&(*__self_0_12)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Vesting {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[100, 149, 66, 138, 95, 200, 128, 241])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Vesting {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [100, 149, 66, 138, 95, 200, 128, 241].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[100, 149, 66, 138, 95, 200, 128, 241] != given_disc {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Vesting {
    fn discriminator() -> [u8; 8] {
        [100, 149, 66, 138, 95, 200, 128, 241]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Vesting {
    fn owner() -> Pubkey {
        crate::ID
    }
}
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
impl borsh::ser::BorshSerialize for Realizor
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.program, writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Realizor
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            program: borsh::BorshDeserialize::deserialize(buf)?,
            metadata: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Realizor {
    #[inline]
    fn clone(&self) -> Realizor {
        match *self {
            Realizor {
                program: ref __self_0_0,
                metadata: ref __self_0_1,
            } => Realizor {
                program: ::core::clone::Clone::clone(&(*__self_0_0)),
                metadata: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Realizor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Realizor {
                program: ref __self_0_0,
                metadata: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Realizor");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "program",
                    &&(*__self_0_0),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "metadata",
                    &&(*__self_0_1),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
pub struct WhitelistEntry {
    pub program_id: Pubkey,
}
impl borsh::ser::BorshSerialize for WhitelistEntry
where
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.program_id, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for WhitelistEntry
where
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            program_id: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
impl ::core::marker::StructuralPartialEq for WhitelistEntry {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for WhitelistEntry {
    #[inline]
    fn eq(&self, other: &WhitelistEntry) -> bool {
        match *other {
            WhitelistEntry {
                program_id: ref __self_1_0,
            } => match *self {
                WhitelistEntry {
                    program_id: ref __self_0_0,
                } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &WhitelistEntry) -> bool {
        match *other {
            WhitelistEntry {
                program_id: ref __self_1_0,
            } => match *self {
                WhitelistEntry {
                    program_id: ref __self_0_0,
                } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for WhitelistEntry {
    #[inline]
    fn default() -> WhitelistEntry {
        WhitelistEntry {
            program_id: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for WhitelistEntry {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for WhitelistEntry {
    #[inline]
    fn clone(&self) -> WhitelistEntry {
        {
            let _: ::core::clone::AssertParamIsClone<Pubkey>;
            *self
        }
    }
}
/// Anchor generated Result to be used as the return type for the
/// program.
pub type Result<T> = std::result::Result<T, Error>;
/// Anchor generated error allowing one to easily return a
/// `ProgramError` or a custom, user defined error code by utilizing
/// its `From` implementation.
#[doc(hidden)]
pub enum Error {
    #[error(transparent)]
    ProgramError(#[from] anchor_lang::solana_program::program_error::ProgramError),
    #[error(transparent)]
    ErrorCode(#[from] ErrorCode),
}
#[allow(unused_qualifications)]
impl std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        use thiserror::private::AsDynError;
        #[allow(deprecated)]
        match self {
            Error::ProgramError { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
            Error::ErrorCode { 0: transparent } => {
                std::error::Error::source(transparent.as_dyn_error())
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::fmt::Display for Error {
    fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
        match self {
            Error::ProgramError(_0) => std::fmt::Display::fmt(_0, __formatter),
            Error::ErrorCode(_0) => std::fmt::Display::fmt(_0, __formatter),
        }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<anchor_lang::solana_program::program_error::ProgramError> for Error {
    #[allow(deprecated)]
    fn from(source: anchor_lang::solana_program::program_error::ProgramError) -> Self {
        Error::ProgramError { 0: source }
    }
}
#[allow(unused_qualifications)]
impl std::convert::From<ErrorCode> for Error {
    #[allow(deprecated)]
    fn from(source: ErrorCode) -> Self {
        Error::ErrorCode { 0: source }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Error::ProgramError(ref __self_0),) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "ProgramError");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Error::ErrorCode(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "ErrorCode");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[repr(u32)]
pub enum ErrorCode {
    InvalidTimestamp,
    InvalidPeriod,
    InvalidDepositAmount,
    InvalidWhitelistEntry,
    InvalidProgramAddress,
    InvalidVaultOwner,
    InvalidVaultAmount,
    InsufficientWithdrawalBalance,
    WhitelistFull,
    WhitelistEntryAlreadyExists,
    InsufficientWhitelistDepositAmount,
    WhitelistDepositOverflow,
    WhitelistWithdrawLimit,
    WhitelistEntryNotFound,
    Unauthorized,
    UnableToWithdrawWhileStaked,
    InvalidLockRealizor,
    UnrealizedVesting,
    InvalidSchedule,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::InvalidTimestamp,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidTimestamp")
            }
            (&ErrorCode::InvalidPeriod,) => ::core::fmt::Formatter::write_str(f, "InvalidPeriod"),
            (&ErrorCode::InvalidDepositAmount,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidDepositAmount")
            }
            (&ErrorCode::InvalidWhitelistEntry,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidWhitelistEntry")
            }
            (&ErrorCode::InvalidProgramAddress,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidProgramAddress")
            }
            (&ErrorCode::InvalidVaultOwner,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVaultOwner")
            }
            (&ErrorCode::InvalidVaultAmount,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVaultAmount")
            }
            (&ErrorCode::InsufficientWithdrawalBalance,) => {
                ::core::fmt::Formatter::write_str(f, "InsufficientWithdrawalBalance")
            }
            (&ErrorCode::WhitelistFull,) => ::core::fmt::Formatter::write_str(f, "WhitelistFull"),
            (&ErrorCode::WhitelistEntryAlreadyExists,) => {
                ::core::fmt::Formatter::write_str(f, "WhitelistEntryAlreadyExists")
            }
            (&ErrorCode::InsufficientWhitelistDepositAmount,) => {
                ::core::fmt::Formatter::write_str(f, "InsufficientWhitelistDepositAmount")
            }
            (&ErrorCode::WhitelistDepositOverflow,) => {
                ::core::fmt::Formatter::write_str(f, "WhitelistDepositOverflow")
            }
            (&ErrorCode::WhitelistWithdrawLimit,) => {
                ::core::fmt::Formatter::write_str(f, "WhitelistWithdrawLimit")
            }
            (&ErrorCode::WhitelistEntryNotFound,) => {
                ::core::fmt::Formatter::write_str(f, "WhitelistEntryNotFound")
            }
            (&ErrorCode::Unauthorized,) => ::core::fmt::Formatter::write_str(f, "Unauthorized"),
            (&ErrorCode::UnableToWithdrawWhileStaked,) => {
                ::core::fmt::Formatter::write_str(f, "UnableToWithdrawWhileStaked")
            }
            (&ErrorCode::InvalidLockRealizor,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidLockRealizor")
            }
            (&ErrorCode::UnrealizedVesting,) => {
                ::core::fmt::Formatter::write_str(f, "UnrealizedVesting")
            }
            (&ErrorCode::InvalidSchedule,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidSchedule")
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorCode {
    #[inline]
    fn clone(&self) -> ErrorCode {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for ErrorCode {}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorCode::InvalidTimestamp => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Vesting end must be greater than the current unix timestamp."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidPeriod => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The number of vesting periods must be greater than zero."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidDepositAmount => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The vesting deposit amount must be greater than zero."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidWhitelistEntry => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The Whitelist entry is not a valid program address."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidProgramAddress => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid program address. Did you provide the correct nonce?"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVaultOwner => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid vault owner."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVaultAmount => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Vault amount must be zero."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InsufficientWithdrawalBalance => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Insufficient withdrawal balance."],
                    &match () {
                        () => [],
                    },
                ))
            }
            ErrorCode::WhitelistFull => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Whitelist is full"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::WhitelistEntryAlreadyExists => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Whitelist entry already exists"],
                    &match () {
                        () => [],
                    },
                ))
            }
            ErrorCode::InsufficientWhitelistDepositAmount => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Balance must go up when performing a whitelist deposit"],
                    &match () {
                        () => [],
                    },
                ))
            }
            ErrorCode::WhitelistDepositOverflow => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Cannot deposit more than withdrawn"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::WhitelistWithdrawLimit => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Tried to withdraw over the specified limit"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::WhitelistEntryNotFound => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Whitelist entry not found."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::Unauthorized => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["You do not have sufficient permissions to perform this action."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::UnableToWithdrawWhileStaked => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(
                    &["You are unable to realize projected rewards until unstaking."],
                    &match () {
                        () => [],
                    },
                ))
            }
            ErrorCode::InvalidLockRealizor => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The given lock realizor doesn\'t match the vesting account."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::UnrealizedVesting => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["You have not realized this vesting account."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidSchedule => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid vesting schedule given."],
                &match () {
                    () => [],
                },
            )),
        }
    }
}
impl std::error::Error for ErrorCode {}
impl std::convert::From<Error> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: Error) -> anchor_lang::solana_program::program_error::ProgramError {
        match e {
            Error::ProgramError(e) => e,
            Error::ErrorCode(c) => {
                anchor_lang::solana_program::program_error::ProgramError::Custom(
                    c as u32 + anchor_lang::__private::ERROR_CODE_OFFSET,
                )
            }
        }
    }
}
impl std::convert::From<ErrorCode> for anchor_lang::solana_program::program_error::ProgramError {
    fn from(e: ErrorCode) -> anchor_lang::solana_program::program_error::ProgramError {
        let err: Error = e.into();
        err.into()
    }
}
impl<'a, 'b, 'c, 'info> From<&mut CreateVesting<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut CreateVesting<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
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
pub fn whitelist_relay_cpi<'info>(
    transfer: &WhitelistTransfer<'info>,
    remaining_accounts: &[AccountInfo<'info>],
    instruction_data: Vec<u8>,
) -> ProgramResult {
    is_whitelisted(transfer)?;
    let mut meta_accounts = <[_]>::into_vec(box [
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
    ]);
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
pub fn is_whitelisted<'info>(transfer: &WhitelistTransfer<'info>) -> ProgramResult {
    if !transfer.lockup.whitelist.contains(&WhitelistEntry {
        program_id: *transfer.whitelisted_program.key,
    }) {
        return Err(ErrorCode::WhitelistEntryNotFound.into());
    }
    Ok(())
}
fn whitelist_auth(ctx: &Context<Auth>) -> ProgramResult {
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
fn is_realized(ctx: &Context<Withdraw>) -> ProgramResult {
    if let Some(realizor) = &ctx.accounts.vesting.realizor {
        let cpi_program = {
            let p = ctx.remaining_accounts[0].clone();
            if p.key != &realizor.program {
                return Err(ErrorCode::InvalidLockRealizor.into());
            }
            p
        };
        let cpi_accounts = ctx.remaining_accounts.to_vec()[1..].to_vec();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let vesting_account = (*ctx.accounts.vesting).clone();
        let vesting = Vesting {
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
            realizor: vesting_account.realizor.clone(),
        };
        realize_lock::is_realized(cpi_ctx, vesting).map_err(|_| ErrorCode::UnrealizedVesting)?;
    }
    Ok(())
}
/// RealizeLock defines the interface an external program must implement if
/// they want to define a "realization condition" on a locked vesting account.
/// This condition must be satisfied *even if a vesting schedule has
/// completed*. Otherwise the user can never earn the locked funds. For example,
/// in the case of the staking program, one cannot received a locked reward
/// until one has completely unstaked.
pub trait RealizeLock<'info, T: Accounts<'info>> {
    fn is_realized(ctx: Context<T>, v: Vesting) -> ProgramResult;
}
/// Anchor generated module for invoking programs implementing an
/// `#[interface]` via CPI.
mod realize_lock {
    use super::*;
    pub fn is_realized<
        'a,
        'b,
        'c,
        'info,
        T: anchor_lang::Accounts<'info>
            + anchor_lang::ToAccountMetas
            + anchor_lang::ToAccountInfos<'info>,
    >(
        ctx: anchor_lang::CpiContext<'a, 'b, 'c, 'info, T>,
        v: Vesting,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        use anchor_lang::prelude::borsh;
        struct Args {
            v: Vesting,
        }
        impl borsh::ser::BorshSerialize for Args
        where
            Vesting: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.v, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Args
        where
            Vesting: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    v: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let ix = {
            let ix = Args { v };
            let mut ix_data = anchor_lang::AnchorSerialize::try_to_vec(&ix)
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotSerialize)?;
            let mut data = [234, 14, 207, 222, 54, 130, 56, 217].to_vec();
            data.append(&mut ix_data);
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: *ctx.program.key,
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        acc_infos.push(ctx.program.clone());
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
    }
}
