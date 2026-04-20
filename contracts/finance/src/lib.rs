#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

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

#[contracttype]
#[derive(Clone, Debug)]
pub struct Summary {
    pub total_income: i128,
    pub total_expense: i128,
    pub balance: i128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct HealthInsight {
    pub saving_rate: i128,
    pub score: u32,
    pub status: String,
}

const TRANSACTION_DATA: Symbol = symbol_short!("TX_DATA");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct FinanceContract;

#[contractimpl]
impl FinanceContract {
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

        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        let next_id: u64 = env
            .storage()
            .instance()
            .get(&NEXT_ID)
            .unwrap_or(1);

        let transaction = Transaction {
            id: next_id,
            owner,
            kind,
            category,
            amount,
            description,
            period,
        };

        transactions.push_back(transaction);

        env.storage()
            .instance()
            .set(&TRANSACTION_DATA, &transactions);

        env.storage()
            .instance()
            .set(&NEXT_ID, &(next_id + 1));

        return String::from_str(&env, "Transaksi berhasil ditambahkan");
    }

    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        return env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));
    }

    pub fn get_transactions_by_owner(env: Env, owner: Address) -> Vec<Transaction> {
        let transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result: Vec<Transaction> = Vec::new(&env);

        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();

            if transaction.owner == owner {
                result.push_back(transaction);
            }
        }

        return result;
    }

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

        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

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

    pub fn delete_transaction(env: Env, owner: Address, id: u64) -> String {
        owner.require_auth();

        let mut transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

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

    pub fn get_financial_health(env: Env, owner: Address, period: u32) -> HealthInsight {
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

        if total_income <= 0 {
            return HealthInsight {
                saving_rate: 0,
                score: 0,
                status: String::from_str(&env, "NoIncome"),
            };
        }

        let balance = total_income - total_expense;
        let saving_rate = (balance * 100) / total_income;

        let mut score: u32 = 0;

        if saving_rate > 0 {
            if saving_rate >= 100 {
                score = 100;
            } else {
                score = saving_rate as u32;
            }
        }

        let status = if score >= 80 {
            String::from_str(&env, "Healthy")
        } else if score >= 50 {
            String::from_str(&env, "Watchful")
        } else {
            String::from_str(&env, "Risky")
        };

        return HealthInsight {
            saving_rate,
            score,
            status,
        };
    }

    pub fn get_transaction_count(env: Env) -> u32 {
        let transactions: Vec<Transaction> = env
            .storage()
            .instance()
            .get(&TRANSACTION_DATA)
            .unwrap_or(Vec::new(&env));

        return transactions.len();
    }
}

#[cfg(test)]
mod test;