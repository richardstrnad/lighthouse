use anchor_lang::{self, InstructionData, ToAccountMetas};
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::BanksClient;
use solana_sdk::{
    instruction::AccountMeta,
    signature::{Keypair, Signature},
    transaction::Transaction,
};

use super::{clone_keypair, instruction, Error, Result};

// Helper object to execute and easily alter characteristics of transactions
// which contain a Lighthouse instruction. There's one instantiation for each
// particular operation (when T and U become concrete types), which are
// aliased a bit further below for convenience. The point of these objects
// is to be easy to set up for the common case of each operation, but at the
// same time easy to tweak via the pub fields and methods (including ones that
// can be added) such that it's easy to validate various test cases.
pub struct TxBuilder<T, U, V> {
    // This is the accounts structure that holds all the pubkeys, and for
    // each particular op we'll use the one generated by Anchor.
    pub accounts: T,
    pub additional_accounts: Vec<AccountMeta>,
    // Similar to the above, but for instruction data.
    pub data: U,
    // The currently configured payer for the tx.
    pub payer: Pubkey,
    pub client: BanksClient,
    // Currently configured signers for the tx. Using only `Keypair`s as
    // signers for now; can make this more generic if needed.
    pub signers: Vec<Keypair>,
    // pub tree: &'a mut Tree<MAX_DEPTH, MAX_BUFFER_SIZE>,
    // When some, indicates that a proof for the specified leaf index should
    // be computed from the inner tree and attached in the form of additional
    // accounts (but only if `self.additional_accounts.len() == 0`, so we
    // dont override a sequence of additional accounts explicitly set for
    // testing purposes).
    // pub need_proof: Option<u32>,
    // This member holds data of a custom type that can be specific to each kind
    // of transaction.
    pub inner: V,
}

pub trait OnSuccessfulTxExec {
    fn on_successful_execute(&mut self) -> Result<()>;
}

impl<'a, T, U, V> TxBuilder<T, U, V>
where
    T: ToAccountMetas,
    U: InstructionData,
{
    pub async fn execute(&mut self, additional_ix: Vec<Instruction>) -> Result<Signature>
    where
        Self: OnSuccessfulTxExec,
    {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        let mut ix = instruction(&self.accounts, &self.data);

        // if self.additional_accounts.is_empty() {
        //     // We're only automatically adding the proof if there are no additional
        //     // accounts explicitly configured.
        //     if let Some(leaf_idx) = self.need_proof {
        //         ix.accounts
        //             .append(&mut self.tree.proof_of_leaf_metas(leaf_idx))
        //     }
        // } else {
        //     // Add the additional accounts metas (if any).
        ix.accounts.append(&mut self.additional_accounts.clone());
        // }

        let ixs = vec![ix];
        let ixs = ixs
            .into_iter()
            .chain(additional_ix.into_iter())
            .collect::<Vec<_>>();

        let tx = &mut Transaction::new_with_payer(&ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        self.client
            .process_transaction(tx.clone())
            .await
            .map_err(Error::BanksClient)?;

        self.on_successful_execute()?;

        // Check the expected tree root matches on-chain state post tx.
        // self.tree.check_expected_root().await

        let signature = tx.signatures[0];

        Ok(signature)
    }

    pub async fn to_transaction(&mut self, additional_ix: Vec<Instruction>) -> Result<Transaction>
    where
        Self: OnSuccessfulTxExec,
    {
        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .await
            .map_err(Error::BanksClient)?;

        let mut ix = instruction(&self.accounts, &self.data);
        ix.accounts.append(&mut self.additional_accounts.clone());

        let ixs = &mut additional_ix
            .into_iter()
            .chain(vec![ix])
            .collect::<Vec<_>>();

        let tx = &mut Transaction::new_with_payer(&ixs, Some(&self.payer));

        // Using `try_partial_sign` to avoid panics (and get an error when something is
        // wrong instead) no matter what signers are configured.
        tx.try_partial_sign(&self.signers.iter().collect::<Vec<_>>(), recent_blockhash)
            .map_err(Error::Signer)?;

        Ok(tx.clone())
    }

    // Returning `&mut Self` to allow method chaining.
    pub fn set_signers(&mut self, signers: &[&Keypair]) -> &mut Self {
        self.signers = signers.iter().map(|k| clone_keypair(k)).collect();
        self
    }

    pub fn set_payer(&mut self, key: Pubkey) -> &mut Self {
        self.payer = key;
        self
    }

    pub fn set_additional_account_metas(&mut self, metas: &[AccountMeta]) -> &mut Self {
        self.additional_accounts = metas.to_vec();
        self
    }

    // Populate the `additional_account` member with read-only and non-signer accounts based
    // on the provided public keys.
    pub fn set_additional_accounts(&mut self, keys: &[Pubkey]) -> &mut Self {
        self.additional_accounts = keys
            .iter()
            .map(|key| AccountMeta::new_readonly(*key, false))
            .collect();
        self
    }

    pub fn set_additional_accounts_from_arrays(&mut self, keys: &[[u8; 32]]) -> &mut Self {
        self.set_additional_accounts(
            keys.iter()
                .copied()
                .map(Pubkey::new_from_array)
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

// The types below have "builder" in their names because we're essentially
// implementing a lightweight builder patter to instantiate, customize, and
// execute transactions.
pub type AssertBuilder<'a> =
    TxBuilder<lighthouse::accounts::AssertV1, lighthouse::instruction::AssertV1, ()>;

impl<'a> OnSuccessfulTxExec for AssertBuilder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CreateCacheAccountBuilder<'a> = TxBuilder<
    lighthouse::accounts::CreateCacheAccountV1,
    lighthouse::instruction::CreateCacheAccountV1,
    (),
>;

impl<'a> OnSuccessfulTxExec for CreateCacheAccountBuilder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CacheLoadAccountV1Builder<'a> = TxBuilder<
    lighthouse::accounts::CacheLoadAccountV1,
    lighthouse::instruction::CacheLoadAccountV1,
    (),
>;

impl<'a> OnSuccessfulTxExec for CacheLoadAccountV1Builder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}

pub type CreateTestAccountV1Builder<'a> = TxBuilder<
    lighthouse::accounts::CreateTestAccountV1,
    lighthouse::instruction::CreateTestAccountV1,
    (),
>;

impl<'a> OnSuccessfulTxExec for CreateTestAccountV1Builder<'a> {
    fn on_successful_execute(&mut self) -> Result<()> {
        // Do nothing here.
        Ok(())
    }
}
