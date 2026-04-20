# Stellar Finance Habit Tracker

Stellar Finance Habit Tracker is a Web3 personal finance tracker built on Stellar Soroban.  
The app helps users record income and expense transactions, categorize spending, and view monthly financial summaries using smart contract storage on Stellar Testnet.

## Problem

Many students and young professionals do not have a simple way to track daily income and expenses.  
This project solves that problem by providing a wallet-based finance tracker where transaction records are stored through a Soroban smart contract.

## Features

- Add income transaction
- Add expense transaction
- View all transactions
- Update transaction
- Delete transaction
- View monthly summary
- Calculate total income
- Calculate total expense
- Calculate balance

## Smart Contract Functions

- add_transaction
- get_transactions
- update_transaction
- delete_transaction
- get_monthly_summary

## Tech Stack

- Stellar
- Soroban
- Rust
- Soroban SDK
- Stellar CLI
- Stellar Testnet

## Contract ID

CAPNKGWAS26DC2LZDD7CZ5C5YHEE6RB7YO32MN3AHYNV7ULI6FYOK5MW

## Testnet Result

The smart contract has been deployed to Stellar Testnet.

Contract Explorer:

https://lab.stellar.org/r/testnet/contract/CAPNKGWAS26DC2LZDD7CZ5C5YHEE6RB7YO32MN3AHYNV7ULI6FYOK5MW

## Sample Test Data

Income:

- Salary, 3000000

Expense:

- Food, 75000
- Transport, 50000

Monthly Summary:

- Total income: 6000000
- Total expense: 125000
- Balance: 5875000

## How to Build

```bash
stellar contract build