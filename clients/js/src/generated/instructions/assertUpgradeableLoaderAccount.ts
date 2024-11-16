/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  Context,
  Pda,
  PublicKey,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';
import {
  LogLevel,
  LogLevelArgs,
  UpgradeableLoaderStateAssertion,
  UpgradeableLoaderStateAssertionArgs,
  getLogLevelSerializer,
  getUpgradeableLoaderStateAssertionSerializer,
} from '../types';

// Accounts.
export type AssertUpgradeableLoaderAccountInstructionAccounts = {
  /** Target account to be asserted */
  targetAccount: PublicKey | Pda;
};

// Data.
export type AssertUpgradeableLoaderAccountInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  assertion: UpgradeableLoaderStateAssertion;
};

export type AssertUpgradeableLoaderAccountInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertion: UpgradeableLoaderStateAssertionArgs;
};

export function getAssertUpgradeableLoaderAccountInstructionDataSerializer(): Serializer<
  AssertUpgradeableLoaderAccountInstructionDataArgs,
  AssertUpgradeableLoaderAccountInstructionData
> {
  return mapSerializer<
    AssertUpgradeableLoaderAccountInstructionDataArgs,
    any,
    AssertUpgradeableLoaderAccountInstructionData
  >(
    struct<AssertUpgradeableLoaderAccountInstructionData>(
      [
        ['discriminator', u8()],
        ['logLevel', getLogLevelSerializer()],
        ['assertion', getUpgradeableLoaderStateAssertionSerializer()],
      ],
      { description: 'AssertUpgradeableLoaderAccountInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: 13,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  ) as Serializer<
    AssertUpgradeableLoaderAccountInstructionDataArgs,
    AssertUpgradeableLoaderAccountInstructionData
  >;
}

// Args.
export type AssertUpgradeableLoaderAccountInstructionArgs =
  AssertUpgradeableLoaderAccountInstructionDataArgs;

// Instruction.
export function assertUpgradeableLoaderAccount(
  context: Pick<Context, 'programs'>,
  input: AssertUpgradeableLoaderAccountInstructionAccounts &
    AssertUpgradeableLoaderAccountInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'lighthouse',
    'L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95'
  );

  // Accounts.
  const resolvedAccounts = {
    targetAccount: {
      index: 0,
      isWritable: false as boolean,
      value: input.targetAccount ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: AssertUpgradeableLoaderAccountInstructionArgs = {
    ...input,
  };

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data =
    getAssertUpgradeableLoaderAccountInstructionDataSerializer().serialize(
      resolvedArgs as AssertUpgradeableLoaderAccountInstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
