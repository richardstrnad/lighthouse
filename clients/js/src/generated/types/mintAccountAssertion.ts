/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Address,
  getAddressDecoder,
  getAddressEncoder,
} from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  GetDataEnumKind,
  GetDataEnumKindContent,
  Option,
  OptionOrNullable,
  combineCodec,
  getBooleanDecoder,
  getBooleanEncoder,
  getDataEnumDecoder,
  getDataEnumEncoder,
  getOptionDecoder,
  getOptionEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
} from '@solana/codecs';
import {
  EquatableOperator,
  EquatableOperatorArgs,
  IntegerOperator,
  IntegerOperatorArgs,
  getEquatableOperatorDecoder,
  getEquatableOperatorEncoder,
  getIntegerOperatorDecoder,
  getIntegerOperatorEncoder,
} from '.';

export type MintAccountAssertion =
  | {
      __kind: 'MintAuthority';
      value: Option<Address>;
      operator: EquatableOperator;
    }
  | { __kind: 'Supply'; value: bigint; operator: IntegerOperator }
  | { __kind: 'Decimals'; value: number; operator: IntegerOperator }
  | { __kind: 'IsInitialized'; value: boolean; operator: EquatableOperator }
  | {
      __kind: 'FreezeAuthority';
      value: Option<Address>;
      operator: EquatableOperator;
    };

export type MintAccountAssertionArgs =
  | {
      __kind: 'MintAuthority';
      value: OptionOrNullable<Address>;
      operator: EquatableOperatorArgs;
    }
  | { __kind: 'Supply'; value: number | bigint; operator: IntegerOperatorArgs }
  | { __kind: 'Decimals'; value: number; operator: IntegerOperatorArgs }
  | { __kind: 'IsInitialized'; value: boolean; operator: EquatableOperatorArgs }
  | {
      __kind: 'FreezeAuthority';
      value: OptionOrNullable<Address>;
      operator: EquatableOperatorArgs;
    };

export function getMintAccountAssertionEncoder(): Encoder<MintAccountAssertionArgs> {
  return getDataEnumEncoder([
    [
      'MintAuthority',
      getStructEncoder([
        ['value', getOptionEncoder(getAddressEncoder())],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
    [
      'Supply',
      getStructEncoder([
        ['value', getU64Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'Decimals',
      getStructEncoder([
        ['value', getU8Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
    [
      'IsInitialized',
      getStructEncoder([
        ['value', getBooleanEncoder()],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
    [
      'FreezeAuthority',
      getStructEncoder([
        ['value', getOptionEncoder(getAddressEncoder())],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
  ]);
}

export function getMintAccountAssertionDecoder(): Decoder<MintAccountAssertion> {
  return getDataEnumDecoder([
    [
      'MintAuthority',
      getStructDecoder([
        ['value', getOptionDecoder(getAddressDecoder())],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
    [
      'Supply',
      getStructDecoder([
        ['value', getU64Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'Decimals',
      getStructDecoder([
        ['value', getU8Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
    [
      'IsInitialized',
      getStructDecoder([
        ['value', getBooleanDecoder()],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
    [
      'FreezeAuthority',
      getStructDecoder([
        ['value', getOptionDecoder(getAddressDecoder())],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
  ]);
}

export function getMintAccountAssertionCodec(): Codec<
  MintAccountAssertionArgs,
  MintAccountAssertion
> {
  return combineCodec(
    getMintAccountAssertionEncoder(),
    getMintAccountAssertionDecoder()
  );
}

// Data Enum Helpers.
export function mintAccountAssertion(
  kind: 'MintAuthority',
  data: GetDataEnumKindContent<MintAccountAssertionArgs, 'MintAuthority'>
): GetDataEnumKind<MintAccountAssertionArgs, 'MintAuthority'>;
export function mintAccountAssertion(
  kind: 'Supply',
  data: GetDataEnumKindContent<MintAccountAssertionArgs, 'Supply'>
): GetDataEnumKind<MintAccountAssertionArgs, 'Supply'>;
export function mintAccountAssertion(
  kind: 'Decimals',
  data: GetDataEnumKindContent<MintAccountAssertionArgs, 'Decimals'>
): GetDataEnumKind<MintAccountAssertionArgs, 'Decimals'>;
export function mintAccountAssertion(
  kind: 'IsInitialized',
  data: GetDataEnumKindContent<MintAccountAssertionArgs, 'IsInitialized'>
): GetDataEnumKind<MintAccountAssertionArgs, 'IsInitialized'>;
export function mintAccountAssertion(
  kind: 'FreezeAuthority',
  data: GetDataEnumKindContent<MintAccountAssertionArgs, 'FreezeAuthority'>
): GetDataEnumKind<MintAccountAssertionArgs, 'FreezeAuthority'>;
export function mintAccountAssertion<
  K extends MintAccountAssertionArgs['__kind']
>(kind: K, data?: any): Extract<MintAccountAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isMintAccountAssertion<
  K extends MintAccountAssertion['__kind']
>(
  kind: K,
  value: MintAccountAssertion
): value is MintAccountAssertion & { __kind: K } {
  return value.__kind === kind;
}
