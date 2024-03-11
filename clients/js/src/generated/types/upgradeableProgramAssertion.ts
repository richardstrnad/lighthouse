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
  combineCodec,
  getDataEnumDecoder,
  getDataEnumEncoder,
  getStructDecoder,
  getStructEncoder,
} from '@solana/codecs';
import {
  EquatableOperator,
  EquatableOperatorArgs,
  getEquatableOperatorDecoder,
  getEquatableOperatorEncoder,
} from '.';

export type UpgradeableProgramAssertion = {
  __kind: 'ProgramDataAddress';
  value: Address;
  operator: EquatableOperator;
};

export type UpgradeableProgramAssertionArgs = {
  __kind: 'ProgramDataAddress';
  value: Address;
  operator: EquatableOperatorArgs;
};

export function getUpgradeableProgramAssertionEncoder(): Encoder<UpgradeableProgramAssertionArgs> {
  return getDataEnumEncoder([
    [
      'ProgramDataAddress',
      getStructEncoder([
        ['value', getAddressEncoder()],
        ['operator', getEquatableOperatorEncoder()],
      ]),
    ],
  ]);
}

export function getUpgradeableProgramAssertionDecoder(): Decoder<UpgradeableProgramAssertion> {
  return getDataEnumDecoder([
    [
      'ProgramDataAddress',
      getStructDecoder([
        ['value', getAddressDecoder()],
        ['operator', getEquatableOperatorDecoder()],
      ]),
    ],
  ]);
}

export function getUpgradeableProgramAssertionCodec(): Codec<
  UpgradeableProgramAssertionArgs,
  UpgradeableProgramAssertion
> {
  return combineCodec(
    getUpgradeableProgramAssertionEncoder(),
    getUpgradeableProgramAssertionDecoder()
  );
}

// Data Enum Helpers.
export function upgradeableProgramAssertion(
  kind: 'ProgramDataAddress',
  data: GetDataEnumKindContent<
    UpgradeableProgramAssertionArgs,
    'ProgramDataAddress'
  >
): GetDataEnumKind<UpgradeableProgramAssertionArgs, 'ProgramDataAddress'>;
export function upgradeableProgramAssertion<
  K extends UpgradeableProgramAssertionArgs['__kind']
>(
  kind: K,
  data?: any
): Extract<UpgradeableProgramAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isUpgradeableProgramAssertion<
  K extends UpgradeableProgramAssertion['__kind']
>(
  kind: K,
  value: UpgradeableProgramAssertion
): value is UpgradeableProgramAssertion & { __kind: K } {
  return value.__kind === kind;
}
