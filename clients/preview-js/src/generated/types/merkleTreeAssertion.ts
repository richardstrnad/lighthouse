/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getDiscriminatedUnionDecoder,
  getDiscriminatedUnionEncoder,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  type Codec,
  type Decoder,
  type Encoder,
  type GetDiscriminatedUnionVariant,
  type GetDiscriminatedUnionVariantContent,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export type MerkleTreeAssertion = {
  __kind: 'VerifyLeaf';
  leafIndex: number;
  leafHash: ReadonlyUint8Array;
};

export type MerkleTreeAssertionArgs = MerkleTreeAssertion;

export function getMerkleTreeAssertionEncoder(): Encoder<MerkleTreeAssertionArgs> {
  return getDiscriminatedUnionEncoder([
    [
      'VerifyLeaf',
      getStructEncoder([
        ['leafIndex', getU32Encoder()],
        ['leafHash', fixEncoderSize(getBytesEncoder(), 32)],
      ]),
    ],
  ]);
}

export function getMerkleTreeAssertionDecoder(): Decoder<MerkleTreeAssertion> {
  return getDiscriminatedUnionDecoder([
    [
      'VerifyLeaf',
      getStructDecoder([
        ['leafIndex', getU32Decoder()],
        ['leafHash', fixDecoderSize(getBytesDecoder(), 32)],
      ]),
    ],
  ]);
}

export function getMerkleTreeAssertionCodec(): Codec<
  MerkleTreeAssertionArgs,
  MerkleTreeAssertion
> {
  return combineCodec(
    getMerkleTreeAssertionEncoder(),
    getMerkleTreeAssertionDecoder()
  );
}

// Data Enum Helpers.
export function merkleTreeAssertion(
  kind: 'VerifyLeaf',
  data: GetDiscriminatedUnionVariantContent<
    MerkleTreeAssertionArgs,
    '__kind',
    'VerifyLeaf'
  >
): GetDiscriminatedUnionVariant<
  MerkleTreeAssertionArgs,
  '__kind',
  'VerifyLeaf'
>;
export function merkleTreeAssertion<
  K extends MerkleTreeAssertionArgs['__kind'],
  Data,
>(kind: K, data?: Data) {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isMerkleTreeAssertion<K extends MerkleTreeAssertion['__kind']>(
  kind: K,
  value: MerkleTreeAssertion
): value is MerkleTreeAssertion & { __kind: K } {
  return value.__kind === kind;
}
