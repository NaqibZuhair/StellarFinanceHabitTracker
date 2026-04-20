# Stellar Finance Habit Tracker

Stellar Finance Habit Tracker is a Web3 personal finance smart contract built on Stellar Soroban.

This project allows users to record income and expense transactions, categorize spending, view monthly financial summaries, and calculate a simple financial health score using wallet-owned data on Stellar Testnet.

## Problem

Many students and young professionals do not have a simple and consistent way to track daily income and expenses.

Because of that, they often lose visibility into their spending habits, monthly cash flow, and financial condition.

## Solution

Stellar Finance Habit Tracker provides a wallet-based finance tracking system.

Users can add income and expense records through a Soroban smart contract. Each transaction is linked to a Stellar wallet address. The contract can calculate monthly income, expense, balance, and financial health score.

## Why Web3

This project uses Web3 because wallet address can act as user identity.

Instead of using email login or centralized account storage, each transaction is connected to a Stellar wallet address. The smart contract stores transaction records and allows the result to be verified on Stellar Testnet.

This project uses simulated financial data for Testnet. For production use, sensitive financial details should be stored off-chain, while the smart contract stores proofs, summaries, or non-sensitive records.

## Features

- Add income transaction
- Add expense transaction
- View all transactions
- View transactions by owner
- Update transaction
- Delete transaction
- View monthly summary
- Calculate total income
- Calculate total expense
- Calculate balance
- Calculate financial health score
- Count total transactions

## Smart Contract Functions

- add_transaction
- get_transactions
- get_transactions_by_owner
- update_transaction
- delete_transaction
- get_monthly_summary
- get_financial_health
- get_transaction_count

## What Makes This Project Different

Most workshop starter projects only store simple notes.

This project changes the use case into a personal finance habit tracker. Each record has financial meaning because income and expense transactions affect monthly summary, balance, saving rate, and financial health score.

The project is not only a basic CRUD smart contract. It includes transaction categorization, monthly cash flow calculation, and a simple financial insight layer.

## Financial Health Score

The app calculates a simple financial health score based on monthly saving rate.


<img width="1460" height="624" alt="image" src="https://github.com/user-attachments/assets/f3258671-3f54-49eb-a6ef-68ab6d948a49" />


Formula:

```text
saving_rate = balance / total_income x 100


