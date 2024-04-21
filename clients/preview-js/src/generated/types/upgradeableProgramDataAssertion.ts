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
  getDataEnumDecoder,
  getDataEnumEncoder,
  getOptionDecoder,
  getOptionEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
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

export type UpgradeableProgramDataAssertion =
  | {
      __kind: 'UpgradeAuthority';
      value: Option<Address>;
      operator: EquatableOperator;
    }
  | { __kind: 'Slot'; value: bigint; operator: IntegerOperator };

export type UpgradeableProgramDataAssertionArgs =
  | {
      __kind: 'UpgradeAuthority';
      value: OptionOrNullable<Address>;
      operator: EquatableOperatorArgs;
    }
  | { __kind: 'Slot'; value: number | bigint; operator: IntegerOperatorArgs };

export function getUpgradeableProgramDataAssertionEncoder(): Encoder<UpgradeableProgramDataAssertionArgs> {
  return getDataEnumEncoder([
    [
      'UpgradeAuthority',
      getStructEncoder([
        ['value', getOptionEncoder(getAddressEncoder())],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
    [
      'Slot',
      getStructEncoder([
        ['value', getU64Encoder()],
        ['operator', getIntegerOperatorEncoder()],
      ]),
    ],
  ]);
}

export function getUpgradeableProgramDataAssertionDecoder(): Decoder<UpgradeableProgramDataAssertion> {
  return getDataEnumDecoder([
    [
      'UpgradeAuthority',
      getStructDecoder([
        ['value', getOptionDecoder(getAddressDecoder())],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
    [
      'Slot',
      getStructDecoder([
        ['value', getU64Decoder()],
        ['operator', getIntegerOperatorDecoder()],
      ]),
    ],
  ]);
}

export function getUpgradeableProgramDataAssertionCodec(): Codec<
  UpgradeableProgramDataAssertionArgs,
  UpgradeableProgramDataAssertion
> {
  return combineCodec(
    getUpgradeableProgramDataAssertionEncoder(),
    getUpgradeableProgramDataAssertionDecoder()
  );
}

// Data Enum Helpers.
export function upgradeableProgramDataAssertion(
  kind: 'UpgradeAuthority',
  data: GetDataEnumKindContent<
    UpgradeableProgramDataAssertionArgs,
    'UpgradeAuthority'
  >
): GetDataEnumKind<UpgradeableProgramDataAssertionArgs, 'UpgradeAuthority'>;
export function upgradeableProgramDataAssertion(
  kind: 'Slot',
  data: GetDataEnumKindContent<UpgradeableProgramDataAssertionArgs, 'Slot'>
): GetDataEnumKind<UpgradeableProgramDataAssertionArgs, 'Slot'>;
export function upgradeableProgramDataAssertion<
  K extends UpgradeableProgramDataAssertionArgs['__kind']
>(
  kind: K,
  data?: any
): Extract<UpgradeableProgramDataAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isUpgradeableProgramDataAssertion<
  K extends UpgradeableProgramDataAssertion['__kind']
>(
  kind: K,
  value: UpgradeableProgramDataAssertion
): value is UpgradeableProgramDataAssertion & { __kind: K } {
  return value.__kind === kind;
}
