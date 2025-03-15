/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/kit';
import { QED_DATA_LOADER_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const UPGRADE_DISCRIMINATOR = 3;

export function getUpgradeDiscriminatorBytes() {
  return getU8Encoder().encode(UPGRADE_DISCRIMINATOR);
}

export type UpgradeInstruction<
  TProgram extends string = typeof QED_DATA_LOADER_PROGRAM_ADDRESS,
  TAccountProgramDataAccount extends string | IAccountMeta<string> = string,
  TAccountProgramAccount extends string | IAccountMeta<string> = string,
  TAccountBufferAccount extends string | IAccountMeta<string> = string,
  TAccountSpillAccount extends string | IAccountMeta<string> = string,
  TAccountRentSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarRent111111111111111111111111111111111',
  TAccountClockSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarC1ock11111111111111111111111111111111',
  TAccountAuthority extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountProgramDataAccount extends string
        ? WritableAccount<TAccountProgramDataAccount>
        : TAccountProgramDataAccount,
      TAccountProgramAccount extends string
        ? WritableAccount<TAccountProgramAccount>
        : TAccountProgramAccount,
      TAccountBufferAccount extends string
        ? WritableAccount<TAccountBufferAccount>
        : TAccountBufferAccount,
      TAccountSpillAccount extends string
        ? WritableAccount<TAccountSpillAccount>
        : TAccountSpillAccount,
      TAccountRentSysvar extends string
        ? ReadonlyAccount<TAccountRentSysvar>
        : TAccountRentSysvar,
      TAccountClockSysvar extends string
        ? ReadonlyAccount<TAccountClockSysvar>
        : TAccountClockSysvar,
      TAccountAuthority extends string
        ? ReadonlySignerAccount<TAccountAuthority> &
            IAccountSignerMeta<TAccountAuthority>
        : TAccountAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type UpgradeInstructionData = { discriminator: number };

export type UpgradeInstructionDataArgs = {};

export function getUpgradeInstructionDataEncoder(): Encoder<UpgradeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: UPGRADE_DISCRIMINATOR })
  );
}

export function getUpgradeInstructionDataDecoder(): Decoder<UpgradeInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getUpgradeInstructionDataCodec(): Codec<
  UpgradeInstructionDataArgs,
  UpgradeInstructionData
> {
  return combineCodec(
    getUpgradeInstructionDataEncoder(),
    getUpgradeInstructionDataDecoder()
  );
}

export type UpgradeInput<
  TAccountProgramDataAccount extends string = string,
  TAccountProgramAccount extends string = string,
  TAccountBufferAccount extends string = string,
  TAccountSpillAccount extends string = string,
  TAccountRentSysvar extends string = string,
  TAccountClockSysvar extends string = string,
  TAccountAuthority extends string = string,
> = {
  /** ProgramData account. */
  programDataAccount: Address<TAccountProgramDataAccount>;
  /** Program account. */
  programAccount: Address<TAccountProgramAccount>;
  /** Buffer account where the new program data has been written. */
  bufferAccount: Address<TAccountBufferAccount>;
  /** Spill account. */
  spillAccount: Address<TAccountSpillAccount>;
  /** Rent sysvar. */
  rentSysvar?: Address<TAccountRentSysvar>;
  /** Clock sysvar. */
  clockSysvar?: Address<TAccountClockSysvar>;
  /** Authority. */
  authority: TransactionSigner<TAccountAuthority>;
};

export function getUpgradeInstruction<
  TAccountProgramDataAccount extends string,
  TAccountProgramAccount extends string,
  TAccountBufferAccount extends string,
  TAccountSpillAccount extends string,
  TAccountRentSysvar extends string,
  TAccountClockSysvar extends string,
  TAccountAuthority extends string,
  TProgramAddress extends Address = typeof QED_DATA_LOADER_PROGRAM_ADDRESS,
>(
  input: UpgradeInput<
    TAccountProgramDataAccount,
    TAccountProgramAccount,
    TAccountBufferAccount,
    TAccountSpillAccount,
    TAccountRentSysvar,
    TAccountClockSysvar,
    TAccountAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): UpgradeInstruction<
  TProgramAddress,
  TAccountProgramDataAccount,
  TAccountProgramAccount,
  TAccountBufferAccount,
  TAccountSpillAccount,
  TAccountRentSysvar,
  TAccountClockSysvar,
  TAccountAuthority
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? QED_DATA_LOADER_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    programDataAccount: {
      value: input.programDataAccount ?? null,
      isWritable: true,
    },
    programAccount: { value: input.programAccount ?? null, isWritable: true },
    bufferAccount: { value: input.bufferAccount ?? null, isWritable: true },
    spillAccount: { value: input.spillAccount ?? null, isWritable: true },
    rentSysvar: { value: input.rentSysvar ?? null, isWritable: false },
    clockSysvar: { value: input.clockSysvar ?? null, isWritable: false },
    authority: { value: input.authority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.rentSysvar.value) {
    accounts.rentSysvar.value =
      'SysvarRent111111111111111111111111111111111' as Address<'SysvarRent111111111111111111111111111111111'>;
  }
  if (!accounts.clockSysvar.value) {
    accounts.clockSysvar.value =
      'SysvarC1ock11111111111111111111111111111111' as Address<'SysvarC1ock11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.programDataAccount),
      getAccountMeta(accounts.programAccount),
      getAccountMeta(accounts.bufferAccount),
      getAccountMeta(accounts.spillAccount),
      getAccountMeta(accounts.rentSysvar),
      getAccountMeta(accounts.clockSysvar),
      getAccountMeta(accounts.authority),
    ],
    programAddress,
    data: getUpgradeInstructionDataEncoder().encode({}),
  } as UpgradeInstruction<
    TProgramAddress,
    TAccountProgramDataAccount,
    TAccountProgramAccount,
    TAccountBufferAccount,
    TAccountSpillAccount,
    TAccountRentSysvar,
    TAccountClockSysvar,
    TAccountAuthority
  >;

  return instruction;
}

export type ParsedUpgradeInstruction<
  TProgram extends string = typeof QED_DATA_LOADER_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** ProgramData account. */
    programDataAccount: TAccountMetas[0];
    /** Program account. */
    programAccount: TAccountMetas[1];
    /** Buffer account where the new program data has been written. */
    bufferAccount: TAccountMetas[2];
    /** Spill account. */
    spillAccount: TAccountMetas[3];
    /** Rent sysvar. */
    rentSysvar: TAccountMetas[4];
    /** Clock sysvar. */
    clockSysvar: TAccountMetas[5];
    /** Authority. */
    authority: TAccountMetas[6];
  };
  data: UpgradeInstructionData;
};

export function parseUpgradeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedUpgradeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 7) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      programDataAccount: getNextAccount(),
      programAccount: getNextAccount(),
      bufferAccount: getNextAccount(),
      spillAccount: getNextAccount(),
      rentSysvar: getNextAccount(),
      clockSysvar: getNextAccount(),
      authority: getNextAccount(),
    },
    data: getUpgradeInstructionDataDecoder().decode(instruction.data),
  };
}
