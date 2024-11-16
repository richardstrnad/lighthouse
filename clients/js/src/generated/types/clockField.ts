/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum ClockField {
  Slot,
  EpochStartTimestamp,
  Epoch,
  LeaderScheduleEpoch,
  UnixTimestamp,
}

export type ClockFieldArgs = ClockField;

export function getClockFieldSerializer(): Serializer<
  ClockFieldArgs,
  ClockField
> {
  return scalarEnum<ClockField>(ClockField, {
    description: 'ClockField',
  }) as Serializer<ClockFieldArgs, ClockField>;
}
