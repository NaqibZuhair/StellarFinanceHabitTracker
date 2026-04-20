#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

// Struktur data untuk menyimpan transaksi finance
#[contracttype]
#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: u64,
    pub owner: Address,
    pub kind: u32,
    pub category: String,
    pub amount: i128,
    pub description: String,
    pub period: u32,
}

// Struktur data untuk ringkasan bulanan
#[contracttype]
#[derive(Clone, Debug)]
pub struct Summary {
    pub total_income: i128,
    pub total_expense: i128,
    pub balance: i128,
}

// Storage key
const TRANSACTION_DATA: Symbol = symbol_short!("TX_DATA");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct FinanceContract;

#[contractimpl]
impl FinanceContract {
    // Fungsi untuk mendapatkan semua transaksi
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        return env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk menambahkan transaksi baru
    pub fn add_transaction(
        env: Env,
        owner: Address,
        kind: u32,
        category: String,
        amount: i128,
        description: String,
        period: u32,
    ) -> String {
        owner.require_auth();

        if amount <= 0 {
            return String::from_str(&env, "Amount harus lebih dari 0");
        }

        if kind != 1 && kind != 2 {
            return String::from_str(&env, "Kind harus 1 income atau 2 expense");
        }

        // 1. ambil data transaksi lama dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. ambil id berikutnya
        let next_id: u64 = env.storage().instance().get(&NEXT_ID).unwrap_or(1);

        // 3. buat object transaksi baru
        let transaction = Transaction {
            id: next_id,
            owner,
            kind,
            category,
            amount,
            description,
            period,
        };

        // 4. tambahkan transaksi baru ke data lama
        transactions.push_back(transaction);

        // 5. simpan transaksi ke storage
        env.storage()
            .instance()
            .set(&TRANSACTION_DATA, &transactions);

        // 6. simpan id berikutnya
        env.storage().instance().set(&NEXT_ID, &(next_id + 1));

        return String::from_str(&env, "Transaksi berhasil ditambahkan");
    }

    // Fungsi untuk mengubah transaksi berdasarkan id
    pub fn update_transaction(
        env: Env,
        owner: Address,
        id: u64,
        kind: u32,
        category: String,
        amount: i128,
        description: String,
        period: u32,
    ) -> String {
        owner.require_auth();

        if amount <= 0 {
            return String::from_str(&env, "Amount harus lebih dari 0");
        }

        if kind != 1 && kind != 2 {
            return String::from_str(&env, "Kind harus 1 income atau 2 expense");
        }

        // 1. ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. cari transaksi berdasarkan id
        for i in 0..transactions.len() {
            let old_transaction = transactions.get(i).unwrap();

            if old_transaction.id == id && old_transaction.owner == owner {
                let updated_transaction = Transaction {
                    id,
                    owner,
                    kind,
                    category,
                    amount,
                    description,
                    period,
                };

                transactions.set(i, updated_transaction);

                env.storage()
                    .instance()
                    .set(&TRANSACTION_DATA, &transactions);

                return String::from_str(&env, "Transaksi berhasil diubah");
            }
        }

        return String::from_str(&env, "Transaksi tidak ditemukan");
    }

    // Fungsi untuk menghapus transaksi berdasarkan id
    pub fn delete_transaction(env: Env, owner: Address, id: u64) -> String {
        owner.require_auth();

        // 1. ambil data transaksi dari storage
        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. cari transaksi berdasarkan id
        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.id == id && transaction.owner == owner {
                transactions.remove(i);

                env.storage()
                    .instance()
                    .set(&TRANSACTION_DATA, &transactions);

                return String::from_str(&env, "Transaksi berhasil dihapus");
            }
        }

        return String::from_str(&env, "Transaksi tidak ditemukan");
    }

    // Fungsi untuk mendapatkan ringkasan bulanan
    pub fn get_monthly_summary(env: Env, owner: Address, period: u32) -> Summary {
        let transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        let mut total_income: i128 = 0;
        let mut total_expense: i128 = 0;

        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.owner == owner && transaction.period == period {
                if transaction.kind == 1 {
                    total_income += transaction.amount;
                }

                if transaction.kind == 2 {
                    total_expense += transaction.amount;
                }
            }
        }

        return Summary {
            total_income,
            total_expense,
            balance: total_income - total_expense,
        };
    }
}

#[cfg(test)]
mod test;