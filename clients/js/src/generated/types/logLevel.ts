/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum LogLevel {
  Silent,
  PlaintextMessage,
  EncodedMessage,
  EncodedNoop,
  FailedPlaintextMessage,
  FailedEncodedMessage,
  FailedEncodedNoop,
}

export type LogLevelArgs = LogLevel;

export function getLogLevelSerializer(): Serializer<LogLevelArgs, LogLevel> {
  return scalarEnum<LogLevel>(LogLevel, {
    description: 'LogLevel',
  }) as Serializer<LogLevelArgs, LogLevel>;
}
