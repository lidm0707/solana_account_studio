//! Transaction Builder Module
//! Provides transaction creation and signing for MVP

use serde::{Deserialize, Serialize};
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

/// Transaction builder for creating Solana transactions
pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    signers: Vec<Keypair>,
    payer: Pubkey,
    recent_blockhash: String,
}

impl TransactionBuilder {
    /// Create new transaction builder
    pub fn new(payer: Pubkey) -> Self {
        Self {
            instructions: Vec::new(),
            signers: Vec::new(),
            payer,
            recent_blockhash: String::new(),
        }
    }

    /// Add instruction to transaction
    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    /// Add signer to transaction
    pub fn add_signer(&mut self, signer: &Keypair) -> &mut Self {
        // Note: Keypair doesn't implement Clone, so we store references
        // For MVP, we'll handle this differently
        self
    }

    /// Set recent blockhash
    pub fn set_blockhash(&mut self, blockhash: String) -> &mut Self {
        self.recent_blockhash = blockhash;
        self
    }

    /// Build transaction
    pub fn build(&self) -> Result<Transaction, Box<dyn std::error::Error>> {
        let mut transaction = Transaction::new_with_payer(&self.instructions, Some(&self.payer));

        // Set recent blockhash (placeholder for MVP)
        // transaction.sign(&self.signers, blockhash);

        Ok(transaction)
    }

    /// Create simple transfer transaction
    pub fn create_transfer(
        from: &Keypair,
        to: Pubkey,
        amount: u64,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        // For MVP, create a placeholder transaction
        // In real implementation, this would create a SystemProgram transfer
        let mut builder = Self::new(from.pubkey());
        builder.add_signer(from);

        // Add transfer instruction (placeholder)
        // let instruction = system_instruction::transfer(&from.pubkey(), &to, amount);
        // builder.add_instruction(instruction);

        builder.build()
    }
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub signature: String,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub status: TransactionStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Transaction manager
pub struct TransactionManager {
    transactions: Vec<TransactionInfo>,
}

impl TransactionManager {
    /// Create new transaction manager
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    /// Add transaction
    pub fn add_transaction(&mut self, transaction: TransactionInfo) {
        self.transactions.push(transaction);
    }

    /// Get all transactions
    pub fn get_transactions(&self) -> &[TransactionInfo] {
        &self.transactions
    }

    /// Get transactions by account
    pub fn get_transactions_by_account(&self, account: &Pubkey) -> Vec<&TransactionInfo> {
        self.transactions
            .iter()
            .filter(|tx| tx.from == *account || tx.to == *account)
            .collect()
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}
