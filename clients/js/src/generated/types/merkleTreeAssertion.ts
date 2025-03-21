/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
  bytes,
  dataEnum,
  struct,
  u32,
} from '@metaplex-foundation/umi/serializers';

export type MerkleTreeAssertion = {
  __kind: 'VerifyLeaf';
  leafIndex: number;
  leafHash: Uint8Array;
};

export type MerkleTreeAssertionArgs = MerkleTreeAssertion;

export function getMerkleTreeAssertionSerializer(): Serializer<
  MerkleTreeAssertionArgs,
  MerkleTreeAssertion
> {
  return dataEnum<MerkleTreeAssertion>(
    [
      [
        'VerifyLeaf',
        struct<GetDataEnumKindContent<MerkleTreeAssertion, 'VerifyLeaf'>>([
          ['leafIndex', u32()],
          ['leafHash', bytes({ size: 32 })],
        ]),
      ],
    ],
    { description: 'MerkleTreeAssertion' }
  ) as Serializer<MerkleTreeAssertionArgs, MerkleTreeAssertion>;
}

// Data Enum Helpers.
export function merkleTreeAssertion(
  kind: 'VerifyLeaf',
  data: GetDataEnumKindContent<MerkleTreeAssertionArgs, 'VerifyLeaf'>
): GetDataEnumKind<MerkleTreeAssertionArgs, 'VerifyLeaf'>;
export function merkleTreeAssertion<
  K extends MerkleTreeAssertionArgs['__kind'],
>(kind: K, data?: any): Extract<MerkleTreeAssertionArgs, { __kind: K }> {
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
