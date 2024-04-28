/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
  dataEnum,
  struct,
  tuple,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  EquatableOperator,
  EquatableOperatorArgs,
  IntegerOperator,
  IntegerOperatorArgs,
  MetaAssertion,
  MetaAssertionArgs,
  StakeAssertion,
  StakeAssertionArgs,
  StakeStateType,
  StakeStateTypeArgs,
  getEquatableOperatorSerializer,
  getIntegerOperatorSerializer,
  getMetaAssertionSerializer,
  getStakeAssertionSerializer,
  getStakeStateTypeSerializer,
} from '.';

export type StakeAccountAssertion =
  | { __kind: 'State'; value: StakeStateType; operator: EquatableOperator }
  | { __kind: 'MetaAssertion'; fields: [MetaAssertion] }
  | { __kind: 'StakeAssertion'; fields: [StakeAssertion] }
  | { __kind: 'StakeFlags'; value: number; operator: IntegerOperator };

export type StakeAccountAssertionArgs =
  | {
      __kind: 'State';
      value: StakeStateTypeArgs;
      operator: EquatableOperatorArgs;
    }
  | { __kind: 'MetaAssertion'; fields: [MetaAssertionArgs] }
  | { __kind: 'StakeAssertion'; fields: [StakeAssertionArgs] }
  | { __kind: 'StakeFlags'; value: number; operator: IntegerOperatorArgs };

export function getStakeAccountAssertionSerializer(): Serializer<
  StakeAccountAssertionArgs,
  StakeAccountAssertion
> {
  return dataEnum<StakeAccountAssertion>(
    [
      [
        'State',
        struct<GetDataEnumKindContent<StakeAccountAssertion, 'State'>>([
          ['value', getStakeStateTypeSerializer()],
          ['operator', getEquatableOperatorSerializer()],
        ]),
      ],
      [
        'MetaAssertion',
        struct<GetDataEnumKindContent<StakeAccountAssertion, 'MetaAssertion'>>([
          ['fields', tuple([getMetaAssertionSerializer()])],
        ]),
      ],
      [
        'StakeAssertion',
        struct<GetDataEnumKindContent<StakeAccountAssertion, 'StakeAssertion'>>(
          [['fields', tuple([getStakeAssertionSerializer()])]]
        ),
      ],
      [
        'StakeFlags',
        struct<GetDataEnumKindContent<StakeAccountAssertion, 'StakeFlags'>>([
          ['value', u8()],
          ['operator', getIntegerOperatorSerializer()],
        ]),
      ],
    ],
    { description: 'StakeAccountAssertion' }
  ) as Serializer<StakeAccountAssertionArgs, StakeAccountAssertion>;
}

// Data Enum Helpers.
export function stakeAccountAssertion(
  kind: 'State',
  data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'State'>
): GetDataEnumKind<StakeAccountAssertionArgs, 'State'>;
export function stakeAccountAssertion(
  kind: 'MetaAssertion',
  data: GetDataEnumKindContent<
    StakeAccountAssertionArgs,
    'MetaAssertion'
  >['fields']
): GetDataEnumKind<StakeAccountAssertionArgs, 'MetaAssertion'>;
export function stakeAccountAssertion(
  kind: 'StakeAssertion',
  data: GetDataEnumKindContent<
    StakeAccountAssertionArgs,
    'StakeAssertion'
  >['fields']
): GetDataEnumKind<StakeAccountAssertionArgs, 'StakeAssertion'>;
export function stakeAccountAssertion(
  kind: 'StakeFlags',
  data: GetDataEnumKindContent<StakeAccountAssertionArgs, 'StakeFlags'>
): GetDataEnumKind<StakeAccountAssertionArgs, 'StakeFlags'>;
export function stakeAccountAssertion<
  K extends StakeAccountAssertionArgs['__kind'],
>(kind: K, data?: any): Extract<StakeAccountAssertionArgs, { __kind: K }> {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}
export function isStakeAccountAssertion<
  K extends StakeAccountAssertion['__kind'],
>(
  kind: K,
  value: StakeAccountAssertion
): value is StakeAccountAssertion & { __kind: K } {
  return value.__kind === kind;
}
