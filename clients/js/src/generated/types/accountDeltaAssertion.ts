/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Codec,
  Decoder,
  Encoder,
  GetDataEnumKind,
  GetDataEnumKindContent,
  combineCodec,
  getDataEnumDecoder,
  getDataEnumEncoder,
  getStructDecoder,
  getStructEncoder,
  getU16Decoder,
  getU16Encoder,
} from '@solana/codecs';
import {
  AccountInfoDeltaAssertion,
  AccountInfoDeltaAssertionArgs,
  DataValueDeltaAssertion,
  DataValueDeltaAssertionArgs,
  getAccountInfoDeltaAssertionDecoder,
  getAccountInfoDeltaAssertionEncoder,
  getDataValueDeltaAssertionDecoder,
  getDataValueDeltaAssertionEncoder,
} from '.';

export type AccountDeltaAssertion =
  | {
      __kind: 'AccountInfo';
      aOffset: number;
      assertion: AccountInfoDeltaAssertion;
    }
  | {
      __kind: 'Data';
      aOffset: number;
      bOffset: number;
      assertion: DataValueDeltaAssertion;
    };

export type AccountDeltaAssertionArgs =
  | {
      __kind: 'AccountInfo';
      aOffset: number;
      assertion: AccountInfoDeltaAssertionArgs;
    }
  | {
      __kind: 'Data';
      aOffset: number;
      bOffset: number;
      assertion: DataValueDeltaAssertionArgs;
    };

export function getAccountDeltaAssertionEncoder(): Encoder<AccountDeltaAssertionArgs> {
  return getDataEnumEncoder([
    [
      'AccountInfo',
      getStructEncoder([
        ['aOffset', getU16Encoder()],
        ['assertion', getAccountInfoDeltaAssertionEncoder()],
      ]),
    ],
    [
      'Data',
      getStructEncoder([
        ['aOffset', getU16Encoder()],
        ['bOffset', getU16Encoder()],
        ['assertion', getDataValueDeltaAssertionEncoder()],
      ]),
    ],
  ]);
}

export function getAccountDeltaAssertionDecoder(): Decoder<AccountDeltaAssertion> {
  return getDataEnumDecoder([
    [
      'AccountInfo',
      getStructDecoder([
        ['aOffset', getU16Decoder()],
        ['assertion', getAccountInfoDeltaAssertionDecoder()],
      ]),
    ],
    [
      'Data',
      getStructDecoder([
        ['aOffset', getU16Decoder()],
        ['bOffset', getU16Decoder()],
        ['assertion', getDataValueDeltaAssertionDecoder()],
      ]),
    ],
  ]);
}

export function getAccountDeltaAssertionCodec(): Codec<
  AccountDeltaAssertionArgs,
  AccountDeltaAssertion
> {
  return combineCodec(
    getAccountDeltaAssertionEncoder(),
    getAccountDeltaAssertionDecoder()
  );
}

// Data Enum Helpers.
export function accountDeltaAssertion(
  kind: 'AccountInfo',
  data: GetDataEnumKindContent<AccountDeltaAssertionArgs, 'AccountInfo'>
): GetDataEnumKind<AccountDeltaAssertionArgs, 'AccountInfo'>;
export function accountDeltaAssertion(
  kind: 'Data',
  data: GetDataEnumKindContent<AccountDeltaAssertionArgs, 'Data'>
): GetDataEnumKind<AccountDeltaAssertionArgs, 'Data'>;
export function accountDeltaAssertion<
  K extends AccountDeltaAssertionArgs['__kind']
>(kind: K, data?: any): Extract<AccountDeltaAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isAccountDeltaAssertion<
  K extends AccountDeltaAssertion['__kind']
>(
  kind: K,
  value: AccountDeltaAssertion
): value is AccountDeltaAssertion & { __kind: K } {
  return value.__kind === kind;
}
