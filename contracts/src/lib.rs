#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod storage_types;
pub use storage_types::*;

#[contract]
pub struct NesteraContract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl NesteraContract {
    /// Withdraw funds from a Lock Save plan after maturity
    /// 
    /// # Arguments
    /// * `env` - The contract environment
    /// * `user` - The address of the user who owns the lock
    /// * `plan_id` - The ID of the savings plan (lock) to withdraw from
    /// 
    /// # Returns
    /// * `Result<i128, SavingsError>` - The withdrawn amount on success, or an error
    pub fn withdraw_lock_save(
        env: Env,
        user: Address,
        plan_id: u64,
    ) -> Result<i128, SavingsError> {
        // Authorize the caller
        user.require_auth();

        // Ensure user exists
        let user_key = DataKey::User(user.clone());
        let mut user_data: User = env
            .storage()
            .persistent()
            .get(&user_key)
            .ok_or(SavingsError::UserNotFound)?;

        // Fetch the savings plan
        let plan_key = DataKey::SavingsPlan(user.clone(), plan_id);
        let mut plan: SavingsPlan = env
            .storage()
            .persistent()
            .get(&plan_key)
            .ok_or(SavingsError::PlanNotFound)?;

        // Validate that this is a Lock plan and get maturity time
        let maturity_time = match plan.plan_type {
            PlanType::Lock(maturity) => maturity,
            _ => return Err(SavingsError::PlanNotFound),
        };

        // Check if the lock has matured
        let current_timestamp = env.ledger().timestamp();
        if current_timestamp < maturity_time {
            return Err(SavingsError::LockNotMatured);
        }

        // Check that it hasn't been withdrawn already
        if plan.is_withdrawn {
            return Err(SavingsError::AlreadyWithdrawn);
        }

        // Get the withdrawal amount
        let withdrawal_amount = plan.balance;

        // Mark as withdrawn
        plan.is_withdrawn = true;
        plan.balance = 0;
        plan.last_withdraw = current_timestamp;
        plan.is_completed = true;

        // Update user's total balance (subtract withdrawn amount)
        user_data.total_balance = user_data
            .total_balance
            .checked_sub(withdrawal_amount)
            .ok_or(SavingsError::InsufficientBalance)?;

        // Save updated data
        env.storage().persistent().set(&plan_key, &plan);
        env.storage().persistent().set(&user_key, &user_data);

        Ok(withdrawal_amount)
    }
}

mod test;
