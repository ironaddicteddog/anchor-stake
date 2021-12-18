#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use lockup::{CreateVesting, RealizeLock, Realizor, Vesting};
use std::convert::Into;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        62u8, 204u8, 136u8, 136u8, 88u8, 35u8, 66u8, 206u8, 202u8, 255u8, 142u8, 172u8, 142u8,
        246u8, 135u8, 207u8, 16u8, 193u8, 39u8, 235u8, 163u8, 225u8, 250u8, 6u8, 132u8, 136u8,
        154u8, 128u8, 203u8, 77u8, 72u8, 176u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use registry::*;
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
    pub struct Registry;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Registry {
        #[inline]
        fn clone(&self) -> Registry {
            match *self {
                Registry => Registry,
            }
        }
    }
    impl anchor_lang::AccountDeserialize for Registry {
        fn try_deserialize(
            buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Registry::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(
            _buf: &mut &[u8],
        ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError>
        {
            Ok(Registry)
        }
    }
    impl anchor_lang::Id for Registry {
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
        [237, 187, 50, 70, 74, 26, 144, 230] => {
            __private::__global::new_registry(program_id, accounts, ix_data)
        }
        [223, 79, 250, 198, 20, 31, 79, 117] => {
            __private::__global::set_lockup_program(program_id, accounts, ix_data)
        }
        [175, 175, 109, 31, 13, 152, 155, 237] => {
            __private::__global::initialize(program_id, accounts, ix_data)
        }
        [116, 34, 168, 129, 238, 12, 90, 56] => {
            __private::__global::update_registrar(program_id, accounts, ix_data)
        }
        [49, 46, 45, 241, 122, 143, 136, 73] => {
            __private::__global::create_member(program_id, accounts, ix_data)
        }
        [106, 188, 198, 184, 221, 120, 117, 103] => {
            __private::__global::update_member_balances(program_id, accounts, ix_data)
        }
        [255, 3, 165, 86, 185, 28, 102, 93] => {
            __private::__global::update_member_balances_lock(program_id, accounts, ix_data)
        }
        [46, 229, 3, 194, 47, 105, 211, 28] => {
            __private::__global::update_member(program_id, accounts, ix_data)
        }
        [242, 35, 198, 137, 82, 225, 242, 182] => {
            __private::__global::deposit(program_id, accounts, ix_data)
        }
        [88, 91, 135, 52, 79, 190, 164, 141] => {
            __private::__global::deposit_locked(program_id, accounts, ix_data)
        }
        [206, 176, 202, 18, 200, 209, 179, 108] => {
            __private::__global::stake(program_id, accounts, ix_data)
        }
        [200, 243, 106, 111, 170, 72, 31, 117] => {
            __private::__global::start_unstake(program_id, accounts, ix_data)
        }
        [44, 65, 159, 108, 149, 89, 27, 203] => {
            __private::__global::end_unstake(program_id, accounts, ix_data)
        }
        [183, 18, 70, 156, 148, 109, 161, 34] => {
            __private::__global::withdraw(program_id, accounts, ix_data)
        }
        [96, 224, 88, 102, 223, 189, 8, 228] => {
            __private::__global::withdraw_locked(program_id, accounts, ix_data)
        }
        [170, 77, 234, 93, 202, 35, 96, 101] => {
            __private::__global::drop_reward(program_id, accounts, ix_data)
        }
        [149, 95, 181, 242, 94, 90, 158, 162] => {
            __private::__global::claim_reward(program_id, accounts, ix_data)
        }
        [231, 6, 114, 60, 66, 89, 22, 135] => {
            __private::__global::claim_reward_locked(program_id, accounts, ix_data)
        }
        [59, 250, 57, 193, 205, 47, 0, 122] => {
            __private::__global::expire_reward(program_id, accounts, ix_data)
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
        pub fn new_registry(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: NewRegistry");
            let ix = instruction::NewRegistry::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::NewRegistry { _bump } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                NewRegistry::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::new_registry(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn set_lockup_program(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: SetLockupProgram");
            let ix = instruction::SetLockupProgram::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetLockupProgram { lockup_program } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                SetLockupProgram::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::set_lockup_program(
                Context::new(program_id, &mut accounts, remaining_accounts),
                lockup_program,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn initialize(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: Initialize");
            let ix = instruction::Initialize::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Initialize {
                mint,
                authority,
                nonce,
                withdrawal_timelock,
                stake_rate,
                reward_q_len,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Initialize::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::initialize(
                Context::new(program_id, &mut accounts, remaining_accounts),
                mint,
                authority,
                nonce,
                withdrawal_timelock,
                stake_rate,
                reward_q_len,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_registrar(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: UpdateRegistrar");
            let ix = instruction::UpdateRegistrar::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateRegistrar {
                new_authority,
                withdrawal_timelock,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                UpdateRegistrar::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::update_registrar(
                Context::new(program_id, &mut accounts, remaining_accounts),
                new_authority,
                withdrawal_timelock,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_member(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: CreateMember");
            let ix = instruction::CreateMember::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateMember { nonce } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                CreateMember::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::create_member(
                Context::new(program_id, &mut accounts, remaining_accounts),
                nonce,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_member_balances(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: UpdateMemberBalances");
            let ix = instruction::UpdateMemberBalances::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateMemberBalances { nonce } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                UpdateMemberBalances::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::update_member_balances(
                Context::new(program_id, &mut accounts, remaining_accounts),
                nonce,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_member_balances_lock(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: UpdateMemberBalancesLock");
            let ix = instruction::UpdateMemberBalancesLock::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateMemberBalancesLock { nonce } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = UpdateMemberBalancesLock::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
            )?;
            registry::update_member_balances_lock(
                Context::new(program_id, &mut accounts, remaining_accounts),
                nonce,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_member(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: UpdateMember");
            let ix = instruction::UpdateMember::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateMember { metadata } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                UpdateMember::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::update_member(
                Context::new(program_id, &mut accounts, remaining_accounts),
                metadata,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: Deposit");
            let ix = instruction::Deposit::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Deposit { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Deposit::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::deposit(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn deposit_locked(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: DepositLocked");
            let ix = instruction::DepositLocked::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DepositLocked { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DepositLocked::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::deposit_locked(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn stake(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: Stake");
            let ix = instruction::Stake::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Stake { spt_amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Stake::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::stake(
                Context::new(program_id, &mut accounts, remaining_accounts),
                spt_amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn start_unstake(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: StartUnstake");
            let ix = instruction::StartUnstake::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::StartUnstake { spt_amount, locked } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                StartUnstake::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::start_unstake(
                Context::new(program_id, &mut accounts, remaining_accounts),
                spt_amount,
                locked,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn end_unstake(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: EndUnstake");
            let ix = instruction::EndUnstake::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::EndUnstake = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                EndUnstake::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::end_unstake(Context::new(program_id, &mut accounts, remaining_accounts))?;
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
            registry::withdraw(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw_locked(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: WithdrawLocked");
            let ix = instruction::WithdrawLocked::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WithdrawLocked { amount } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                WithdrawLocked::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::withdraw_locked(
                Context::new(program_id, &mut accounts, remaining_accounts),
                amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn drop_reward(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: DropReward");
            let ix = instruction::DropReward::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::DropReward {
                kind,
                total,
                expiry_ts,
                expiry_receiver,
                nonce,
            } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                DropReward::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::drop_reward(
                Context::new(program_id, &mut accounts, remaining_accounts),
                kind,
                total,
                expiry_ts,
                expiry_receiver,
                nonce,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn claim_reward(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: ClaimReward");
            let ix = instruction::ClaimReward::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::ClaimReward = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                ClaimReward::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::claim_reward(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn claim_reward_locked(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: ClaimRewardLocked");
            let ix = instruction::ClaimRewardLocked::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::ClaimRewardLocked { _bump, nonce } = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                ClaimRewardLocked::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::claim_reward_locked(
                Context::new(program_id, &mut accounts, remaining_accounts),
                _bump,
                nonce,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn expire_reward(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> ProgramResult {
            ::solana_program::log::sol_log("Instruction: ExpireReward");
            let ix = instruction::ExpireReward::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::ExpireReward = ix;
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                ExpireReward::try_accounts(program_id, &mut remaining_accounts, ix_data)?;
            registry::expire_reward(Context::new(program_id, &mut accounts, remaining_accounts))?;
            accounts.exit(program_id)
        }
    }
}
mod registry {
    use super::*;
    pub fn new_registry(ctx: Context<NewRegistry>, _bump: u8) -> ProgramResult {
        ctx.accounts.registry.lockup_program = *ctx.accounts.lockup_program.key;
        Ok(())
    }
    pub fn set_lockup_program(
        ctx: Context<SetLockupProgram>,
        lockup_program: Pubkey,
    ) -> ProgramResult {
        let expected: Pubkey = "HUgFuN4PbvF5YzjDSw9dQ8uTJUcwm2ANsMXwvRdY4ABx"
            .parse()
            .unwrap();
        if ctx.accounts.authority.key != &expected {
            return Err(ErrorCode::InvalidProgramAuthority.into());
        }
        ctx.accounts.registry.lockup_program = lockup_program;
        Ok(())
    }
    pub fn initialize(
        ctx: Context<Initialize>,
        mint: Pubkey,
        authority: Pubkey,
        nonce: u8,
        withdrawal_timelock: i64,
        stake_rate: u64,
        reward_q_len: u32,
    ) -> ProgramResult {
        Initialize::accounts(&ctx, nonce)?;
        let registrar = &mut ctx.accounts.registrar;
        registrar.authority = authority;
        registrar.nonce = nonce;
        registrar.mint = mint;
        registrar.pool_mint = *ctx.accounts.pool_mint.to_account_info().key;
        registrar.stake_rate = stake_rate;
        registrar.reward_event_q = *ctx.accounts.reward_event_q.to_account_info().key;
        registrar.withdrawal_timelock = withdrawal_timelock;
        let reward_q = &mut ctx.accounts.reward_event_q;
        reward_q
            .events
            .resize(reward_q_len as usize, Default::default());
        Ok(())
    }
    pub fn update_registrar(
        ctx: Context<UpdateRegistrar>,
        new_authority: Option<Pubkey>,
        withdrawal_timelock: Option<i64>,
    ) -> ProgramResult {
        let registrar = &mut ctx.accounts.registrar;
        if let Some(new_authority) = new_authority {
            registrar.authority = new_authority;
        }
        if let Some(withdrawal_timelock) = withdrawal_timelock {
            registrar.withdrawal_timelock = withdrawal_timelock;
        }
        Ok(())
    }
    pub fn create_member(ctx: Context<CreateMember>, nonce: u8) -> ProgramResult {
        CreateMember::accounts(&ctx, nonce)?;
        let member = &mut ctx.accounts.member;
        member.registrar = *ctx.accounts.registrar.to_account_info().key;
        member.beneficiary = *ctx.accounts.beneficiary.key;
        member.nonce = nonce;
        Ok(())
    }
    pub fn update_member_balances(ctx: Context<UpdateMemberBalances>, nonce: u8) -> ProgramResult {
        UpdateMemberBalances::accounts(&ctx, nonce)?;
        let member = &mut ctx.accounts.member;
        member.balances = (&ctx.accounts.balances).into();
        Ok(())
    }
    pub fn update_member_balances_lock(
        ctx: Context<UpdateMemberBalancesLock>,
        nonce: u8,
    ) -> ProgramResult {
        UpdateMemberBalancesLock::accounts(&ctx, nonce)?;
        let member = &mut ctx.accounts.member;
        member.balances_locked = (&ctx.accounts.balances_locked).into();
        Ok(())
    }
    pub fn update_member(ctx: Context<UpdateMember>, metadata: Option<Pubkey>) -> ProgramResult {
        let member = &mut ctx.accounts.member;
        if let Some(m) = metadata {
            member.metadata = m;
        }
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        token::transfer(ctx.accounts.into(), amount).map_err(Into::into)
    }
    pub fn deposit_locked(ctx: Context<DepositLocked>, amount: u64) -> ProgramResult {
        token::transfer(ctx.accounts.into(), amount).map_err(Into::into)
    }
    pub fn stake(ctx: Context<Stake>, spt_amount: u64) -> ProgramResult {
        no_available_rewards(
            &ctx.accounts.reward_event_q,
            &ctx.accounts.member,
            &ctx.accounts.balances,
        )?;
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.member.to_account_info().key.as_ref(),
                &[ctx.accounts.member.nonce],
            ];
            let member_signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                token::Transfer {
                    from: ctx.accounts.balances.vault.to_account_info(),
                    to: ctx.accounts.balances.vault_stake.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            let token_amount = spt_amount
                .checked_mul(ctx.accounts.registrar.stake_rate)
                .unwrap();
            token::transfer(cpi_ctx, token_amount)?;
        }
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                &[ctx.accounts.registrar.nonce],
            ];
            let registrar_signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                token::MintTo {
                    mint: ctx.accounts.pool_mint.to_account_info(),
                    to: ctx.accounts.balances.spt.to_account_info(),
                    authority: ctx.accounts.registrar_signer.to_account_info(),
                },
                registrar_signer,
            );
            token::mint_to(cpi_ctx, spt_amount)?;
        }
        let member = &mut ctx.accounts.member;
        member.last_stake_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
    pub fn start_unstake(
        ctx: Context<StartUnstake>,
        spt_amount: u64,
        locked: bool,
    ) -> ProgramResult {
        no_available_rewards(
            &ctx.accounts.reward_event_q,
            &ctx.accounts.member,
            &ctx.accounts.balances,
        )?;
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[ctx.accounts.member.nonce],
        ];
        let member_signer = &[&seeds[..]];
        {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                token::Burn {
                    mint: ctx.accounts.pool_mint.to_account_info(),
                    to: ctx.accounts.balances.spt.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            token::burn(cpi_ctx, spt_amount)?;
        }
        let token_amount = spt_amount
            .checked_mul(ctx.accounts.registrar.stake_rate)
            .unwrap();
        {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                token::Transfer {
                    from: ctx.accounts.balances.vault_stake.to_account_info(),
                    to: ctx.accounts.balances.vault_pw.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            token::transfer(cpi_ctx, token_amount)?;
        }
        let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
        pending_withdrawal.burned = false;
        pending_withdrawal.member = *ctx.accounts.member.to_account_info().key;
        pending_withdrawal.start_ts = ctx.accounts.clock.unix_timestamp;
        pending_withdrawal.end_ts =
            ctx.accounts.clock.unix_timestamp + ctx.accounts.registrar.withdrawal_timelock;
        pending_withdrawal.amount = token_amount;
        pending_withdrawal.pool = ctx.accounts.registrar.pool_mint;
        pending_withdrawal.registrar = *ctx.accounts.registrar.to_account_info().key;
        pending_withdrawal.locked = locked;
        let member = &mut ctx.accounts.member;
        member.last_stake_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
    pub fn end_unstake(ctx: Context<EndUnstake>) -> ProgramResult {
        if ctx.accounts.pending_withdrawal.end_ts > ctx.accounts.clock.unix_timestamp {
            return Err(ErrorCode::UnstakeTimelock.into());
        }
        let balances = {
            if ctx.accounts.pending_withdrawal.locked {
                &ctx.accounts.member.balances_locked
            } else {
                &ctx.accounts.member.balances
            }
        };
        if &balances.vault != ctx.accounts.vault.key {
            return Err(ErrorCode::InvalidVault.into());
        }
        if &balances.vault_pw != ctx.accounts.vault_pw.key {
            return Err(ErrorCode::InvalidVault.into());
        }
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.member.to_account_info().key.as_ref(),
                &[ctx.accounts.member.nonce],
            ];
            let signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                Transfer {
                    from: ctx.accounts.vault_pw.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.member_signer.clone(),
                },
                signer,
            );
            token::transfer(cpi_ctx, ctx.accounts.pending_withdrawal.amount)?;
        }
        let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
        pending_withdrawal.burned = true;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[ctx.accounts.member.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.depositor.to_account_info(),
            authority: ctx.accounts.member_signer.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount).map_err(Into::into)
    }
    pub fn withdraw_locked(ctx: Context<WithdrawLocked>, amount: u64) -> ProgramResult {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[ctx.accounts.member.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.member_vault.to_account_info(),
            to: ctx.accounts.vesting_vault.to_account_info(),
            authority: ctx.accounts.member_signer.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount).map_err(Into::into)
    }
    pub fn drop_reward(
        ctx: Context<DropReward>,
        kind: RewardVendorKind,
        total: u64,
        expiry_ts: i64,
        expiry_receiver: Pubkey,
        nonce: u8,
    ) -> ProgramResult {
        DropReward::accounts(&ctx, nonce)?;
        if total < ctx.accounts.pool_mint.supply {
            return Err(ErrorCode::InsufficientReward.into());
        }
        if ctx.accounts.clock.unix_timestamp >= expiry_ts {
            return Err(ErrorCode::InvalidExpiry.into());
        }
        if ctx.accounts.registrar.to_account_info().key == &dxl_registrar::ID {
            if ctx.accounts.vendor_vault.mint != dxl_mint::ID {
                return Err(ErrorCode::InvalidMint.into());
            }
            if total < DXL_MIN_REWARD {
                return Err(ErrorCode::InsufficientReward.into());
            }
        } else if ctx.accounts.registrar.to_account_info().key == &fida_registrar::ID {
            if ctx.accounts.vendor_vault.mint != fida_mint::ID {
                return Err(ErrorCode::InvalidMint.into());
            }
            if total < FIDA_MIN_REWARD {
                return Err(ErrorCode::InsufficientReward.into());
            }
        } else if ctx.accounts.registrar.to_account_info().key == &srm_registrar::ID
            || ctx.accounts.registrar.to_account_info().key == &msrm_registrar::ID
        {
            if ctx.accounts.vendor_vault.mint != srm_mint::ID {
                return Err(ErrorCode::InvalidMint.into());
            }
            if total < SRM_MIN_REWARD {
                return Err(ErrorCode::InsufficientReward.into());
            }
        } else {
            ::solana_program::log::sol_log(
                "Reward amount not constrained. Please open a pull request.",
            );
        }
        if let RewardVendorKind::Locked {
            start_ts,
            end_ts,
            period_count,
        } = kind
        {
            if !lockup::is_valid_schedule(start_ts, end_ts, period_count) {
                return Err(ErrorCode::InvalidVestingSchedule.into());
            }
        }
        token::transfer(ctx.accounts.into(), total)?;
        let reward_q = &mut ctx.accounts.reward_event_q;
        let cursor = reward_q.append(RewardEvent {
            vendor: *ctx.accounts.vendor.to_account_info().key,
            ts: ctx.accounts.clock.unix_timestamp,
            locked: kind != RewardVendorKind::Unlocked,
        })?;
        let vendor = &mut ctx.accounts.vendor;
        vendor.registrar = *ctx.accounts.registrar.to_account_info().key;
        vendor.vault = *ctx.accounts.vendor_vault.to_account_info().key;
        vendor.mint = ctx.accounts.vendor_vault.mint;
        vendor.nonce = nonce;
        vendor.pool_token_supply = ctx.accounts.pool_mint.supply;
        vendor.reward_event_q_cursor = cursor;
        vendor.start_ts = ctx.accounts.clock.unix_timestamp;
        vendor.expiry_ts = expiry_ts;
        vendor.expiry_receiver = expiry_receiver;
        vendor.from = *ctx.accounts.depositor_authority.key;
        vendor.total = total;
        vendor.expired = false;
        vendor.kind = kind;
        Ok(())
    }
    pub fn claim_reward(ctx: Context<ClaimReward>) -> ProgramResult {
        reward_eligible(&ctx.accounts.cmn)?;
        if RewardVendorKind::Unlocked != ctx.accounts.cmn.vendor.kind {
            return Err(ErrorCode::ExpectedUnlockedVendor.into());
        }
        let spt_total =
            ctx.accounts.cmn.balances_spt.amount + ctx.accounts.cmn.balances_locked_spt.amount;
        let reward_amount = spt_total
            .checked_mul(ctx.accounts.cmn.vendor.total)
            .unwrap()
            .checked_div(ctx.accounts.cmn.vendor.pool_token_supply)
            .unwrap();
        if !(reward_amount > 0) {
            ::core::panicking::panic("assertion failed: reward_amount > 0")
        };
        let seeds = &[
            ctx.accounts.cmn.registrar.to_account_info().key.as_ref(),
            ctx.accounts.cmn.vendor.to_account_info().key.as_ref(),
            &[ctx.accounts.cmn.vendor.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.cmn.token_program.clone(),
            token::Transfer {
                from: ctx.accounts.cmn.vault.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.cmn.vendor_signer.to_account_info(),
            },
            signer,
        );
        token::transfer(cpi_ctx, reward_amount)?;
        let member = &mut ctx.accounts.cmn.member;
        member.rewards_cursor = ctx.accounts.cmn.vendor.reward_event_q_cursor + 1;
        Ok(())
    }
    pub fn claim_reward_locked<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimRewardLocked<'info>>,
        _bump: u8,
        nonce: u8,
    ) -> ProgramResult {
        reward_eligible(&ctx.accounts.cmn)?;
        let (start_ts, end_ts, period_count) = match ctx.accounts.cmn.vendor.kind {
            RewardVendorKind::Unlocked => return Err(ErrorCode::ExpectedLockedVendor.into()),
            RewardVendorKind::Locked {
                start_ts,
                end_ts,
                period_count,
            } => (start_ts, end_ts, period_count),
        };
        let spt_total =
            ctx.accounts.cmn.balances_spt.amount + ctx.accounts.cmn.balances_locked_spt.amount;
        let reward_amount = spt_total
            .checked_mul(ctx.accounts.cmn.vendor.total)
            .unwrap()
            .checked_div(ctx.accounts.cmn.vendor.pool_token_supply)
            .unwrap();
        if !(reward_amount > 0) {
            ::core::panicking::panic("assertion failed: reward_amount > 0")
        };
        let realizor = Some(Realizor {
            program: *ctx.program_id,
            metadata: *ctx.accounts.cmn.member.to_account_info().key,
        });
        let seeds = &[
            ctx.accounts.cmn.registrar.to_account_info().key.as_ref(),
            ctx.accounts.cmn.vendor.to_account_info().key.as_ref(),
            &[ctx.accounts.cmn.vendor.nonce],
        ];
        let signer = &[&seeds[..]];
        let mut depositor_authority = ctx.accounts.cmn.vendor_signer.clone();
        depositor_authority.is_signer = true;
        let mut new_remaining_accounts = &[
            ctx.remaining_accounts[0].clone(),
            ctx.remaining_accounts[1].clone(),
            ctx.remaining_accounts[2].clone(),
            depositor_authority.clone(),
            ctx.remaining_accounts[4].clone(),
            ctx.remaining_accounts[5].clone(),
            ctx.remaining_accounts[6].clone(),
        ][..];
        let cpi_program = ctx.accounts.lockup_program.clone();
        let cpi_accounts = {
            let accs = CreateVesting::try_accounts(
                ctx.accounts.lockup_program.key,
                &mut new_remaining_accounts,
                &[],
            )?;
            lockup::cpi::accounts::CreateVesting {
                vesting: accs.vesting.to_account_info(),
                vault: accs.vault.to_account_info(),
                depositor: accs.depositor.to_account_info(),
                depositor_authority: accs.depositor_authority.to_account_info(),
                token_program: accs.token_program.to_account_info(),
                clock: accs.clock.to_account_info(),
                rent: accs.rent.to_account_info(),
            }
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        lockup::cpi::create_vesting(
            cpi_ctx,
            ctx.accounts.cmn.member.beneficiary,
            reward_amount,
            nonce,
            start_ts,
            end_ts,
            period_count,
            realizor,
        )?;
        let member = &mut ctx.accounts.cmn.member;
        member.rewards_cursor = ctx.accounts.cmn.vendor.reward_event_q_cursor + 1;
        Ok(())
    }
    pub fn expire_reward(ctx: Context<ExpireReward>) -> ProgramResult {
        if ctx.accounts.clock.unix_timestamp < ctx.accounts.vendor.expiry_ts {
            return Err(ErrorCode::VendorNotYetExpired.into());
        }
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.vendor.to_account_info().key.as_ref(),
            &[ctx.accounts.vendor.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.clone(),
            token::Transfer {
                to: ctx.accounts.expiry_receiver_token.to_account_info(),
                from: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.vendor_signer.to_account_info(),
            },
            signer,
        );
        token::transfer(cpi_ctx, ctx.accounts.vault.amount)?;
        let vendor = &mut ctx.accounts.vendor;
        vendor.expired = true;
        Ok(())
    }
    pub struct Registry {
        pub lockup_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for Registry
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lockup_program, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Registry
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                lockup_program: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Registry {
        #[inline]
        fn clone(&self) -> Registry {
            match *self {
                Registry {
                    lockup_program: ref __self_0_0,
                } => Registry {
                    lockup_program: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Registry {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> std::result::Result<(), ProgramError> {
            writer
                .write_all(&[47, 174, 110, 246, 184, 182, 252, 218])
                .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
            AnchorSerialize::serialize(self, writer)
                .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Registry {
        fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
            if buf.len() < [47, 174, 110, 246, 184, 182, 252, 218].len() {
                return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
            }
            let given_disc = &buf[..8];
            if &[47, 174, 110, 246, 184, 182, 252, 218] != given_disc {
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
    impl anchor_lang::Discriminator for Registry {
        fn discriminator() -> [u8; 8] {
            [47, 174, 110, 246, 184, 182, 252, 218]
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Registry {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl<'info> RealizeLock<'info, IsRealized<'info>> for Registry {
        fn is_realized(ctx: Context<IsRealized>, v: Vesting) -> ProgramResult {
            if let Some(realizor) = &v.realizor {
                if &realizor.metadata != ctx.accounts.member.to_account_info().key {
                    return Err(ErrorCode::InvalidRealizorMetadata.into());
                }
                if !(ctx.accounts.member.beneficiary == v.beneficiary) {
                    ::core::panicking::panic(
                        "assertion failed: ctx.accounts.member.beneficiary == v.beneficiary",
                    )
                };
                let total_staked =
                    ctx.accounts.member_spt.amount + ctx.accounts.member_spt_locked.amount;
                if total_staked != 0 {
                    return Err(ErrorCode::UnrealizedReward.into());
                }
            }
            Ok(())
        }
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
    pub struct NewRegistry {
        pub _bump: u8,
    }
    impl borsh::ser::BorshSerialize for NewRegistry
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
    impl borsh::de::BorshDeserialize for NewRegistry
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
    impl anchor_lang::InstructionData for NewRegistry {
        fn data(&self) -> Vec<u8> {
            let mut d = [237, 187, 50, 70, 74, 26, 144, 230].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct SetLockupProgram {
        pub lockup_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetLockupProgram
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lockup_program, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetLockupProgram
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                lockup_program: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetLockupProgram {
        fn data(&self) -> Vec<u8> {
            let mut d = [223, 79, 250, 198, 20, 31, 79, 117].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Initialize {
        pub mint: Pubkey,
        pub authority: Pubkey,
        pub nonce: u8,
        pub withdrawal_timelock: i64,
        pub stake_rate: u64,
        pub reward_q_len: u32,
    }
    impl borsh::ser::BorshSerialize for Initialize
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            borsh::BorshSerialize::serialize(&self.withdrawal_timelock, writer)?;
            borsh::BorshSerialize::serialize(&self.stake_rate, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_q_len, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Initialize
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mint: borsh::BorshDeserialize::deserialize(buf)?,
                authority: borsh::BorshDeserialize::deserialize(buf)?,
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
                withdrawal_timelock: borsh::BorshDeserialize::deserialize(buf)?,
                stake_rate: borsh::BorshDeserialize::deserialize(buf)?,
                reward_q_len: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Initialize {
        fn data(&self) -> Vec<u8> {
            let mut d = [175, 175, 109, 31, 13, 152, 155, 237].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateRegistrar {
        pub new_authority: Option<Pubkey>,
        pub withdrawal_timelock: Option<i64>,
    }
    impl borsh::ser::BorshSerialize for UpdateRegistrar
    where
        Option<Pubkey>: borsh::ser::BorshSerialize,
        Option<i64>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.new_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.withdrawal_timelock, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateRegistrar
    where
        Option<Pubkey>: borsh::BorshDeserialize,
        Option<i64>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                new_authority: borsh::BorshDeserialize::deserialize(buf)?,
                withdrawal_timelock: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateRegistrar {
        fn data(&self) -> Vec<u8> {
            let mut d = [116, 34, 168, 129, 238, 12, 90, 56].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateMember {
        pub nonce: u8,
    }
    impl borsh::ser::BorshSerialize for CreateMember
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateMember
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CreateMember {
        fn data(&self) -> Vec<u8> {
            let mut d = [49, 46, 45, 241, 122, 143, 136, 73].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateMemberBalances {
        pub nonce: u8,
    }
    impl borsh::ser::BorshSerialize for UpdateMemberBalances
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateMemberBalances
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateMemberBalances {
        fn data(&self) -> Vec<u8> {
            let mut d = [106, 188, 198, 184, 221, 120, 117, 103].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateMemberBalancesLock {
        pub nonce: u8,
    }
    impl borsh::ser::BorshSerialize for UpdateMemberBalancesLock
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateMemberBalancesLock
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateMemberBalancesLock {
        fn data(&self) -> Vec<u8> {
            let mut d = [255, 3, 165, 86, 185, 28, 102, 93].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateMember {
        pub metadata: Option<Pubkey>,
    }
    impl borsh::ser::BorshSerialize for UpdateMember
    where
        Option<Pubkey>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.metadata, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateMember
    where
        Option<Pubkey>: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                metadata: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for UpdateMember {
        fn data(&self) -> Vec<u8> {
            let mut d = [46, 229, 3, 194, 47, 105, 211, 28].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Deposit {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for Deposit
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
    impl borsh::de::BorshDeserialize for Deposit
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
    impl anchor_lang::InstructionData for Deposit {
        fn data(&self) -> Vec<u8> {
            let mut d = [242, 35, 198, 137, 82, 225, 242, 182].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DepositLocked {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for DepositLocked
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
    impl borsh::de::BorshDeserialize for DepositLocked
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
    impl anchor_lang::InstructionData for DepositLocked {
        fn data(&self) -> Vec<u8> {
            let mut d = [88, 91, 135, 52, 79, 190, 164, 141].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Stake {
        pub spt_amount: u64,
    }
    impl borsh::ser::BorshSerialize for Stake
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.spt_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Stake
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                spt_amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Stake {
        fn data(&self) -> Vec<u8> {
            let mut d = [206, 176, 202, 18, 200, 209, 179, 108].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct StartUnstake {
        pub spt_amount: u64,
        pub locked: bool,
    }
    impl borsh::ser::BorshSerialize for StartUnstake
    where
        u64: borsh::ser::BorshSerialize,
        bool: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.spt_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.locked, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for StartUnstake
    where
        u64: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                spt_amount: borsh::BorshDeserialize::deserialize(buf)?,
                locked: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for StartUnstake {
        fn data(&self) -> Vec<u8> {
            let mut d = [200, 243, 106, 111, 170, 72, 31, 117].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct EndUnstake;
    impl borsh::ser::BorshSerialize for EndUnstake {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for EndUnstake {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for EndUnstake {
        fn data(&self) -> Vec<u8> {
            let mut d = [44, 65, 159, 108, 149, 89, 27, 203].to_vec();
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
    pub struct WithdrawLocked {
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for WithdrawLocked
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
    impl borsh::de::BorshDeserialize for WithdrawLocked
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
    impl anchor_lang::InstructionData for WithdrawLocked {
        fn data(&self) -> Vec<u8> {
            let mut d = [96, 224, 88, 102, 223, 189, 8, 228].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct DropReward {
        pub kind: RewardVendorKind,
        pub total: u64,
        pub expiry_ts: i64,
        pub expiry_receiver: Pubkey,
        pub nonce: u8,
    }
    impl borsh::ser::BorshSerialize for DropReward
    where
        RewardVendorKind: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.kind, writer)?;
            borsh::BorshSerialize::serialize(&self.total, writer)?;
            borsh::BorshSerialize::serialize(&self.expiry_ts, writer)?;
            borsh::BorshSerialize::serialize(&self.expiry_receiver, writer)?;
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for DropReward
    where
        RewardVendorKind: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                kind: borsh::BorshDeserialize::deserialize(buf)?,
                total: borsh::BorshDeserialize::deserialize(buf)?,
                expiry_ts: borsh::BorshDeserialize::deserialize(buf)?,
                expiry_receiver: borsh::BorshDeserialize::deserialize(buf)?,
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for DropReward {
        fn data(&self) -> Vec<u8> {
            let mut d = [170, 77, 234, 93, 202, 35, 96, 101].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ClaimReward;
    impl borsh::ser::BorshSerialize for ClaimReward {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ClaimReward {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for ClaimReward {
        fn data(&self) -> Vec<u8> {
            let mut d = [149, 95, 181, 242, 94, 90, 158, 162].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ClaimRewardLocked {
        pub _bump: u8,
        pub nonce: u8,
    }
    impl borsh::ser::BorshSerialize for ClaimRewardLocked
    where
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._bump, writer)?;
            borsh::BorshSerialize::serialize(&self.nonce, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ClaimRewardLocked
    where
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _bump: borsh::BorshDeserialize::deserialize(buf)?,
                nonce: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for ClaimRewardLocked {
        fn data(&self) -> Vec<u8> {
            let mut d = [231, 6, 114, 60, 66, 89, 22, 135].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ExpireReward;
    impl borsh::ser::BorshSerialize for ExpireReward {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ExpireReward {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for ExpireReward {
        fn data(&self) -> Vec<u8> {
            let mut d = [59, 250, 57, 193, 205, 47, 0, 122].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_update_member_balances_lock::*;
    pub use crate::__client_accounts_drop_reward::*;
    pub use crate::__client_accounts_update_member::*;
    pub use crate::__client_accounts_withdraw_locked::*;
    pub use crate::__client_accounts_deposit_locked::*;
    pub use crate::__client_accounts_set_lockup_program::*;
    pub use crate::__client_accounts_new_registry::*;
    pub use crate::__client_accounts_start_unstake::*;
    pub use crate::__client_accounts_end_unstake::*;
    pub use crate::__client_accounts_deposit::*;
    pub use crate::__client_accounts_claim_reward_locked::*;
    pub use crate::__client_accounts_create_member::*;
    pub use crate::__client_accounts_stake::*;
    pub use crate::__client_accounts_claim_reward::*;
    pub use crate::__client_accounts_expire_reward::*;
    pub use crate::__client_accounts_initialize::*;
    pub use crate::__client_accounts_update_registrar::*;
    pub use crate::__client_accounts_withdraw::*;
    pub use crate::__client_accounts_update_member_balances::*;
}
pub struct Initialize<'info> {
    #[account(zero)]
    registrar: Box<Account<'info, Registrar>>,
    #[account(zero)]
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account("pool_mint.decimals == 0")]
    pool_mint: Account<'info, Mint>,
    rent: Sysvar<'info, Rent>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Initialize<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar = &accounts[0];
        *accounts = &accounts[1..];
        let reward_event_q = &accounts[0];
        *accounts = &accounts[1..];
        let pool_mint: anchor_lang::Account<Mint> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let registrar: Box<anchor_lang::Account<Registrar>> = {
            let mut __data: &[u8] = &registrar.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(&registrar)?)
        };
        if !registrar.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            registrar.to_account_info().lamports(),
            registrar.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        let __anchor_rent = Rent::get()?;
        let reward_event_q: Box<anchor_lang::Account<RewardQueue>> = {
            let mut __data: &[u8] = &reward_event_q.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(&reward_event_q)?)
        };
        if !reward_event_q.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            reward_event_q.to_account_info().lamports(),
            reward_event_q.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !(pool_mint.decimals == 0) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(Initialize {
            registrar,
            reward_event_q,
            pool_mint,
            rent,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.reward_event_q.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.reward_event_q.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Initialize<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.registrar, program_id)?;
        anchor_lang::AccountsExit::exit(&self.reward_event_q, program_id)?;
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
pub(crate) mod __client_accounts_initialize {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Initialize {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub reward_event_q: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Initialize
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Initialize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.registrar,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.reward_event_q,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.pool_mint,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
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
pub(crate) mod __cpi_client_accounts_initialize {
    use super::*;
    pub struct Initialize<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub reward_event_q: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.registrar),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.reward_event_q),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.pool_mint),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.reward_event_q,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos
        }
    }
}
impl<'info> Initialize<'info> {
    fn accounts(ctx: &Context<Initialize<'info>>, nonce: u8) -> ProgramResult {
        let registrar_signer = Pubkey::create_program_address(
            &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| ErrorCode::InvalidNonce)?;
        if ctx.accounts.pool_mint.mint_authority != COption::Some(registrar_signer) {
            return Err(ErrorCode::InvalidPoolMintAuthority.into());
        }
        if !(ctx.accounts.pool_mint.supply == 0) {
            ::core::panicking::panic("assertion failed: ctx.accounts.pool_mint.supply == 0")
        };
        Ok(())
    }
}
pub struct UpdateRegistrar<'info> {
    # [account (mut , has_one = authority)]
    registrar: Box<Account<'info, Registrar>>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for UpdateRegistrar<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !registrar.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &registrar.authority != authority.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(UpdateRegistrar {
            registrar,
            authority,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateRegistrar<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateRegistrar<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateRegistrar<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.registrar, program_id)?;
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
pub(crate) mod __client_accounts_update_registrar {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct UpdateRegistrar {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateRegistrar
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateRegistrar {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.registrar,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.authority,
                    true,
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
pub(crate) mod __cpi_client_accounts_update_registrar {
    use super::*;
    pub struct UpdateRegistrar<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateRegistrar<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.registrar),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.authority),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateRegistrar<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos
        }
    }
}
pub struct CreateMember<'info> {
    registrar: Box<Account<'info, Registrar>>,
    #[account(zero)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    member_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CreateMember<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member = &accounts[0];
        *accounts = &accounts[1..];
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let member: Box<anchor_lang::Account<Member>> = {
            let mut __data: &[u8] = &member.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(&member)?)
        };
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            member.to_account_info().lamports(),
            member.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(CreateMember {
            registrar,
            member,
            beneficiary,
            member_signer,
            token_program,
            rent,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateMember<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateMember<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateMember<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
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
pub(crate) mod __client_accounts_create_member {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct CreateMember {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateMember
    where
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
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateMember {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
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
pub(crate) mod __cpi_client_accounts_create_member {
    use super::*;
    pub struct CreateMember<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateMember<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
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
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateMember<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos
        }
    }
}
impl<'info> CreateMember<'info> {
    fn accounts(ctx: &Context<CreateMember>, nonce: u8) -> ProgramResult {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[nonce],
        ];
        let member_signer = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| ErrorCode::InvalidNonce)?;
        if &member_signer != ctx.accounts.member_signer.to_account_info().key {
            return Err(ErrorCode::InvalidMemberSigner.into());
        }
        Ok(())
    }
}
pub struct UpdateMemberBalances<'info> {
    registrar: Box<Account<'info, Registrar>>,
    #[account(mut)]
    member: Box<Account<'info, Member>>,
    #[account(
        "&balances.spt.owner == member_signer.key",
        "balances.spt.mint == registrar.pool_mint",
        "balances.vault.mint == registrar.mint"
    )]
    balances: BalanceSandboxAccounts<'info>,
    member_signer: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for UpdateMemberBalances<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances: BalanceSandboxAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(&balances.spt.owner == member_signer.key) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(balances.spt.mint == registrar.pool_mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(balances.vault.mint == registrar.mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(UpdateMemberBalances {
            registrar,
            member,
            balances,
            member_signer,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMemberBalances<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.balances.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateMemberBalances<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.balances.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateMemberBalances<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
        anchor_lang::AccountsExit::exit(&self.balances, program_id)?;
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
pub(crate) mod __client_accounts_update_member_balances {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct UpdateMemberBalances {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances: __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateMemberBalances
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts:
            borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.balances, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateMemberBalances {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
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
pub(crate) mod __cpi_client_accounts_update_member_balances {
    use super::*;
    pub use __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct UpdateMemberBalances<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances: __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateMemberBalances<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMemberBalances<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.balances,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos
        }
    }
}
impl<'info> UpdateMemberBalances<'info> {
    fn accounts(ctx: &Context<UpdateMemberBalances>, nonce: u8) -> ProgramResult {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[nonce],
        ];
        let member_signer = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| ErrorCode::InvalidNonce)?;
        if &member_signer != ctx.accounts.member_signer.to_account_info().key {
            return Err(ErrorCode::InvalidMemberSigner.into());
        }
        Ok(())
    }
}
pub struct UpdateMemberBalancesLock<'info> {
    registrar: Box<Account<'info, Registrar>>,
    #[account(mut)]
    member: Box<Account<'info, Member>>,
    #[account(
        "&balances_locked.spt.owner == member_signer.key",
        "balances_locked.spt.mint == registrar.pool_mint",
        "balances_locked.vault.mint == registrar.mint"
    )]
    balances_locked: BalanceSandboxAccounts<'info>,
    member_signer: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for UpdateMemberBalancesLock<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances_locked: BalanceSandboxAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(&balances_locked.spt.owner == member_signer.key) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(balances_locked.spt.mint == registrar.pool_mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(balances_locked.vault.mint == registrar.mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(UpdateMemberBalancesLock {
            registrar,
            member,
            balances_locked,
            member_signer,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMemberBalancesLock<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.balances_locked.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateMemberBalancesLock<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.balances_locked.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateMemberBalancesLock<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
        anchor_lang::AccountsExit::exit(&self.balances_locked, program_id)?;
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
pub(crate) mod __client_accounts_update_member_balances_lock {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct UpdateMemberBalancesLock {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances_locked: __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateMemberBalancesLock
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts:
            borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.balances_locked, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateMemberBalancesLock {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.extend(self.balances_locked.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
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
pub(crate) mod __cpi_client_accounts_update_member_balances_lock {
    use super::*;
    pub use __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct UpdateMemberBalancesLock<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances_locked:
            __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateMemberBalancesLock<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.extend(self.balances_locked.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMemberBalancesLock<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.balances_locked,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos
        }
    }
}
impl<'info> UpdateMemberBalancesLock<'info> {
    fn accounts(ctx: &Context<UpdateMemberBalancesLock>, nonce: u8) -> ProgramResult {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[nonce],
        ];
        let member_signer = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| ErrorCode::InvalidNonce)?;
        if &member_signer != ctx.accounts.member_signer.to_account_info().key {
            return Err(ErrorCode::InvalidMemberSigner.into());
        }
        Ok(())
    }
}
pub struct BalanceSandboxAccounts<'info> {
    #[account(mut)]
    spt: Account<'info, TokenAccount>,
    #[account(mut, "vault.owner == spt.owner")]
    vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        "vault_stake.owner == spt.owner",
        "vault_stake.mint == vault.mint"
    )]
    vault_stake: Account<'info, TokenAccount>,
    #[account(mut, "vault_pw.owner == spt.owner", "vault_pw.mint == vault.mint")]
    vault_pw: Account<'info, TokenAccount>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for BalanceSandboxAccounts<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let spt: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault_stake: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault_pw: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !spt.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vault.owner == spt.owner) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vault_stake.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vault_stake.owner == spt.owner) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(vault_stake.mint == vault.mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vault_pw.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vault_pw.owner == spt.owner) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(vault_pw.mint == vault.mint) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(BalanceSandboxAccounts {
            spt,
            vault,
            vault_stake,
            vault_pw,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for BalanceSandboxAccounts<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.spt.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vault_stake.to_account_infos());
        account_infos.extend(self.vault_pw.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for BalanceSandboxAccounts<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.spt.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vault_stake.to_account_metas(None));
        account_metas.extend(self.vault_pw.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for BalanceSandboxAccounts<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.spt, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault_stake, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault_pw, program_id)?;
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
pub(crate) mod __client_accounts_balance_sandbox_accounts {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct BalanceSandboxAccounts {
        pub spt: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault_stake: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault_pw: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for BalanceSandboxAccounts
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.spt, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vault_stake, writer)?;
            borsh::BorshSerialize::serialize(&self.vault_pw, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for BalanceSandboxAccounts {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.spt, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault_stake,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault_pw,
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
pub(crate) mod __cpi_client_accounts_balance_sandbox_accounts {
    use super::*;
    pub struct BalanceSandboxAccounts<'info> {
        pub spt: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault_stake: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault_pw: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for BalanceSandboxAccounts<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.spt),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault_stake),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault_pw),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for BalanceSandboxAccounts<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.spt));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vault_stake,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault_pw));
            account_infos
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'info> ::core::clone::Clone for BalanceSandboxAccounts<'info> {
    #[inline]
    fn clone(&self) -> BalanceSandboxAccounts<'info> {
        match *self {
            BalanceSandboxAccounts {
                spt: ref __self_0_0,
                vault: ref __self_0_1,
                vault_stake: ref __self_0_2,
                vault_pw: ref __self_0_3,
            } => BalanceSandboxAccounts {
                spt: ::core::clone::Clone::clone(&(*__self_0_0)),
                vault: ::core::clone::Clone::clone(&(*__self_0_1)),
                vault_stake: ::core::clone::Clone::clone(&(*__self_0_2)),
                vault_pw: ::core::clone::Clone::clone(&(*__self_0_3)),
            },
        }
    }
}
# [instruction (registry_bump : u8)]
pub struct NewRegistry<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub lockup_program: AccountInfo<'info>,
    # [account (init , seeds = [b"registry" . as_ref ()] , bump = registry_bump , payer = authority , space = 1000)]
    pub registry: Box<Account<'info, Registry>>,
    pub system_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for NewRegistry<'info>
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
            registry_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.registry_bump, writer)?;
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
                    registry_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { registry_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let lockup_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry = &accounts[0];
        *accounts = &accounts[1..];
        let system_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __anchor_rent = Rent::get()?;
        let registry = {
            let actual_field = registry.to_account_info();
            let actual_owner = actual_field.owner;
            let space = 1000;
            if !false || actual_owner == &anchor_lang::solana_program::system_program::ID {
                let payer = authority.to_account_info();
                let __current_lamports = registry.to_account_info().lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::create_account(
                            payer.to_account_info().key,
                            registry.to_account_info().key,
                            lamports,
                            space as u64,
                            program_id,
                        ),
                        &[
                            payer.to_account_info(),
                            registry.to_account_info(),
                            system_program.to_account_info(),
                        ],
                        &[&[b"registry".as_ref(), &[registry_bump][..]][..]],
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
                                registry.to_account_info().key,
                                required_lamports,
                            ),
                            &[
                                payer.to_account_info(),
                                registry.to_account_info(),
                                system_program.to_account_info(),
                            ],
                        )?;
                    }
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::allocate(
                            registry.to_account_info().key,
                            space as u64,
                        ),
                        &[registry.to_account_info(), system_program.to_account_info()],
                        &[&[b"registry".as_ref(), &[registry_bump][..]][..]],
                    )?;
                    anchor_lang::solana_program::program::invoke_signed(
                        &anchor_lang::solana_program::system_instruction::assign(
                            registry.to_account_info().key,
                            program_id,
                        ),
                        &[registry.to_account_info(), system_program.to_account_info()],
                        &[&[b"registry".as_ref(), &[registry_bump][..]][..]],
                    )?;
                }
            }
            let pa: Box<anchor_lang::Account<Registry>> =
                Box::new(anchor_lang::Account::try_from_unchecked(&registry)?);
            if !(!false || actual_owner == &anchor_lang::solana_program::system_program::ID) {
                if space != actual_field.data_len() {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSpace.into());
                }
                if actual_owner != program_id {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintOwner.into());
                }
                let expected_key = anchor_lang::prelude::Pubkey::create_program_address(
                    &[b"registry".as_ref(), &[registry_bump][..]][..],
                    program_id,
                )
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
                if expected_key != registry.key() {
                    return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
                }
            }
            pa
        };
        let (__program_signer, __bump) =
            anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
                &[b"registry".as_ref()],
                program_id,
            );
        if registry.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if __bump != registry_bump {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !registry.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            registry.to_account_info().lamports(),
            registry.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(NewRegistry {
            authority,
            lockup_program,
            registry,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for NewRegistry<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.lockup_program.to_account_infos());
        account_infos.extend(self.registry.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for NewRegistry<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas.extend(self.lockup_program.to_account_metas(None));
        account_metas.extend(self.registry.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for NewRegistry<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.registry, program_id)?;
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
pub(crate) mod __client_accounts_new_registry {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct NewRegistry {
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub lockup_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub registry: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for NewRegistry
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.lockup_program, writer)?;
            borsh::BorshSerialize::serialize(&self.registry, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for NewRegistry {
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
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.lockup_program,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.registry,
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
pub(crate) mod __cpi_client_accounts_new_registry {
    use super::*;
    pub struct NewRegistry<'info> {
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub lockup_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registry: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for NewRegistry<'info> {
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
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.lockup_program),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.registry),
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
    impl<'info> anchor_lang::ToAccountInfos<'info> for NewRegistry<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.lockup_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registry));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.system_program,
            ));
            account_infos
        }
    }
}
# [instruction (registry_bump : u8)]
pub struct SetLockupProgram<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    # [account (mut , seeds = [b"registry" . as_ref ()] , bump = registry_bump)]
    pub registry: Box<Account<'info, Registry>>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for SetLockupProgram<'info>
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
            registry_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.registry_bump, writer)?;
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
                    registry_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { registry_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry: Box<anchor_lang::Account<Registry>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[b"registry".as_ref(), &[registry_bump][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if registry.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !registry.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(SetLockupProgram {
            authority,
            registry,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SetLockupProgram<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.registry.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SetLockupProgram<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.authority.to_account_metas(Some(true)));
        account_metas.extend(self.registry.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SetLockupProgram<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.registry, program_id)?;
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
pub(crate) mod __client_accounts_set_lockup_program {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct SetLockupProgram {
        pub authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub registry: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetLockupProgram
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.registry, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SetLockupProgram {
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
                self.registry,
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
pub(crate) mod __cpi_client_accounts_set_lockup_program {
    use super::*;
    pub struct SetLockupProgram<'info> {
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registry: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SetLockupProgram<'info> {
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
                anchor_lang::Key::key(&self.registry),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SetLockupProgram<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registry));
            account_infos
        }
    }
}
pub struct IsRealized<'info> {
    #[account(
        "&member.balances.spt == member_spt.to_account_info().key",
        "&member.balances_locked.spt == member_spt_locked.to_account_info().key"
    )]
    member: Box<Account<'info, Member>>,
    member_spt: Account<'info, TokenAccount>,
    member_spt_locked: Account<'info, TokenAccount>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for IsRealized<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_spt: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_spt_locked: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !(&member.balances.spt == member_spt.to_account_info().key) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(&member.balances_locked.spt == member_spt_locked.to_account_info().key) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(IsRealized {
            member,
            member_spt,
            member_spt_locked,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for IsRealized<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.member_spt.to_account_infos());
        account_infos.extend(self.member_spt_locked.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for IsRealized<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.member_spt.to_account_metas(None));
        account_metas.extend(self.member_spt_locked.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for IsRealized<'info>
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
pub(crate) mod __client_accounts_is_realized {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct IsRealized {
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_spt: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_spt_locked: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for IsRealized
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.member_spt, writer)?;
            borsh::BorshSerialize::serialize(&self.member_spt_locked, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for IsRealized {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_spt,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_spt_locked,
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
pub(crate) mod __cpi_client_accounts_is_realized {
    use super::*;
    pub struct IsRealized<'info> {
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_spt: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_spt_locked: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for IsRealized<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_spt),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_spt_locked),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for IsRealized<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_spt,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_spt_locked,
            ));
            account_infos
        }
    }
}
pub struct UpdateMember<'info> {
    # [account (mut , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for UpdateMember<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(UpdateMember {
            member,
            beneficiary,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMember<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateMember<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateMember<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
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
pub(crate) mod __client_accounts_update_member {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct UpdateMember {
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateMember
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateMember {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
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
pub(crate) mod __cpi_client_accounts_update_member {
    use super::*;
    pub struct UpdateMember<'info> {
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateMember<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateMember<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos
        }
    }
}
pub struct Deposit<'info> {
    # [account (has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, "vault.to_account_info().key == &member.balances.vault")]
    vault: Account<'info, TokenAccount>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(signer, "depositor_authority.key == &member.beneficiary")]
    depositor_authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Deposit<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vault.to_account_info().key == &member.balances.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !depositor.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !depositor_authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(depositor_authority.key == &member.beneficiary) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(Deposit {
            member,
            beneficiary,
            vault,
            depositor,
            depositor_authority,
            token_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.depositor.to_account_infos());
        account_infos.extend(self.depositor_authority.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.depositor.to_account_metas(None));
        account_metas.extend(self.depositor_authority.to_account_metas(Some(true)));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Deposit<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
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
pub(crate) mod __client_accounts_deposit {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Deposit {
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor_authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Deposit
    where
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
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Deposit {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
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
pub(crate) mod __cpi_client_accounts_deposit {
    use super::*;
    pub struct Deposit<'info> {
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Deposit<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
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
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Deposit<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.depositor_authority,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos
        }
    }
}
pub struct DepositLocked<'info> {
    #[account(
        "vesting.to_account_info().owner == &registry.lockup_program",
        "vesting.beneficiary == member.beneficiary"
    )]
    vesting: Account<'info, Vesting>,
    #[account(mut, "vesting_vault.key == &vesting.vault")]
    vesting_vault: AccountInfo<'info>,
    #[account(signer)]
    depositor_authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    #[account(
        mut,
        "member_vault.to_account_info().key == &member.balances_locked.vault"
    )]
    member_vault: Account<'info, TokenAccount>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () , & [member . nonce] ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    # [account (seeds = [b"the-state" . as_ref ()] , bump = 0)]
    registry: Box<Account<'info, Registry>>,
    registrar: Box<Account<'info, Registrar>>,
    # [account (has_one = registrar , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for DepositLocked<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let vesting: anchor_lang::Account<Vesting> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry: Box<anchor_lang::Account<Registry>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !(vesting.to_account_info().owner == &registry.lockup_program) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(vesting.beneficiary == member.beneficiary) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vesting_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vesting_vault.key == &vesting.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !depositor_authority.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !member_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(member_vault.to_account_info().key == &member.balances_locked.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce],
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        let __program_signer =
            Pubkey::create_program_address(&[b"the-state".as_ref(), &[0][..]][..], program_id)
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if registry.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(DepositLocked {
            vesting,
            vesting_vault,
            depositor_authority,
            token_program,
            member_vault,
            member_signer,
            registry,
            registrar,
            member,
            beneficiary,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for DepositLocked<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.vesting_vault.to_account_infos());
        account_infos.extend(self.depositor_authority.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.member_vault.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.registry.to_account_infos());
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for DepositLocked<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.vesting_vault.to_account_metas(None));
        account_metas.extend(self.depositor_authority.to_account_metas(Some(true)));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.member_vault.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.registry.to_account_metas(None));
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for DepositLocked<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vesting_vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.member_vault, program_id)?;
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
pub(crate) mod __client_accounts_deposit_locked {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct DepositLocked {
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor_authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub registry: anchor_lang::solana_program::pubkey::Pubkey,
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DepositLocked
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
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.member_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.registry, writer)?;
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for DepositLocked {
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vesting_vault,
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registry,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
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
pub(crate) mod __cpi_client_accounts_deposit_locked {
    use super::*;
    pub struct DepositLocked<'info> {
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registry: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for DepositLocked<'info> {
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vesting_vault),
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member_vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registry),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for DepositLocked<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vesting_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.depositor_authority,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registry));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos
        }
    }
}
pub struct Stake<'info> {
    # [account (has_one = pool_mint , has_one = reward_event_q)]
    registrar: Box<Account<'info, Registrar>>,
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account(mut)]
    pool_mint: Account<'info, Mint>,
    # [account (mut , has_one = beneficiary , has_one = registrar)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("BalanceSandbox::from(&balances) == member.balances")]
    balances: BalanceSandboxAccounts<'info>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref ()] , bump = registrar . nonce)]
    registrar_signer: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for Stake<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let reward_event_q: Box<anchor_lang::Account<RewardQueue>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pool_mint: anchor_lang::Account<Mint> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances: BalanceSandboxAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registrar_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &registrar.pool_mint != pool_mint.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &registrar.reward_event_q != reward_event_q.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !pool_mint.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(BalanceSandbox::from(&balances) == member.balances) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                &[registrar.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if registrar_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(Stake {
            registrar,
            reward_event_q,
            pool_mint,
            member,
            beneficiary,
            balances,
            member_signer,
            registrar_signer,
            clock,
            token_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Stake<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.reward_event_q.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.balances.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.registrar_signer.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Stake<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.reward_event_q.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.balances.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.registrar_signer.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Stake<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)?;
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
        anchor_lang::AccountsExit::exit(&self.balances, program_id)?;
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
pub(crate) mod __client_accounts_stake {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct Stake {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub reward_event_q: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances: __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub registrar_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for Stake
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts:
            borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.balances, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.registrar_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Stake {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.reward_event_q,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pool_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
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
pub(crate) mod __cpi_client_accounts_stake {
    use super::*;
    pub use __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct Stake<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub reward_event_q: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances: __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registrar_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Stake<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.reward_event_q),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pool_mint),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Stake<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.reward_event_q,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.balances,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.registrar_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos
        }
    }
}
pub struct StartUnstake<'info> {
    # [account (has_one = reward_event_q , has_one = pool_mint)]
    registrar: Box<Account<'info, Registrar>>,
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    #[account(zero)]
    pending_withdrawal: Box<Account<'info, PendingWithdrawal>>,
    # [account (has_one = beneficiary , has_one = registrar)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("BalanceSandbox::from(&balances) == member.balances")]
    balances: BalanceSandboxAccounts<'info>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    rent: Sysvar<'info, Rent>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for StartUnstake<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let reward_event_q: Box<anchor_lang::Account<RewardQueue>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pool_mint: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pending_withdrawal = &accounts[0];
        *accounts = &accounts[1..];
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances: BalanceSandboxAccounts<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &registrar.reward_event_q != reward_event_q.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &registrar.pool_mint != pool_mint.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !pool_mint.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __anchor_rent = Rent::get()?;
        let pending_withdrawal: Box<anchor_lang::Account<PendingWithdrawal>> = {
            let mut __data: &[u8] = &pending_withdrawal.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(
                &pending_withdrawal,
            )?)
        };
        if !pending_withdrawal.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            pending_withdrawal.to_account_info().lamports(),
            pending_withdrawal.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(BalanceSandbox::from(&balances) == member.balances) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(StartUnstake {
            registrar,
            reward_event_q,
            pool_mint,
            pending_withdrawal,
            member,
            beneficiary,
            balances,
            member_signer,
            token_program,
            clock,
            rent,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for StartUnstake<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.reward_event_q.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.pending_withdrawal.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.balances.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for StartUnstake<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.reward_event_q.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.pending_withdrawal.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.balances.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for StartUnstake<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)?;
        anchor_lang::AccountsExit::exit(&self.pending_withdrawal, program_id)?;
        anchor_lang::AccountsExit::exit(&self.balances, program_id)?;
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
pub(crate) mod __client_accounts_start_unstake {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct StartUnstake {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub reward_event_q: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub pending_withdrawal: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances: __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for StartUnstake
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        __client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts:
            borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.pending_withdrawal, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.balances, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for StartUnstake {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.reward_event_q,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pool_mint,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pending_withdrawal,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
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
                    self.clock, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
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
pub(crate) mod __cpi_client_accounts_start_unstake {
    use super::*;
    pub use __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts;
    pub struct StartUnstake<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub reward_event_q: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pending_withdrawal: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances: __cpi_client_accounts_balance_sandbox_accounts::BalanceSandboxAccounts<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for StartUnstake<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.reward_event_q),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pool_mint),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pending_withdrawal),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.extend(self.balances.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
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
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for StartUnstake<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.reward_event_q,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.pending_withdrawal,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(
                &self.balances,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos
        }
    }
}
pub struct EndUnstake<'info> {
    registrar: Box<Account<'info, Registrar>>,
    # [account (has_one = registrar , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    # [account (mut , has_one = registrar , has_one = member , "!pending_withdrawal.burned")]
    pending_withdrawal: Box<Account<'info, PendingWithdrawal>>,
    #[account(mut)]
    vault: AccountInfo<'info>,
    #[account(mut)]
    vault_pw: AccountInfo<'info>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for EndUnstake<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pending_withdrawal: Box<anchor_lang::Account<PendingWithdrawal>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault_pw: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !pending_withdrawal.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &pending_withdrawal.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &pending_withdrawal.member != member.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !(!pending_withdrawal.burned) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !vault_pw.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(EndUnstake {
            registrar,
            member,
            beneficiary,
            pending_withdrawal,
            vault,
            vault_pw,
            member_signer,
            clock,
            token_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for EndUnstake<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.pending_withdrawal.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vault_pw.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for EndUnstake<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.pending_withdrawal.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vault_pw.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for EndUnstake<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.pending_withdrawal, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault_pw, program_id)?;
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
pub(crate) mod __client_accounts_end_unstake {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct EndUnstake {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub pending_withdrawal: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault_pw: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for EndUnstake
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
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.pending_withdrawal, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vault_pw, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for EndUnstake {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.pending_withdrawal,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault_pw,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.clock, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
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
pub(crate) mod __cpi_client_accounts_end_unstake {
    use super::*;
    pub struct EndUnstake<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pending_withdrawal: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault_pw: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for EndUnstake<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.pending_withdrawal),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault_pw),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for EndUnstake<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.pending_withdrawal,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault_pw));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos
        }
    }
}
pub struct Withdraw<'info> {
    registrar: Box<Account<'info, Registrar>>,
    # [account (has_one = registrar , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, "vault.to_account_info().key == &member.balances.vault")]
    vault: Account<'info, TokenAccount>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
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
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vault.to_account_info().key == &member.balances.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !depositor.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(Withdraw {
            registrar,
            member,
            beneficiary,
            vault,
            member_signer,
            depositor,
            token_program,
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
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.depositor.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
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
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.depositor.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
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
pub(crate) mod __client_accounts_withdraw {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct Withdraw {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
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
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
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
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
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
                    self.member_signer,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.depositor,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
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
pub(crate) mod __cpi_client_accounts_withdraw {
    use super::*;
    pub struct Withdraw<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Withdraw<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
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
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.depositor),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
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
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos
        }
    }
}
pub struct WithdrawLocked<'info> {
    #[account(
        "vesting.to_account_info().owner == &registry.lockup_program",
        "vesting.beneficiary == member.beneficiary"
    )]
    vesting: Account<'info, Vesting>,
    #[account(mut, "vesting_vault.key == &vesting.vault")]
    vesting_vault: AccountInfo<'info>,
    #[account(signer)]
    vesting_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    #[account(
        mut,
        "member_vault.to_account_info().key == &member.balances_locked.vault"
    )]
    member_vault: Account<'info, TokenAccount>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , member . to_account_info () . key . as_ref () ,] , bump = member . nonce)]
    member_signer: AccountInfo<'info>,
    # [account (seeds = [b"the-state" . as_ref ()] , bump = 0)]
    registry: Box<Account<'info, Registry>>,
    registrar: Box<Account<'info, Registrar>>,
    # [account (has_one = registrar , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for WithdrawLocked<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let vesting: anchor_lang::Account<Vesting> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting_vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vesting_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry: Box<anchor_lang::Account<Registry>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !(vesting.to_account_info().owner == &registry.lockup_program) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(vesting.beneficiary == member.beneficiary) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vesting_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(vesting_vault.key == &vesting.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !vesting_signer.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !member_vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(member_vault.to_account_info().key == &member.balances_locked.vault) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                member.to_account_info().key.as_ref(),
                &[member.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if member_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        let __program_signer =
            Pubkey::create_program_address(&[b"the-state".as_ref(), &[0][..]][..], program_id)
                .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if registry.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        Ok(WithdrawLocked {
            vesting,
            vesting_vault,
            vesting_signer,
            token_program,
            member_vault,
            member_signer,
            registry,
            registrar,
            member,
            beneficiary,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawLocked<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.vesting.to_account_infos());
        account_infos.extend(self.vesting_vault.to_account_infos());
        account_infos.extend(self.vesting_signer.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.member_vault.to_account_infos());
        account_infos.extend(self.member_signer.to_account_infos());
        account_infos.extend(self.registry.to_account_infos());
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for WithdrawLocked<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.vesting.to_account_metas(None));
        account_metas.extend(self.vesting_vault.to_account_metas(None));
        account_metas.extend(self.vesting_signer.to_account_metas(Some(true)));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.member_vault.to_account_metas(None));
        account_metas.extend(self.member_signer.to_account_metas(None));
        account_metas.extend(self.registry.to_account_metas(None));
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for WithdrawLocked<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vesting_vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.member_vault, program_id)?;
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
pub(crate) mod __client_accounts_withdraw_locked {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct WithdrawLocked {
        pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vesting_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub member_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub registry: anchor_lang::solana_program::pubkey::Pubkey,
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for WithdrawLocked
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
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.member_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.member_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.registry, writer)?;
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for WithdrawLocked {
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vesting_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vesting_signer,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.token_program,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member_vault,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registry,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.member,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
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
pub(crate) mod __cpi_client_accounts_withdraw_locked {
    use super::*;
    pub struct WithdrawLocked<'info> {
        pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vesting_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registry: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WithdrawLocked<'info> {
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
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vesting_vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vesting_signer),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.token_program),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member_vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registry),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.member),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawLocked<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vesting_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vesting_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.member_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registry));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos
        }
    }
}
pub struct DropReward<'info> {
    # [account (has_one = reward_event_q , has_one = pool_mint)]
    registrar: Box<Account<'info, Registrar>>,
    #[account(mut)]
    reward_event_q: Box<Account<'info, RewardQueue>>,
    pool_mint: Account<'info, Mint>,
    #[account(zero)]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vendor_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(signer)]
    depositor_authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    rent: Sysvar<'info, Rent>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for DropReward<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let reward_event_q: Box<anchor_lang::Account<RewardQueue>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let pool_mint: anchor_lang::Account<Mint> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor = &accounts[0];
        *accounts = &accounts[1..];
        let vendor_vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let depositor_authority: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let rent: Sysvar<Rent> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if &registrar.reward_event_q != reward_event_q.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &registrar.pool_mint != pool_mint.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !reward_event_q.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __anchor_rent = Rent::get()?;
        let vendor: Box<anchor_lang::Account<RewardVendor>> = {
            let mut __data: &[u8] = &vendor.try_borrow_data()?;
            let mut __disc_bytes = [0u8; 8];
            __disc_bytes.copy_from_slice(&__data[..8]);
            let __discriminator = u64::from_le_bytes(__disc_bytes);
            if __discriminator != 0 {
                return Err(anchor_lang::__private::ErrorCode::ConstraintZero.into());
            }
            Box::new(anchor_lang::Account::try_from_unchecked(&vendor)?)
        };
        if !vendor.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !__anchor_rent.is_exempt(
            vendor.to_account_info().lamports(),
            vendor.to_account_info().try_data_len()?,
        ) {
            return Err(anchor_lang::__private::ErrorCode::ConstraintRentExempt.into());
        }
        if !vendor_vault.to_account_info().is_writable {
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
        Ok(DropReward {
            registrar,
            reward_event_q,
            pool_mint,
            vendor,
            vendor_vault,
            depositor,
            depositor_authority,
            token_program,
            clock,
            rent,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for DropReward<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.reward_event_q.to_account_infos());
        account_infos.extend(self.pool_mint.to_account_infos());
        account_infos.extend(self.vendor.to_account_infos());
        account_infos.extend(self.vendor_vault.to_account_infos());
        account_infos.extend(self.depositor.to_account_infos());
        account_infos.extend(self.depositor_authority.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos.extend(self.rent.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for DropReward<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.reward_event_q.to_account_metas(None));
        account_metas.extend(self.pool_mint.to_account_metas(None));
        account_metas.extend(self.vendor.to_account_metas(None));
        account_metas.extend(self.vendor_vault.to_account_metas(None));
        account_metas.extend(self.depositor.to_account_metas(None));
        account_metas.extend(self.depositor_authority.to_account_metas(Some(true)));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas.extend(self.rent.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for DropReward<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.reward_event_q, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vendor, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vendor_vault, program_id)?;
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
pub(crate) mod __client_accounts_drop_reward {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct DropReward {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub reward_event_q: anchor_lang::solana_program::pubkey::Pubkey,
        pub pool_mint: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor_vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor: anchor_lang::solana_program::pubkey::Pubkey,
        pub depositor_authority: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
        pub rent: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for DropReward
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
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor, writer)?;
            borsh::BorshSerialize::serialize(&self.depositor_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            borsh::BorshSerialize::serialize(&self.rent, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for DropReward {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.reward_event_q,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.pool_mint,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vendor,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vendor_vault,
                false,
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
                    self.clock, false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.rent, false,
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
pub(crate) mod __cpi_client_accounts_drop_reward {
    use super::*;
    pub struct DropReward<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub reward_event_q: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub depositor_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for DropReward<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.reward_event_q),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.pool_mint),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vendor),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vendor_vault),
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
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.rent),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for DropReward<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.reward_event_q,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.pool_mint));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vendor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vendor_vault,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.depositor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.depositor_authority,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
            account_infos
        }
    }
}
impl<'info> DropReward<'info> {
    fn accounts(ctx: &Context<DropReward>, nonce: u8) -> ProgramResult {
        let vendor_signer = Pubkey::create_program_address(
            &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.vendor.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| ErrorCode::InvalidNonce)?;
        if vendor_signer != ctx.accounts.vendor_vault.owner {
            return Err(ErrorCode::InvalidVaultOwner.into());
        }
        Ok(())
    }
}
pub struct ClaimReward<'info> {
    cmn: ClaimRewardCommon<'info>,
    #[account(mut)]
    to: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ClaimReward<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let cmn: ClaimRewardCommon<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let to: AccountInfo = anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !to.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        Ok(ClaimReward { cmn, to })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimReward<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.cmn.to_account_infos());
        account_infos.extend(self.to.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ClaimReward<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.cmn.to_account_metas(None));
        account_metas.extend(self.to.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ClaimReward<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.cmn, program_id)?;
        anchor_lang::AccountsExit::exit(&self.to, program_id)?;
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
pub(crate) mod __client_accounts_claim_reward {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_claim_reward_common::ClaimRewardCommon;
    pub struct ClaimReward {
        pub cmn: __client_accounts_claim_reward_common::ClaimRewardCommon,
        pub to: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ClaimReward
    where
        __client_accounts_claim_reward_common::ClaimRewardCommon: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.cmn, writer)?;
            borsh::BorshSerialize::serialize(&self.to, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ClaimReward {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.cmn.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.to, false,
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
pub(crate) mod __cpi_client_accounts_claim_reward {
    use super::*;
    pub use __cpi_client_accounts_claim_reward_common::ClaimRewardCommon;
    pub struct ClaimReward<'info> {
        pub cmn: __cpi_client_accounts_claim_reward_common::ClaimRewardCommon<'info>,
        pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ClaimReward<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.cmn.to_account_metas(None));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.to),
                false,
            ));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimReward<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.cmn));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.to));
            account_infos
        }
    }
}
# [instruction (registry_bump : u8)]
pub struct ClaimRewardLocked<'info> {
    cmn: ClaimRewardCommon<'info>,
    # [account (seeds = [b"registry" . as_ref ()] , bump = registry_bump)]
    registry: Box<Account<'info, Registry>>,
    #[account("lockup_program.key == &registry.lockup_program")]
    lockup_program: AccountInfo<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ClaimRewardLocked<'info>
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
            registry_bump: u8,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.registry_bump, writer)?;
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
                    registry_bump: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { registry_bump } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
        let cmn: ClaimRewardCommon<'info> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let registry: Box<anchor_lang::Account<Registry>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let lockup_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let __program_signer = Pubkey::create_program_address(
            &[b"registry".as_ref(), &[registry_bump][..]][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if registry.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(lockup_program.key == &registry.lockup_program) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(ClaimRewardLocked {
            cmn,
            registry,
            lockup_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimRewardLocked<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.cmn.to_account_infos());
        account_infos.extend(self.registry.to_account_infos());
        account_infos.extend(self.lockup_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ClaimRewardLocked<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.cmn.to_account_metas(None));
        account_metas.extend(self.registry.to_account_metas(None));
        account_metas.extend(self.lockup_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ClaimRewardLocked<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.cmn, program_id)?;
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
pub(crate) mod __client_accounts_claim_reward_locked {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub use __client_accounts_claim_reward_common::ClaimRewardCommon;
    pub struct ClaimRewardLocked {
        pub cmn: __client_accounts_claim_reward_common::ClaimRewardCommon,
        pub registry: anchor_lang::solana_program::pubkey::Pubkey,
        pub lockup_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ClaimRewardLocked
    where
        __client_accounts_claim_reward_common::ClaimRewardCommon: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.cmn, writer)?;
            borsh::BorshSerialize::serialize(&self.registry, writer)?;
            borsh::BorshSerialize::serialize(&self.lockup_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ClaimRewardLocked {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.cmn.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registry,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.lockup_program,
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
pub(crate) mod __cpi_client_accounts_claim_reward_locked {
    use super::*;
    pub use __cpi_client_accounts_claim_reward_common::ClaimRewardCommon;
    pub struct ClaimRewardLocked<'info> {
        pub cmn: __cpi_client_accounts_claim_reward_common::ClaimRewardCommon<'info>,
        pub registry: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub lockup_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ClaimRewardLocked<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.cmn.to_account_metas(None));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registry),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.lockup_program),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimRewardLocked<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(anchor_lang::ToAccountInfos::to_account_infos(&self.cmn));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registry));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.lockup_program,
            ));
            account_infos
        }
    }
}
pub struct ClaimRewardCommon<'info> {
    registrar: Box<Account<'info, Registrar>>,
    # [account (mut , has_one = registrar , has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("balances_spt.key() == member.balances.spt")]
    balances_spt: Account<'info, TokenAccount>,
    #[account("balances_locked_spt.key() == member.balances_locked.spt")]
    balances_locked_spt: Account<'info, TokenAccount>,
    # [account (has_one = registrar , has_one = vault)]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vault: AccountInfo<'info>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , vendor . to_account_info () . key . as_ref () ,] , bump = vendor . nonce)]
    vendor_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ClaimRewardCommon<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let member: Box<anchor_lang::Account<Member>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let beneficiary: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances_spt: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let balances_locked_spt: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor: Box<anchor_lang::Account<RewardVendor>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !member.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &member.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &member.beneficiary != beneficiary.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !beneficiary.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !(balances_spt.key() == member.balances.spt) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if !(balances_locked_spt.key() == member.balances_locked.spt) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        if &vendor.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &vendor.vault != vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                vendor.to_account_info().key.as_ref(),
                &[vendor.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if vendor_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(ClaimRewardCommon {
            registrar,
            member,
            beneficiary,
            balances_spt,
            balances_locked_spt,
            vendor,
            vault,
            vendor_signer,
            token_program,
            clock,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimRewardCommon<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.member.to_account_infos());
        account_infos.extend(self.beneficiary.to_account_infos());
        account_infos.extend(self.balances_spt.to_account_infos());
        account_infos.extend(self.balances_locked_spt.to_account_infos());
        account_infos.extend(self.vendor.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vendor_signer.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ClaimRewardCommon<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.member.to_account_metas(None));
        account_metas.extend(self.beneficiary.to_account_metas(Some(true)));
        account_metas.extend(self.balances_spt.to_account_metas(None));
        account_metas.extend(self.balances_locked_spt.to_account_metas(None));
        account_metas.extend(self.vendor.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vendor_signer.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ClaimRewardCommon<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.member, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
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
pub(crate) mod __client_accounts_claim_reward_common {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct ClaimRewardCommon {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub member: anchor_lang::solana_program::pubkey::Pubkey,
        pub beneficiary: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances_spt: anchor_lang::solana_program::pubkey::Pubkey,
        pub balances_locked_spt: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ClaimRewardCommon
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
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.member, writer)?;
            borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
            borsh::BorshSerialize::serialize(&self.balances_spt, writer)?;
            borsh::BorshSerialize::serialize(&self.balances_locked_spt, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ClaimRewardCommon {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.member,
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.beneficiary,
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.balances_spt,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.balances_locked_spt,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vendor,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vendor_signer,
                    false,
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
pub(crate) mod __cpi_client_accounts_claim_reward_common {
    use super::*;
    pub struct ClaimRewardCommon<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub member: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub beneficiary: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances_spt: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub balances_locked_spt: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ClaimRewardCommon<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.member),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.beneficiary),
                    true,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.balances_spt),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.balances_locked_spt),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vendor),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vendor_signer),
                    false,
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
                    anchor_lang::Key::key(&self.clock),
                    false,
                ),
            );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimRewardCommon<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.member));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.beneficiary,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.balances_spt,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.balances_locked_spt,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vendor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vendor_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
pub struct ExpireReward<'info> {
    registrar: Box<Account<'info, Registrar>>,
    # [account (mut , has_one = registrar , has_one = vault , has_one = expiry_receiver)]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vault: Account<'info, TokenAccount>,
    # [account (seeds = [registrar . to_account_info () . key . as_ref () , vendor . to_account_info () . key . as_ref () ,] , bump = vendor . nonce)]
    vendor_signer: AccountInfo<'info>,
    #[account(signer)]
    expiry_receiver: AccountInfo<'info>,
    #[account(mut)]
    expiry_receiver_token: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ExpireReward<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
    ) -> std::result::Result<Self, anchor_lang::solana_program::program_error::ProgramError> {
        let registrar: Box<anchor_lang::Account<Registrar>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor: Box<anchor_lang::Account<RewardVendor>> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vault: anchor_lang::Account<TokenAccount> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let vendor_signer: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let expiry_receiver: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let expiry_receiver_token: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let token_program: AccountInfo =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        let clock: Sysvar<Clock> =
            anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data)?;
        if !vendor.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if &vendor.registrar != registrar.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &vendor.vault != vault.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if &vendor.expiry_receiver != expiry_receiver.to_account_info().key {
            return Err(anchor_lang::__private::ErrorCode::ConstraintHasOne.into());
        }
        if !vault.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        let __program_signer = Pubkey::create_program_address(
            &[
                registrar.to_account_info().key.as_ref(),
                vendor.to_account_info().key.as_ref(),
                &[vendor.nonce][..],
            ][..],
            program_id,
        )
        .map_err(|_| anchor_lang::__private::ErrorCode::ConstraintSeeds)?;
        if vendor_signer.to_account_info().key != &__program_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSeeds.into());
        }
        if !expiry_receiver.is_signer {
            return Err(anchor_lang::__private::ErrorCode::ConstraintSigner.into());
        }
        if !expiry_receiver_token.to_account_info().is_writable {
            return Err(anchor_lang::__private::ErrorCode::ConstraintMut.into());
        }
        if !(token_program.key == &token::ID) {
            return Err(anchor_lang::__private::ErrorCode::Deprecated.into());
        }
        Ok(ExpireReward {
            registrar,
            vendor,
            vault,
            vendor_signer,
            expiry_receiver,
            expiry_receiver_token,
            token_program,
            clock,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ExpireReward<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.registrar.to_account_infos());
        account_infos.extend(self.vendor.to_account_infos());
        account_infos.extend(self.vault.to_account_infos());
        account_infos.extend(self.vendor_signer.to_account_infos());
        account_infos.extend(self.expiry_receiver.to_account_infos());
        account_infos.extend(self.expiry_receiver_token.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.clock.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ExpireReward<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.registrar.to_account_metas(None));
        account_metas.extend(self.vendor.to_account_metas(None));
        account_metas.extend(self.vault.to_account_metas(None));
        account_metas.extend(self.vendor_signer.to_account_metas(None));
        account_metas.extend(self.expiry_receiver.to_account_metas(Some(true)));
        account_metas.extend(self.expiry_receiver_token.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.clock.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ExpireReward<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::solana_program::entrypoint::ProgramResult {
        anchor_lang::AccountsExit::exit(&self.vendor, program_id)?;
        anchor_lang::AccountsExit::exit(&self.vault, program_id)?;
        anchor_lang::AccountsExit::exit(&self.expiry_receiver_token, program_id)?;
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
pub(crate) mod __client_accounts_expire_reward {
    use super::*;
    use anchor_lang::prelude::borsh;
    pub struct ExpireReward {
        pub registrar: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor: anchor_lang::solana_program::pubkey::Pubkey,
        pub vault: anchor_lang::solana_program::pubkey::Pubkey,
        pub vendor_signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub expiry_receiver: anchor_lang::solana_program::pubkey::Pubkey,
        pub expiry_receiver_token: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
        pub clock: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ExpireReward
    where
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
            borsh::BorshSerialize::serialize(&self.registrar, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor, writer)?;
            borsh::BorshSerialize::serialize(&self.vault, writer)?;
            borsh::BorshSerialize::serialize(&self.vendor_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.expiry_receiver, writer)?;
            borsh::BorshSerialize::serialize(&self.expiry_receiver_token, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.clock, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ExpireReward {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.registrar,
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vendor,
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.vault, false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.vendor_signer,
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    self.expiry_receiver,
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                self.expiry_receiver_token,
                false,
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
pub(crate) mod __cpi_client_accounts_expire_reward {
    use super::*;
    pub struct ExpireReward<'info> {
        pub registrar: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vendor_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub expiry_receiver: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub expiry_receiver_token: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ExpireReward<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.registrar),
                    false,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vendor),
                false,
            ));
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.vault),
                false,
            ));
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.vendor_signer),
                    false,
                ),
            );
            account_metas.push(
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    anchor_lang::Key::key(&self.expiry_receiver),
                    true,
                ),
            );
            account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                anchor_lang::Key::key(&self.expiry_receiver_token),
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
    impl<'info> anchor_lang::ToAccountInfos<'info> for ExpireReward<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.registrar));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vendor));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.vendor_signer,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.expiry_receiver,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.expiry_receiver_token,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                &self.token_program,
            ));
            account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
            account_infos
        }
    }
}
pub struct Registrar {
    /// Priviledged account.
    pub authority: Pubkey,
    /// Nonce to derive the program-derived address owning the vaults.
    pub nonce: u8,
    /// Number of seconds that must pass for a withdrawal to complete.
    pub withdrawal_timelock: i64,
    /// Global event queue for reward vendoring.
    pub reward_event_q: Pubkey,
    /// Mint of the tokens that can be staked.
    pub mint: Pubkey,
    /// Staking pool token mint.
    pub pool_mint: Pubkey,
    /// The amount of tokens (not decimal) that must be staked to get a single
    /// staking pool token.
    pub stake_rate: u64,
}
impl borsh::ser::BorshSerialize for Registrar
where
    Pubkey: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, writer)?;
        borsh::BorshSerialize::serialize(&self.withdrawal_timelock, writer)?;
        borsh::BorshSerialize::serialize(&self.reward_event_q, writer)?;
        borsh::BorshSerialize::serialize(&self.mint, writer)?;
        borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
        borsh::BorshSerialize::serialize(&self.stake_rate, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Registrar
where
    Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            authority: borsh::BorshDeserialize::deserialize(buf)?,
            nonce: borsh::BorshDeserialize::deserialize(buf)?,
            withdrawal_timelock: borsh::BorshDeserialize::deserialize(buf)?,
            reward_event_q: borsh::BorshDeserialize::deserialize(buf)?,
            mint: borsh::BorshDeserialize::deserialize(buf)?,
            pool_mint: borsh::BorshDeserialize::deserialize(buf)?,
            stake_rate: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Registrar {
    #[inline]
    fn clone(&self) -> Registrar {
        match *self {
            Registrar {
                authority: ref __self_0_0,
                nonce: ref __self_0_1,
                withdrawal_timelock: ref __self_0_2,
                reward_event_q: ref __self_0_3,
                mint: ref __self_0_4,
                pool_mint: ref __self_0_5,
                stake_rate: ref __self_0_6,
            } => Registrar {
                authority: ::core::clone::Clone::clone(&(*__self_0_0)),
                nonce: ::core::clone::Clone::clone(&(*__self_0_1)),
                withdrawal_timelock: ::core::clone::Clone::clone(&(*__self_0_2)),
                reward_event_q: ::core::clone::Clone::clone(&(*__self_0_3)),
                mint: ::core::clone::Clone::clone(&(*__self_0_4)),
                pool_mint: ::core::clone::Clone::clone(&(*__self_0_5)),
                stake_rate: ::core::clone::Clone::clone(&(*__self_0_6)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Registrar {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[193, 202, 205, 51, 78, 168, 150, 128])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Registrar {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [193, 202, 205, 51, 78, 168, 150, 128].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[193, 202, 205, 51, 78, 168, 150, 128] != given_disc {
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
impl anchor_lang::Discriminator for Registrar {
    fn discriminator() -> [u8; 8] {
        [193, 202, 205, 51, 78, 168, 150, 128]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Registrar {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct Member {
    /// Registrar the member belongs to.
    pub registrar: Pubkey,
    /// The effective owner of the Member account.
    pub beneficiary: Pubkey,
    /// Arbitrary metadata account owned by any program.
    pub metadata: Pubkey,
    /// Sets of balances owned by the Member.
    pub balances: BalanceSandbox,
    /// Locked balances owned by the Member.
    pub balances_locked: BalanceSandbox,
    /// Next position in the rewards event queue to process.
    pub rewards_cursor: u32,
    /// The clock timestamp of the last time this account staked or switched
    /// entities. Used as a proof to reward vendors that the Member account
    /// was staked at a given point in time.
    pub last_stake_ts: i64,
    /// Signer nonce.
    pub nonce: u8,
}
impl borsh::ser::BorshSerialize for Member
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    BalanceSandbox: borsh::ser::BorshSerialize,
    BalanceSandbox: borsh::ser::BorshSerialize,
    u32: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.registrar, writer)?;
        borsh::BorshSerialize::serialize(&self.beneficiary, writer)?;
        borsh::BorshSerialize::serialize(&self.metadata, writer)?;
        borsh::BorshSerialize::serialize(&self.balances, writer)?;
        borsh::BorshSerialize::serialize(&self.balances_locked, writer)?;
        borsh::BorshSerialize::serialize(&self.rewards_cursor, writer)?;
        borsh::BorshSerialize::serialize(&self.last_stake_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Member
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    BalanceSandbox: borsh::BorshDeserialize,
    BalanceSandbox: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            registrar: borsh::BorshDeserialize::deserialize(buf)?,
            beneficiary: borsh::BorshDeserialize::deserialize(buf)?,
            metadata: borsh::BorshDeserialize::deserialize(buf)?,
            balances: borsh::BorshDeserialize::deserialize(buf)?,
            balances_locked: borsh::BorshDeserialize::deserialize(buf)?,
            rewards_cursor: borsh::BorshDeserialize::deserialize(buf)?,
            last_stake_ts: borsh::BorshDeserialize::deserialize(buf)?,
            nonce: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Member {
    #[inline]
    fn clone(&self) -> Member {
        match *self {
            Member {
                registrar: ref __self_0_0,
                beneficiary: ref __self_0_1,
                metadata: ref __self_0_2,
                balances: ref __self_0_3,
                balances_locked: ref __self_0_4,
                rewards_cursor: ref __self_0_5,
                last_stake_ts: ref __self_0_6,
                nonce: ref __self_0_7,
            } => Member {
                registrar: ::core::clone::Clone::clone(&(*__self_0_0)),
                beneficiary: ::core::clone::Clone::clone(&(*__self_0_1)),
                metadata: ::core::clone::Clone::clone(&(*__self_0_2)),
                balances: ::core::clone::Clone::clone(&(*__self_0_3)),
                balances_locked: ::core::clone::Clone::clone(&(*__self_0_4)),
                rewards_cursor: ::core::clone::Clone::clone(&(*__self_0_5)),
                last_stake_ts: ::core::clone::Clone::clone(&(*__self_0_6)),
                nonce: ::core::clone::Clone::clone(&(*__self_0_7)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Member {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[54, 19, 162, 21, 29, 166, 17, 198])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Member {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [54, 19, 162, 21, 29, 166, 17, 198].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[54, 19, 162, 21, 29, 166, 17, 198] != given_disc {
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
impl anchor_lang::Discriminator for Member {
    fn discriminator() -> [u8; 8] {
        [54, 19, 162, 21, 29, 166, 17, 198]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for Member {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct BalanceSandbox {
    pub spt: Pubkey,
    pub vault: Pubkey,
    pub vault_stake: Pubkey,
    pub vault_pw: Pubkey,
}
impl borsh::ser::BorshSerialize for BalanceSandbox
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.spt, writer)?;
        borsh::BorshSerialize::serialize(&self.vault, writer)?;
        borsh::BorshSerialize::serialize(&self.vault_stake, writer)?;
        borsh::BorshSerialize::serialize(&self.vault_pw, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for BalanceSandbox
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            spt: borsh::BorshDeserialize::deserialize(buf)?,
            vault: borsh::BorshDeserialize::deserialize(buf)?,
            vault_stake: borsh::BorshDeserialize::deserialize(buf)?,
            vault_pw: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for BalanceSandbox {
    #[inline]
    fn default() -> BalanceSandbox {
        BalanceSandbox {
            spt: ::core::default::Default::default(),
            vault: ::core::default::Default::default(),
            vault_stake: ::core::default::Default::default(),
            vault_pw: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for BalanceSandbox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            BalanceSandbox {
                spt: ref __self_0_0,
                vault: ref __self_0_1,
                vault_stake: ref __self_0_2,
                vault_pw: ref __self_0_3,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "BalanceSandbox");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "spt", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "vault", &&(*__self_0_1));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "vault_stake",
                    &&(*__self_0_2),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "vault_pw",
                    &&(*__self_0_3),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for BalanceSandbox {
    #[inline]
    fn clone(&self) -> BalanceSandbox {
        match *self {
            BalanceSandbox {
                spt: ref __self_0_0,
                vault: ref __self_0_1,
                vault_stake: ref __self_0_2,
                vault_pw: ref __self_0_3,
            } => BalanceSandbox {
                spt: ::core::clone::Clone::clone(&(*__self_0_0)),
                vault: ::core::clone::Clone::clone(&(*__self_0_1)),
                vault_stake: ::core::clone::Clone::clone(&(*__self_0_2)),
                vault_pw: ::core::clone::Clone::clone(&(*__self_0_3)),
            },
        }
    }
}
impl ::core::marker::StructuralPartialEq for BalanceSandbox {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for BalanceSandbox {
    #[inline]
    fn eq(&self, other: &BalanceSandbox) -> bool {
        match *other {
            BalanceSandbox {
                spt: ref __self_1_0,
                vault: ref __self_1_1,
                vault_stake: ref __self_1_2,
                vault_pw: ref __self_1_3,
            } => match *self {
                BalanceSandbox {
                    spt: ref __self_0_0,
                    vault: ref __self_0_1,
                    vault_stake: ref __self_0_2,
                    vault_pw: ref __self_0_3,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &BalanceSandbox) -> bool {
        match *other {
            BalanceSandbox {
                spt: ref __self_1_0,
                vault: ref __self_1_1,
                vault_stake: ref __self_1_2,
                vault_pw: ref __self_1_3,
            } => match *self {
                BalanceSandbox {
                    spt: ref __self_0_0,
                    vault: ref __self_0_1,
                    vault_stake: ref __self_0_2,
                    vault_pw: ref __self_0_3,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                }
            },
        }
    }
}
pub struct PendingWithdrawal {
    /// Registrar this account belongs to.
    pub registrar: Pubkey,
    /// Member this account belongs to.
    pub member: Pubkey,
    /// One time token. True if the withdrawal has been completed.
    pub burned: bool,
    /// The pool being withdrawn from.
    pub pool: Pubkey,
    /// Unix timestamp when this account was initialized.
    pub start_ts: i64,
    /// Timestamp when the pending withdrawal completes.
    pub end_ts: i64,
    /// The number of tokens redeemed from the staking pool.
    pub amount: u64,
    /// True if the withdrawal applies to locked balances.
    pub locked: bool,
}
impl borsh::ser::BorshSerialize for PendingWithdrawal
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    bool: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    bool: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.registrar, writer)?;
        borsh::BorshSerialize::serialize(&self.member, writer)?;
        borsh::BorshSerialize::serialize(&self.burned, writer)?;
        borsh::BorshSerialize::serialize(&self.pool, writer)?;
        borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.end_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.amount, writer)?;
        borsh::BorshSerialize::serialize(&self.locked, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for PendingWithdrawal
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    bool: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    bool: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            registrar: borsh::BorshDeserialize::deserialize(buf)?,
            member: borsh::BorshDeserialize::deserialize(buf)?,
            burned: borsh::BorshDeserialize::deserialize(buf)?,
            pool: borsh::BorshDeserialize::deserialize(buf)?,
            start_ts: borsh::BorshDeserialize::deserialize(buf)?,
            end_ts: borsh::BorshDeserialize::deserialize(buf)?,
            amount: borsh::BorshDeserialize::deserialize(buf)?,
            locked: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for PendingWithdrawal {
    #[inline]
    fn clone(&self) -> PendingWithdrawal {
        match *self {
            PendingWithdrawal {
                registrar: ref __self_0_0,
                member: ref __self_0_1,
                burned: ref __self_0_2,
                pool: ref __self_0_3,
                start_ts: ref __self_0_4,
                end_ts: ref __self_0_5,
                amount: ref __self_0_6,
                locked: ref __self_0_7,
            } => PendingWithdrawal {
                registrar: ::core::clone::Clone::clone(&(*__self_0_0)),
                member: ::core::clone::Clone::clone(&(*__self_0_1)),
                burned: ::core::clone::Clone::clone(&(*__self_0_2)),
                pool: ::core::clone::Clone::clone(&(*__self_0_3)),
                start_ts: ::core::clone::Clone::clone(&(*__self_0_4)),
                end_ts: ::core::clone::Clone::clone(&(*__self_0_5)),
                amount: ::core::clone::Clone::clone(&(*__self_0_6)),
                locked: ::core::clone::Clone::clone(&(*__self_0_7)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for PendingWithdrawal {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[61, 103, 179, 177, 148, 199, 63, 171])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for PendingWithdrawal {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [61, 103, 179, 177, 148, 199, 63, 171].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[61, 103, 179, 177, 148, 199, 63, 171] != given_disc {
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
impl anchor_lang::Discriminator for PendingWithdrawal {
    fn discriminator() -> [u8; 8] {
        [61, 103, 179, 177, 148, 199, 63, 171]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for PendingWithdrawal {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct RewardQueue {
    head: u32,
    tail: u32,
    events: Vec<RewardEvent>,
}
impl borsh::ser::BorshSerialize for RewardQueue
where
    u32: borsh::ser::BorshSerialize,
    u32: borsh::ser::BorshSerialize,
    Vec<RewardEvent>: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.head, writer)?;
        borsh::BorshSerialize::serialize(&self.tail, writer)?;
        borsh::BorshSerialize::serialize(&self.events, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for RewardQueue
where
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    Vec<RewardEvent>: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            head: borsh::BorshDeserialize::deserialize(buf)?,
            tail: borsh::BorshDeserialize::deserialize(buf)?,
            events: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for RewardQueue {
    #[inline]
    fn clone(&self) -> RewardQueue {
        match *self {
            RewardQueue {
                head: ref __self_0_0,
                tail: ref __self_0_1,
                events: ref __self_0_2,
            } => RewardQueue {
                head: ::core::clone::Clone::clone(&(*__self_0_0)),
                tail: ::core::clone::Clone::clone(&(*__self_0_1)),
                events: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for RewardQueue {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[162, 152, 119, 24, 192, 185, 100, 167])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for RewardQueue {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [162, 152, 119, 24, 192, 185, 100, 167].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[162, 152, 119, 24, 192, 185, 100, 167] != given_disc {
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
impl anchor_lang::Discriminator for RewardQueue {
    fn discriminator() -> [u8; 8] {
        [162, 152, 119, 24, 192, 185, 100, 167]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for RewardQueue {
    fn owner() -> Pubkey {
        crate::ID
    }
}
impl RewardQueue {
    pub fn append(&mut self, event: RewardEvent) -> Result<u32> {
        let cursor = self.head;
        let h_idx = self.index_of(self.head);
        self.events[h_idx] = event;
        let is_full = self.index_of(self.head + 1) == self.index_of(self.tail);
        if is_full {
            self.tail += 1;
        }
        self.head += 1;
        Ok(cursor)
    }
    pub fn index_of(&self, counter: u32) -> usize {
        counter as usize % self.capacity()
    }
    pub fn capacity(&self) -> usize {
        self.events.len()
    }
    pub fn get(&self, cursor: u32) -> &RewardEvent {
        &self.events[cursor as usize % self.capacity()]
    }
    pub fn head(&self) -> u32 {
        self.head
    }
    pub fn tail(&self) -> u32 {
        self.tail
    }
}
pub struct RewardEvent {
    vendor: Pubkey,
    ts: i64,
    locked: bool,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for RewardEvent {
    #[inline]
    fn default() -> RewardEvent {
        RewardEvent {
            vendor: ::core::default::Default::default(),
            ts: ::core::default::Default::default(),
            locked: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for RewardEvent {
    #[inline]
    fn clone(&self) -> RewardEvent {
        {
            let _: ::core::clone::AssertParamIsClone<Pubkey>;
            let _: ::core::clone::AssertParamIsClone<i64>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for RewardEvent {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for RewardEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            RewardEvent {
                vendor: ref __self_0_0,
                ts: ref __self_0_1,
                locked: ref __self_0_2,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "RewardEvent");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "vendor", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "ts", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "locked", &&(*__self_0_2));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl borsh::ser::BorshSerialize for RewardEvent
where
    Pubkey: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    bool: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.vendor, writer)?;
        borsh::BorshSerialize::serialize(&self.ts, writer)?;
        borsh::BorshSerialize::serialize(&self.locked, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for RewardEvent
where
    Pubkey: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    bool: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            vendor: borsh::BorshDeserialize::deserialize(buf)?,
            ts: borsh::BorshDeserialize::deserialize(buf)?,
            locked: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
pub struct RewardVendor {
    pub registrar: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub nonce: u8,
    pub pool_token_supply: u64,
    pub reward_event_q_cursor: u32,
    pub start_ts: i64,
    pub expiry_ts: i64,
    pub expiry_receiver: Pubkey,
    pub from: Pubkey,
    pub total: u64,
    pub expired: bool,
    pub kind: RewardVendorKind,
}
impl borsh::ser::BorshSerialize for RewardVendor
where
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    u32: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
    bool: borsh::ser::BorshSerialize,
    RewardVendorKind: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.registrar, writer)?;
        borsh::BorshSerialize::serialize(&self.vault, writer)?;
        borsh::BorshSerialize::serialize(&self.mint, writer)?;
        borsh::BorshSerialize::serialize(&self.nonce, writer)?;
        borsh::BorshSerialize::serialize(&self.pool_token_supply, writer)?;
        borsh::BorshSerialize::serialize(&self.reward_event_q_cursor, writer)?;
        borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_ts, writer)?;
        borsh::BorshSerialize::serialize(&self.expiry_receiver, writer)?;
        borsh::BorshSerialize::serialize(&self.from, writer)?;
        borsh::BorshSerialize::serialize(&self.total, writer)?;
        borsh::BorshSerialize::serialize(&self.expired, writer)?;
        borsh::BorshSerialize::serialize(&self.kind, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for RewardVendor
where
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    bool: borsh::BorshDeserialize,
    RewardVendorKind: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            registrar: borsh::BorshDeserialize::deserialize(buf)?,
            vault: borsh::BorshDeserialize::deserialize(buf)?,
            mint: borsh::BorshDeserialize::deserialize(buf)?,
            nonce: borsh::BorshDeserialize::deserialize(buf)?,
            pool_token_supply: borsh::BorshDeserialize::deserialize(buf)?,
            reward_event_q_cursor: borsh::BorshDeserialize::deserialize(buf)?,
            start_ts: borsh::BorshDeserialize::deserialize(buf)?,
            expiry_ts: borsh::BorshDeserialize::deserialize(buf)?,
            expiry_receiver: borsh::BorshDeserialize::deserialize(buf)?,
            from: borsh::BorshDeserialize::deserialize(buf)?,
            total: borsh::BorshDeserialize::deserialize(buf)?,
            expired: borsh::BorshDeserialize::deserialize(buf)?,
            kind: borsh::BorshDeserialize::deserialize(buf)?,
        })
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for RewardVendor {
    #[inline]
    fn clone(&self) -> RewardVendor {
        match *self {
            RewardVendor {
                registrar: ref __self_0_0,
                vault: ref __self_0_1,
                mint: ref __self_0_2,
                nonce: ref __self_0_3,
                pool_token_supply: ref __self_0_4,
                reward_event_q_cursor: ref __self_0_5,
                start_ts: ref __self_0_6,
                expiry_ts: ref __self_0_7,
                expiry_receiver: ref __self_0_8,
                from: ref __self_0_9,
                total: ref __self_0_10,
                expired: ref __self_0_11,
                kind: ref __self_0_12,
            } => RewardVendor {
                registrar: ::core::clone::Clone::clone(&(*__self_0_0)),
                vault: ::core::clone::Clone::clone(&(*__self_0_1)),
                mint: ::core::clone::Clone::clone(&(*__self_0_2)),
                nonce: ::core::clone::Clone::clone(&(*__self_0_3)),
                pool_token_supply: ::core::clone::Clone::clone(&(*__self_0_4)),
                reward_event_q_cursor: ::core::clone::Clone::clone(&(*__self_0_5)),
                start_ts: ::core::clone::Clone::clone(&(*__self_0_6)),
                expiry_ts: ::core::clone::Clone::clone(&(*__self_0_7)),
                expiry_receiver: ::core::clone::Clone::clone(&(*__self_0_8)),
                from: ::core::clone::Clone::clone(&(*__self_0_9)),
                total: ::core::clone::Clone::clone(&(*__self_0_10)),
                expired: ::core::clone::Clone::clone(&(*__self_0_11)),
                kind: ::core::clone::Clone::clone(&(*__self_0_12)),
            },
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for RewardVendor {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::result::Result<(), ProgramError> {
        writer
            .write_all(&[75, 104, 40, 211, 191, 255, 206, 162])
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        AnchorSerialize::serialize(self, writer)
            .map_err(|_| anchor_lang::__private::ErrorCode::AccountDidNotSerialize)?;
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for RewardVendor {
    fn try_deserialize(buf: &mut &[u8]) -> std::result::Result<Self, ProgramError> {
        if buf.len() < [75, 104, 40, 211, 191, 255, 206, 162].len() {
            return Err(anchor_lang::__private::ErrorCode::AccountDiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &[75, 104, 40, 211, 191, 255, 206, 162] != given_disc {
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
impl anchor_lang::Discriminator for RewardVendor {
    fn discriminator() -> [u8; 8] {
        [75, 104, 40, 211, 191, 255, 206, 162]
    }
}
#[automatically_derived]
impl anchor_lang::Owner for RewardVendor {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub enum RewardVendorKind {
    Unlocked,
    Locked {
        start_ts: i64,
        end_ts: i64,
        period_count: u64,
    },
}
impl borsh::ser::BorshSerialize for RewardVendorKind
where
    i64: borsh::ser::BorshSerialize,
    i64: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> core::result::Result<(), borsh::maybestd::io::Error> {
        match self {
            RewardVendorKind::Unlocked => {
                let variant_idx: u8 = 0u8;
                writer.write_all(&variant_idx.to_le_bytes())?;
            }
            RewardVendorKind::Locked {
                start_ts,
                end_ts,
                period_count,
            } => {
                let variant_idx: u8 = 1u8;
                writer.write_all(&variant_idx.to_le_bytes())?;
                borsh::BorshSerialize::serialize(start_ts, writer)?;
                borsh::BorshSerialize::serialize(end_ts, writer)?;
                borsh::BorshSerialize::serialize(period_count, writer)?;
            }
        }
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for RewardVendorKind
where
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
        let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
        let return_value = match variant_idx {
            0u8 => RewardVendorKind::Unlocked,
            1u8 => RewardVendorKind::Locked {
                start_ts: borsh::BorshDeserialize::deserialize(buf)?,
                end_ts: borsh::BorshDeserialize::deserialize(buf)?,
                period_count: borsh::BorshDeserialize::deserialize(buf)?,
            },
            _ => {
                let msg = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Unexpected variant index: "],
                        &match (&variant_idx,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ));
                    res
                };
                return Err(borsh::maybestd::io::Error::new(
                    borsh::maybestd::io::ErrorKind::InvalidInput,
                    msg,
                ));
            }
        };
        Ok(return_value)
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for RewardVendorKind {
    #[inline]
    fn clone(&self) -> RewardVendorKind {
        match (&*self,) {
            (&RewardVendorKind::Unlocked,) => RewardVendorKind::Unlocked,
            (&RewardVendorKind::Locked {
                start_ts: ref __self_0,
                end_ts: ref __self_1,
                period_count: ref __self_2,
            },) => RewardVendorKind::Locked {
                start_ts: ::core::clone::Clone::clone(&(*__self_0)),
                end_ts: ::core::clone::Clone::clone(&(*__self_1)),
                period_count: ::core::clone::Clone::clone(&(*__self_2)),
            },
        }
    }
}
impl ::core::marker::StructuralPartialEq for RewardVendorKind {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for RewardVendorKind {
    #[inline]
    fn eq(&self, other: &RewardVendorKind) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RewardVendorKind::Locked {
                            start_ts: ref __self_0,
                            end_ts: ref __self_1,
                            period_count: ref __self_2,
                        },
                        &RewardVendorKind::Locked {
                            start_ts: ref __arg_1_0,
                            end_ts: ref __arg_1_1,
                            period_count: ref __arg_1_2,
                        },
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RewardVendorKind) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RewardVendorKind::Locked {
                            start_ts: ref __self_0,
                            end_ts: ref __self_1,
                            period_count: ref __self_2,
                        },
                        &RewardVendorKind::Locked {
                            start_ts: ref __arg_1_0,
                            end_ts: ref __arg_1_1,
                            period_count: ref __arg_1_2,
                        },
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    _ => false,
                }
            } else {
                true
            }
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
    RewardQAlreadyInitialized,
    InvalidNonce,
    InvalidPoolMintAuthority,
    InvalidMemberSigner,
    InvalidVaultDeposit,
    InvalidDepositor,
    InvalidVault,
    InvalidVaultOwner,
    Unknown,
    UnstakeTimelock,
    InsufficientReward,
    InvalidExpiry,
    VendorExpired,
    CursorAlreadyProcessed,
    NotStakedDuringDrop,
    VendorNotYetExpired,
    RewardsNeedsProcessing,
    ExpectedLockedVendor,
    ExpectedUnlockedVendor,
    InvalidVestingSigner,
    UnrealizedReward,
    InvalidBeneficiary,
    InvalidRealizorMetadata,
    InvalidVestingSchedule,
    InvalidProgramAuthority,
    InvalidMint,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&ErrorCode::RewardQAlreadyInitialized,) => {
                ::core::fmt::Formatter::write_str(f, "RewardQAlreadyInitialized")
            }
            (&ErrorCode::InvalidNonce,) => ::core::fmt::Formatter::write_str(f, "InvalidNonce"),
            (&ErrorCode::InvalidPoolMintAuthority,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidPoolMintAuthority")
            }
            (&ErrorCode::InvalidMemberSigner,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidMemberSigner")
            }
            (&ErrorCode::InvalidVaultDeposit,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVaultDeposit")
            }
            (&ErrorCode::InvalidDepositor,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidDepositor")
            }
            (&ErrorCode::InvalidVault,) => ::core::fmt::Formatter::write_str(f, "InvalidVault"),
            (&ErrorCode::InvalidVaultOwner,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVaultOwner")
            }
            (&ErrorCode::Unknown,) => ::core::fmt::Formatter::write_str(f, "Unknown"),
            (&ErrorCode::UnstakeTimelock,) => {
                ::core::fmt::Formatter::write_str(f, "UnstakeTimelock")
            }
            (&ErrorCode::InsufficientReward,) => {
                ::core::fmt::Formatter::write_str(f, "InsufficientReward")
            }
            (&ErrorCode::InvalidExpiry,) => ::core::fmt::Formatter::write_str(f, "InvalidExpiry"),
            (&ErrorCode::VendorExpired,) => ::core::fmt::Formatter::write_str(f, "VendorExpired"),
            (&ErrorCode::CursorAlreadyProcessed,) => {
                ::core::fmt::Formatter::write_str(f, "CursorAlreadyProcessed")
            }
            (&ErrorCode::NotStakedDuringDrop,) => {
                ::core::fmt::Formatter::write_str(f, "NotStakedDuringDrop")
            }
            (&ErrorCode::VendorNotYetExpired,) => {
                ::core::fmt::Formatter::write_str(f, "VendorNotYetExpired")
            }
            (&ErrorCode::RewardsNeedsProcessing,) => {
                ::core::fmt::Formatter::write_str(f, "RewardsNeedsProcessing")
            }
            (&ErrorCode::ExpectedLockedVendor,) => {
                ::core::fmt::Formatter::write_str(f, "ExpectedLockedVendor")
            }
            (&ErrorCode::ExpectedUnlockedVendor,) => {
                ::core::fmt::Formatter::write_str(f, "ExpectedUnlockedVendor")
            }
            (&ErrorCode::InvalidVestingSigner,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVestingSigner")
            }
            (&ErrorCode::UnrealizedReward,) => {
                ::core::fmt::Formatter::write_str(f, "UnrealizedReward")
            }
            (&ErrorCode::InvalidBeneficiary,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidBeneficiary")
            }
            (&ErrorCode::InvalidRealizorMetadata,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidRealizorMetadata")
            }
            (&ErrorCode::InvalidVestingSchedule,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidVestingSchedule")
            }
            (&ErrorCode::InvalidProgramAuthority,) => {
                ::core::fmt::Formatter::write_str(f, "InvalidProgramAuthority")
            }
            (&ErrorCode::InvalidMint,) => ::core::fmt::Formatter::write_str(f, "InvalidMint"),
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
            ErrorCode::RewardQAlreadyInitialized => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The given reward queue has already been initialized."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidNonce => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The nonce given doesn\'t derive a valid program address."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidPoolMintAuthority => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid pool mint authority"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidMemberSigner => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Member signer doesn\'t match the derived address."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVaultDeposit => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The given vault owner must match the signing depositor."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidDepositor => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The signing depositor doesn\'t match either of the balance accounts"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVault => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The vault given does not match the vault expected."],
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
            ErrorCode::Unknown => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["An unknown error has occured."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::UnstakeTimelock => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The unstake timelock has not yet expired."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InsufficientReward => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Reward vendors must have at least one token unit per pool token"],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidExpiry => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Reward expiry must be after the current clock timestamp."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::VendorExpired => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The reward vendor has been expired."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::CursorAlreadyProcessed => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["This reward has already been processed."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::NotStakedDuringDrop => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The account was not staked at the time of this reward."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::VendorNotYetExpired => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The vendor is not yet eligible for expiry."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::RewardsNeedsProcessing => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Please collect your reward before otherwise using the program."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::ExpectedLockedVendor => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Locked reward vendor expected but an unlocked vendor was given."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::ExpectedUnlockedVendor => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Unlocked reward vendor expected but a locked vendor was given."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVestingSigner => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Locked deposit from an invalid deposit authority."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::UnrealizedReward => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Locked rewards cannot be realized until one unstaked all tokens."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidBeneficiary => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The beneficiary doesn\'t match."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidRealizorMetadata => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["The given member account does not match the realizor metadata."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidVestingSchedule => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid vesting schedule for the locked reward."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidProgramAuthority => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Please specify the correct authority for this program."],
                &match () {
                    () => [],
                },
            )),
            ErrorCode::InvalidMint => fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &["Invalid mint supplied"],
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
impl<'a, 'b, 'c, 'info> From<&mut Deposit<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut Deposit<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'a, 'b, 'c, 'info> From<&mut DepositLocked<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut DepositLocked<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.vesting_vault.clone(),
            to: accounts.member_vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'a, 'b, 'c, 'info> From<&mut DropReward<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut DropReward<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vendor_vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'info> From<&BalanceSandboxAccounts<'info>> for BalanceSandbox {
    fn from(accs: &BalanceSandboxAccounts<'info>) -> Self {
        Self {
            spt: *accs.spt.to_account_info().key,
            vault: *accs.vault.to_account_info().key,
            vault_stake: *accs.vault_stake.to_account_info().key,
            vault_pw: *accs.vault_pw.to_account_info().key,
        }
    }
}
fn reward_eligible(cmn: &ClaimRewardCommon) -> ProgramResult {
    let vendor = &cmn.vendor;
    let member = &cmn.member;
    if vendor.expired {
        return Err(ErrorCode::VendorExpired.into());
    }
    if member.rewards_cursor > vendor.reward_event_q_cursor {
        return Err(ErrorCode::CursorAlreadyProcessed.into());
    }
    if member.last_stake_ts > vendor.start_ts {
        return Err(ErrorCode::NotStakedDuringDrop.into());
    }
    Ok(())
}
pub fn no_available_rewards<'info>(
    reward_q: &Box<Account<'info, RewardQueue>>,
    member: &Box<Account<'info, Member>>,
    balances: &BalanceSandboxAccounts<'info>,
) -> ProgramResult {
    let mut cursor = member.rewards_cursor;
    let tail = reward_q.tail();
    if cursor < tail {
        cursor = tail;
    }
    while cursor < reward_q.head() {
        let r_event = reward_q.get(cursor);
        if member.last_stake_ts < r_event.ts {
            if balances.spt.amount > 0 {
                return Err(ErrorCode::RewardsNeedsProcessing.into());
            }
        }
        cursor += 1;
    }
    Ok(())
}
pub const SRM_MIN_REWARD: u64 = 500_000_000;
pub const FIDA_MIN_REWARD: u64 = 900_000_000;
pub const DXL_MIN_REWARD: u64 = 900_000_000;
pub mod srm_registrar {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            73u8, 22u8, 61u8, 92u8, 137u8, 161u8, 182u8, 34u8, 181u8, 142u8, 127u8, 106u8, 104u8,
            123u8, 130u8, 205u8, 25u8, 128u8, 185u8, 130u8, 247u8, 84u8, 44u8, 22u8, 50u8, 20u8,
            125u8, 74u8, 171u8, 135u8, 29u8, 55u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod msrm_registrar {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            102u8, 151u8, 85u8, 159u8, 36u8, 8u8, 102u8, 195u8, 86u8, 176u8, 239u8, 9u8, 199u8,
            156u8, 131u8, 169u8, 207u8, 241u8, 9u8, 59u8, 168u8, 8u8, 9u8, 199u8, 91u8, 47u8,
            214u8, 37u8, 107u8, 1u8, 251u8, 251u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod fida_registrar {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            62u8, 65u8, 237u8, 252u8, 172u8, 57u8, 48u8, 247u8, 12u8, 187u8, 72u8, 82u8, 210u8,
            156u8, 187u8, 67u8, 56u8, 90u8, 23u8, 98u8, 124u8, 130u8, 6u8, 172u8, 173u8, 29u8,
            106u8, 24u8, 14u8, 155u8, 110u8, 192u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod dxl_registrar {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            154u8, 179u8, 207u8, 158u8, 63u8, 102u8, 7u8, 102u8, 77u8, 15u8, 106u8, 201u8, 69u8,
            145u8, 84u8, 111u8, 16u8, 99u8, 248u8, 245u8, 173u8, 231u8, 249u8, 221u8, 81u8, 101u8,
            181u8, 72u8, 0u8, 116u8, 178u8, 0u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod srm_mint {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            6u8, 131u8, 16u8, 134u8, 26u8, 152u8, 50u8, 125u8, 5u8, 80u8, 87u8, 77u8, 132u8, 65u8,
            138u8, 166u8, 225u8, 12u8, 51u8, 82u8, 221u8, 170u8, 127u8, 215u8, 245u8, 129u8, 82u8,
            204u8, 238u8, 178u8, 56u8, 135u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod fida_mint {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            202u8, 77u8, 57u8, 150u8, 76u8, 156u8, 181u8, 249u8, 121u8, 13u8, 10u8, 18u8, 150u8,
            159u8, 96u8, 253u8, 151u8, 36u8, 147u8, 98u8, 132u8, 234u8, 74u8, 18u8, 218u8, 222u8,
            212u8, 45u8, 223u8, 166u8, 156u8, 93u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
pub mod dxl_mint {
    /// The static program ID
    pub static ID: ::solana_program::pubkey::Pubkey =
        ::solana_program::pubkey::Pubkey::new_from_array([
            235u8, 199u8, 56u8, 11u8, 75u8, 36u8, 103u8, 165u8, 111u8, 36u8, 125u8, 157u8, 131u8,
            99u8, 95u8, 69u8, 223u8, 119u8, 75u8, 211u8, 65u8, 254u8, 2u8, 142u8, 245u8, 107u8,
            69u8, 96u8, 123u8, 254u8, 37u8, 108u8,
        ]);
    /// Confirms that a given pubkey is equivalent to the program ID
    pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
        id == &ID
    }
    /// Returns the program ID
    pub fn id() -> ::solana_program::pubkey::Pubkey {
        ID
    }
}
