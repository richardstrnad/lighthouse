//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::AccountDeltaAssertion;
use crate::generated::types::LogLevel;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct AssertAccountDelta {
    /// Account A where the delta is calculated from
    pub account_a: solana_program::pubkey::Pubkey,
    /// Account B where the delta is calculated to
    pub account_b: solana_program::pubkey::Pubkey,
}

impl AssertAccountDelta {
    pub fn instruction(
        &self,
        args: AssertAccountDeltaInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AssertAccountDeltaInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.account_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.account_b,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AssertAccountDeltaInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LIGHTHOUSE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AssertAccountDeltaInstructionData {
    discriminator: u8,
}

impl AssertAccountDeltaInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 3 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssertAccountDeltaInstructionArgs {
    pub log_level: LogLevel,
    pub assertion: AccountDeltaAssertion,
}

/// Instruction builder for `AssertAccountDelta`.
///
/// ### Accounts:
///
///   0. `[]` account_a
///   1. `[]` account_b
#[derive(Default)]
pub struct AssertAccountDeltaBuilder {
    account_a: Option<solana_program::pubkey::Pubkey>,
    account_b: Option<solana_program::pubkey::Pubkey>,
    log_level: Option<LogLevel>,
    assertion: Option<AccountDeltaAssertion>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AssertAccountDeltaBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Account A where the delta is calculated from
    #[inline(always)]
    pub fn account_a(&mut self, account_a: solana_program::pubkey::Pubkey) -> &mut Self {
        self.account_a = Some(account_a);
        self
    }
    /// Account B where the delta is calculated to
    #[inline(always)]
    pub fn account_b(&mut self, account_b: solana_program::pubkey::Pubkey) -> &mut Self {
        self.account_b = Some(account_b);
        self
    }
    /// `[optional argument, defaults to 'LogLevel::Silent']`
    #[inline(always)]
    pub fn log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.log_level = Some(log_level);
        self
    }
    #[inline(always)]
    pub fn assertion(&mut self, assertion: AccountDeltaAssertion) -> &mut Self {
        self.assertion = Some(assertion);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = AssertAccountDelta {
            account_a: self.account_a.expect("account_a is not set"),
            account_b: self.account_b.expect("account_b is not set"),
        };
        let args = AssertAccountDeltaInstructionArgs {
            log_level: self.log_level.clone().unwrap_or(LogLevel::Silent),
            assertion: self.assertion.clone().expect("assertion is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `assert_account_delta` CPI accounts.
pub struct AssertAccountDeltaCpiAccounts<'a, 'b> {
    /// Account A where the delta is calculated from
    pub account_a: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account B where the delta is calculated to
    pub account_b: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `assert_account_delta` CPI instruction.
pub struct AssertAccountDeltaCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account A where the delta is calculated from
    pub account_a: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account B where the delta is calculated to
    pub account_b: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AssertAccountDeltaInstructionArgs,
}

impl<'a, 'b> AssertAccountDeltaCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AssertAccountDeltaCpiAccounts<'a, 'b>,
        args: AssertAccountDeltaInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            account_a: accounts.account_a,
            account_b: accounts.account_b,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.account_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.account_b.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AssertAccountDeltaInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LIGHTHOUSE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.account_a.clone());
        account_infos.push(self.account_b.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `AssertAccountDelta` via CPI.
///
/// ### Accounts:
///
///   0. `[]` account_a
///   1. `[]` account_b
pub struct AssertAccountDeltaCpiBuilder<'a, 'b> {
    instruction: Box<AssertAccountDeltaCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AssertAccountDeltaCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AssertAccountDeltaCpiBuilderInstruction {
            __program: program,
            account_a: None,
            account_b: None,
            log_level: None,
            assertion: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Account A where the delta is calculated from
    #[inline(always)]
    pub fn account_a(
        &mut self,
        account_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.account_a = Some(account_a);
        self
    }
    /// Account B where the delta is calculated to
    #[inline(always)]
    pub fn account_b(
        &mut self,
        account_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.account_b = Some(account_b);
        self
    }
    /// `[optional argument, defaults to 'LogLevel::Silent']`
    #[inline(always)]
    pub fn log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.instruction.log_level = Some(log_level);
        self
    }
    #[inline(always)]
    pub fn assertion(&mut self, assertion: AccountDeltaAssertion) -> &mut Self {
        self.instruction.assertion = Some(assertion);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = AssertAccountDeltaInstructionArgs {
            log_level: self
                .instruction
                .log_level
                .clone()
                .unwrap_or(LogLevel::Silent),
            assertion: self
                .instruction
                .assertion
                .clone()
                .expect("assertion is not set"),
        };
        let instruction = AssertAccountDeltaCpi {
            __program: self.instruction.__program,

            account_a: self.instruction.account_a.expect("account_a is not set"),

            account_b: self.instruction.account_b.expect("account_b is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct AssertAccountDeltaCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    account_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    account_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    log_level: Option<LogLevel>,
    assertion: Option<AccountDeltaAssertion>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
