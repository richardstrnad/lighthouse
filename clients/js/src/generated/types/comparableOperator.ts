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
  combineCodec,
  getScalarEnumDecoder,
  getScalarEnumEncoder,
} from '@solana/codecs';

export enum ComparableOperator {
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterThanOrEqual,
  LessThanOrEqual,
}

export type ComparableOperatorArgs = ComparableOperator;

export function getComparableOperatorEncoder(): Encoder<ComparableOperatorArgs> {
  return getScalarEnumEncoder(ComparableOperator);
}

export function getComparableOperatorDecoder(): Decoder<ComparableOperator> {
  return getScalarEnumDecoder(ComparableOperator);
}

export function getComparableOperatorCodec(): Codec<
  ComparableOperatorArgs,
  ComparableOperator
> {
  return combineCodec(
    getComparableOperatorEncoder(),
    getComparableOperatorDecoder()
  );
}
