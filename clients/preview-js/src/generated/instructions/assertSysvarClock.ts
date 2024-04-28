/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  mapEncoder,
} from '@solana/codecs';
import {
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
} from '@solana/instructions';
import { LIGHTHOUSE_PROGRAM_ADDRESS } from '../programs';
import {
  LogLevel,
  LogLevelArgs,
  SysvarClockAssertion,
  SysvarClockAssertionArgs,
  getLogLevelDecoder,
  getLogLevelEncoder,
  getSysvarClockAssertionDecoder,
  getSysvarClockAssertionEncoder,
} from '../types';

export type AssertSysvarClockInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<TRemainingAccounts>;

export type AssertSysvarClockInstructionData = {
  discriminator: number;
  logLevel: LogLevel;
  assertion: SysvarClockAssertion;
};

export type AssertSysvarClockInstructionDataArgs = {
  logLevel?: LogLevelArgs;
  assertion: SysvarClockAssertionArgs;
};

export function getAssertSysvarClockInstructionDataEncoder(): Encoder<AssertSysvarClockInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['logLevel', getLogLevelEncoder()],
      ['assertion', getSysvarClockAssertionEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: 14,
      logLevel: value.logLevel ?? LogLevel.Silent,
    })
  );
}

export function getAssertSysvarClockInstructionDataDecoder(): Decoder<AssertSysvarClockInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['logLevel', getLogLevelDecoder()],
    ['assertion', getSysvarClockAssertionDecoder()],
  ]);
}

export function getAssertSysvarClockInstructionDataCodec(): Codec<
  AssertSysvarClockInstructionDataArgs,
  AssertSysvarClockInstructionData
> {
  return combineCodec(
    getAssertSysvarClockInstructionDataEncoder(),
    getAssertSysvarClockInstructionDataDecoder()
  );
}

export type AssertSysvarClockInput = {
  logLevel?: AssertSysvarClockInstructionDataArgs['logLevel'];
  assertion: AssertSysvarClockInstructionDataArgs['assertion'];
};

export function getAssertSysvarClockInstruction(
  input: AssertSysvarClockInput
): AssertSysvarClockInstruction<typeof LIGHTHOUSE_PROGRAM_ADDRESS> {
  // Program address.
  const programAddress = LIGHTHOUSE_PROGRAM_ADDRESS;

  // Original args.
  const args = { ...input };

  const instruction = {
    programAddress,
    data: getAssertSysvarClockInstructionDataEncoder().encode(
      args as AssertSysvarClockInstructionDataArgs
    ),
  } as AssertSysvarClockInstruction<typeof LIGHTHOUSE_PROGRAM_ADDRESS>;

  return instruction;
}

export type ParsedAssertSysvarClockInstruction<
  TProgram extends string = typeof LIGHTHOUSE_PROGRAM_ADDRESS,
> = {
  programAddress: Address<TProgram>;
  data: AssertSysvarClockInstructionData;
};

export function parseAssertSysvarClockInstruction<TProgram extends string>(
  instruction: IInstruction<TProgram> & IInstructionWithData<Uint8Array>
): ParsedAssertSysvarClockInstruction<TProgram> {
  return {
    programAddress: instruction.programAddress,
    data: getAssertSysvarClockInstructionDataDecoder().decode(instruction.data),
  };
}
