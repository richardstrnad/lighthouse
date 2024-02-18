use crate::TxBuilder;
use anchor_lang::*;
use borsh::BorshSerialize;
use lighthouse::types::{Assertion, AssertionConfigV1, WriteTypeParameter};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::signature::{Keypair, Signer};

pub struct LighthouseProgram {}

impl<'a> LighthouseProgram {
    fn tx_builder(&mut self, ixs: Vec<Instruction>, payer: Pubkey) -> TxBuilder {
        TxBuilder {
            payer,
            ixs,
            look_up_tables: vec![],
        }
    }

    pub fn entrypoint(&mut self, payer: &Keypair) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![],
                data: vec![],
            }],
            payer.pubkey(),
        )
    }

    pub fn create_assert(
        &'a mut self,
        payer: &'a Keypair,
        target_account: Pubkey,
        assertion: Assertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        match assertion {
            Assertion::AccountInfoField(field) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertAccountInfo
                        .to_vec()
                        .into_iter()
                        .chain((field).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
            Assertion::AccountData(offset, data_value) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertAccountData
                        .to_vec()
                        .into_iter()
                        .chain((offset, data_value).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
            Assertion::MintAccountField(field) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertMintAccountField
                        .to_vec()
                        .into_iter()
                        .chain((field).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
            Assertion::TokenAccountField(field) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertTokenAccountField
                        .to_vec()
                        .into_iter()
                        .chain((field).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
            Assertion::SysvarClockField(field) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertSysvarClockField
                        .to_vec()
                        .into_iter()
                        .chain((field).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
            Assertion::AccountDataHash(hash, operator, start, end) => self.tx_builder(
                vec![Instruction {
                    program_id: lighthouse::ID,
                    accounts: vec![
                        AccountMeta::new_readonly(lighthouse::ID, false),
                        AccountMeta::new_readonly(target_account, false),
                    ],
                    data: lighthouse::LighthouseInstruction::AssertDataHash
                        .to_vec()
                        .into_iter()
                        .chain((hash, operator, start, end).try_to_vec().unwrap())
                        .collect(),
                }],
                payer.pubkey(),
            ),
        }

        // self.tx_builder(
        //     vec![Instruction {
        //         program_id: lighthouse::id(),
        //         accounts: vec![
        //             AccountMeta::new_readonly(lighthouse::ID, true),
        //             AccountMeta::new_readonly(target_account, false),
        //         ],
        //         data: lighthouse::LighthouseInstruction::AssertAccountInfo
        //             .to_vec()
        //             .into_iter()
        //             .chain(assertion.try_to_vec().unwrap())
        //             .collect(),
        //     }],
        //     payer.pubkey(),
        // )
    }

    // pub fn create_assert_compact(
    //     &mut self,
    //     payer: &Keypair,
    //     target_account: Pubkey,
    //     assertion: Assertion,
    // ) -> TxBuilder {
    //     self.tx_builder(
    //         vec![Instruction {
    //             program_id: lighthouse::id(),
    //             accounts: (lighthouse::accounts::AssertCompactV1 { target_account })
    //                 .to_account_metas(None),
    //             data: lighthouse::instruction::AssertCompactV1 { assertion }.data(),
    //         }],
    //         payer.pubkey(),
    //     )
    // }

    pub fn create_assert_multi(
        &mut self,
        payer: &Keypair,
        assertions: Vec<Assertion>,
        additional_accounts: Vec<Pubkey>,
    ) -> TxBuilder {
        // append additional_accounts to accounts
        // accounts.append(
        //     &mut additional_accounts
        //         .into_iter()
        //         .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
        //         .collect(),
        // );

        let mut ixs = vec![];

        for (i, assertion) in assertions.into_iter().enumerate() {
            let accounts = vec![
                AccountMeta::new_readonly(lighthouse::id(), false),
                AccountMeta::new_readonly(
                    additional_accounts[i % additional_accounts.len()],
                    false,
                ),
            ];

            match assertion {
                Assertion::AccountData(offset, data_value) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertAccountData
                            .to_vec()
                            .into_iter()
                            .chain((offset, data_value).try_to_vec().unwrap())
                            .collect(),
                    });
                }
                Assertion::AccountInfoField(field) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertAccountInfo
                            .to_vec()
                            .into_iter()
                            .chain((field).try_to_vec().unwrap())
                            .collect(),
                    });
                }
                Assertion::MintAccountField(field) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertMintAccountField
                            .to_vec()
                            .into_iter()
                            .chain((field).try_to_vec().unwrap())
                            .collect(),
                    });
                }
                Assertion::TokenAccountField(field) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertTokenAccountField
                            .to_vec()
                            .into_iter()
                            .chain((field).try_to_vec().unwrap())
                            .collect(),
                    });
                }
                Assertion::SysvarClockField(field) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertSysvarClockField
                            .to_vec()
                            .into_iter()
                            .chain((field).try_to_vec().unwrap())
                            .collect(),
                    });
                }
                Assertion::AccountDataHash(hash, operator, start, end) => {
                    ixs.push(Instruction {
                        program_id: lighthouse::id(),
                        accounts: accounts.clone(),
                        data: lighthouse::LighthouseInstruction::AssertDataHash
                            .to_vec()
                            .into_iter()
                            .chain((hash, operator, start, end).try_to_vec().unwrap())
                            .collect(),
                    });
                }
            }
        }

        self.tx_builder(ixs, payer.pubkey())
    }

    // pub fn create_assert_multi_compact(
    //     &mut self,
    //     payer: &Keypair,
    //     assertions: Vec<Assertion>,
    //     additional_accounts: Vec<Pubkey>,
    // ) -> TxBuilder {
    //     let mut accounts = (lighthouse::accounts::AssertMultiCompactV1 {
    //         system_program: system_program::id(),
    //     })
    //     .to_account_metas(None);

    //     // append additional_accounts to accounts
    //     accounts.append(
    //         &mut additional_accounts
    //             .into_iter()
    //             .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
    //             .collect(),
    //     );

    //     let assertion_array: CompactAssertionArray = match assertions.len() {
    //         1 => CompactAssertionArray::Size1([assertions[0].clone()]),
    //         2 => CompactAssertionArray::Size2([assertions[0].clone(), assertions[1].clone()]),
    //         3 => CompactAssertionArray::Size3([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //         ]),
    //         4 => CompactAssertionArray::Size4([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //         ]),
    //         5 => CompactAssertionArray::Size5([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //         ]),
    //         6 => CompactAssertionArray::Size6([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //         ]),
    //         7 => CompactAssertionArray::Size7([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //         ]),
    //         8 => CompactAssertionArray::Size8([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //         ]),
    //         9 => CompactAssertionArray::Size9([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //         ]),
    //         10 => CompactAssertionArray::Size10([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //         ]),
    //         11 => CompactAssertionArray::Size11([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //         ]),
    //         12 => CompactAssertionArray::Size12([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //             assertions[11].clone(),
    //         ]),
    //         13 => CompactAssertionArray::Size13([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //             assertions[11].clone(),
    //             assertions[12].clone(),
    //         ]),
    //         14 => CompactAssertionArray::Size14([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //             assertions[11].clone(),
    //             assertions[12].clone(),
    //             assertions[13].clone(),
    //         ]),
    //         15 => CompactAssertionArray::Size15([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //             assertions[11].clone(),
    //             assertions[12].clone(),
    //             assertions[13].clone(),
    //             assertions[14].clone(),
    //         ]),
    //         16 => CompactAssertionArray::Size16([
    //             assertions[0].clone(),
    //             assertions[1].clone(),
    //             assertions[2].clone(),
    //             assertions[3].clone(),
    //             assertions[4].clone(),
    //             assertions[5].clone(),
    //             assertions[6].clone(),
    //             assertions[7].clone(),
    //             assertions[8].clone(),
    //             assertions[9].clone(),
    //             assertions[10].clone(),
    //             assertions[11].clone(),
    //             assertions[12].clone(),
    //             assertions[13].clone(),
    //             assertions[14].clone(),
    //             assertions[15].clone(),
    //         ]),
    //         _ => panic!("Too many assertions for compact array instruction!"),
    //     };

    //     self.tx_builder(
    //         vec![Instruction {
    //             program_id: lighthouse::id(),
    //             accounts,
    //             data: (lighthouse::instruction::AssertMultiCompactV1 {
    //                 assertions: assertion_array,
    //             })
    //             .data(),
    //         }],
    //         payer.pubkey(),
    //     )
    // }

    pub fn create_memory_account(
        &mut self,
        payer: &Keypair,
        memory_index: u8,
        memory_account_size: u64,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: vec![
                    AccountMeta::new_readonly(lighthouse::id(), false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(find_memory_account(payer.pubkey(), memory_index).0, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: lighthouse::LighthouseInstruction::CreateMemoryAccount
                    .to_vec()
                    .into_iter()
                    .chain(vec![memory_index])
                    .chain(memory_account_size.to_le_bytes().to_vec())
                    .collect(),
            }],
            payer.pubkey(),
        )
    }

    // pub fn close_memory_account(&mut self, payer: &Keypair, memory_index: u8) -> TxBuilder {
    //     let (memory_account, memory_account_bump) =
    //         find_memory_account(payer.pubkey(), memory_index);

    //     self.tx_builder(
    //         vec![Instruction {
    //             program_id: lighthouse::id(),
    //             accounts: (lighthouse::accounts::CloseMemoryAccountV1 {
    //                 signer: payer.pubkey(),
    //                 system_program: system_program::id(),
    //                 memory_account,
    //                 rent: sysvar::rent::id(),
    //             })
    //             .to_account_metas(None),
    //             data: lighthouse::instruction::CloseMemoryAccountV1 {
    //                 memory_index,
    //                 memory_bump: memory_account_bump,
    //             }
    //             .data(),
    //         }],
    //         payer.pubkey(),
    //     )
    // }

    pub fn write_v1(
        &mut self,
        payer: &Keypair,
        source_account: Pubkey,
        memory_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> TxBuilder {
        let (memory_account, memory_account_bump) =
            find_memory_account(payer.pubkey(), memory_index);

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: vec![
                    AccountMeta::new_readonly(lighthouse::id(), false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(memory_account, false),
                    AccountMeta::new_readonly(source_account, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: lighthouse::LighthouseInstruction::Write
                    .to_vec()
                    .into_iter()
                    .chain(vec![memory_index])
                    .chain(vec![memory_account_bump])
                    .chain(write_type_parameter.try_to_vec().unwrap())
                    .collect(),
            }],
            payer.pubkey(),
        )
    }
}

pub fn find_memory_account(user: Pubkey, memory_index: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[
            "memory".to_string().as_ref(),
            user.as_ref(),
            &[memory_index],
        ],
        &lighthouse::ID,
    )
}
