#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Ledger}, Address, Env};

#[test]
fn test_user_instantiation() {
    let user = User {
        total_balance: 1_000_000,
        savings_count: 3,
    };
    
    assert_eq!(user.total_balance, 1_000_000);
    assert_eq!(user.savings_count, 3);
}

#[test]
fn test_flexi_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 1,
        plan_type: PlanType::Flexi,
        balance: 500_000,
        start_time: 1000000,
        last_deposit: 1000100,
        last_withdraw: 0,
        interest_rate: 500, // 5.00% APY
        is_completed: false,
        is_withdrawn: false,
    };
    
    assert_eq!(plan.plan_id, 1);
    assert_eq!(plan.plan_type, PlanType::Flexi);
    assert_eq!(plan.balance, 500_000);
    assert!(!plan.is_completed);
}

#[test]
fn test_lock_savings_plan() {
    let locked_until = 2000000;
    let plan = SavingsPlan {
        plan_id: 2,
        plan_type: PlanType::Lock(locked_until),
        balance: 1_000_000,
        start_time: 1000000,
        last_deposit: 1000000,
        last_withdraw: 0,
        interest_rate: 800,
        is_completed: false,
        is_withdrawn: false,
    };
    
    assert_eq!(plan.plan_id, 2);
    match plan.plan_type {
        PlanType::Lock(until) => assert_eq!(until, locked_until),
        _ => panic!("Expected Lock plan type"),
    }
}

#[test]
fn test_goal_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 3,
        plan_type: PlanType::Goal(
            symbol_short!("education"),
            5_000_000,
            1u32, // e.g. 1 = weekly
        ),
        balance: 2_000_000,
        start_time: 1000000,
        last_deposit: 1500000,
        last_withdraw: 0,
        interest_rate: 600,
        is_completed: false,
        is_withdrawn: false,
    };
    
    assert_eq!(plan.plan_id, 3);
    match plan.plan_type {
        PlanType::Goal(category, target_amount, contribution_type) => {
            assert_eq!(category, symbol_short!("education"));
            assert_eq!(target_amount, 5_000_000);
            assert_eq!(contribution_type, 1u32);
        },
        _ => panic!("Expected Goal plan type"),
    }
}

#[test]
fn test_group_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 4,
        plan_type: PlanType::Group(
            101,
            true,
            2u32,
            10_000_000
        ),
        balance: 3_000_000,
        start_time: 1000000,
        last_deposit: 1600000,
        last_withdraw: 0,
        interest_rate: 700,
        is_completed: false,
        is_withdrawn: false,
    };
    
    assert_eq!(plan.plan_id, 4);
    match plan.plan_type {
        PlanType::Group(group_id, is_public, contribution_type, target_amount) => {
            assert_eq!(group_id, 101);
            assert!(is_public);
            assert_eq!(contribution_type, 2u32);
            assert_eq!(target_amount, 10_000_000);
        },
        _ => panic!("Expected Group plan type"),
    }
}

#[test]
fn test_data_key_admin() {
    let key = DataKey::Admin;
    assert_eq!(key, DataKey::Admin);
}

#[test]
fn test_data_key_user() {
    let env = Env::default();
    let user_address = Address::generate(&env);
    let key = DataKey::User(user_address.clone());
    
    match key {
        DataKey::User(addr) => assert_eq!(addr, user_address),
        _ => panic!("Expected User data key"),
    }
}

#[test]
fn test_data_key_savings_plan() {
    let env = Env::default();
    let user_address = Address::generate(&env);
    let plan_id = 42;
    let key = DataKey::SavingsPlan(user_address.clone(), plan_id);
    
    match key {
        DataKey::SavingsPlan(addr, id) => {
            assert_eq!(addr, user_address);
            assert_eq!(id, plan_id);
        },
        _ => panic!("Expected SavingsPlan data key"),
    }
}

#[test]
fn test_xdr_compatibility_user() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());

    let user = User {
        total_balance: 1_500_000,
        savings_count: 5,
    };

    let key = symbol_short!("testuser");
    env.as_contract(&contract_id, || {
        env.storage().instance().set(&key, &user);
        let retrieved_user: User = env.storage().instance().get(&key).unwrap();
        assert_eq!(user, retrieved_user);
    });
}

#[test]
fn test_xdr_compatibility_savings_plan() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    
    let plan = SavingsPlan {
        plan_id: 1,
        plan_type: PlanType::Flexi,
        balance: 750_000,
        start_time: 1000000,
        last_deposit: 1100000,
        last_withdraw: 1050000,
        interest_rate: 550,
        is_completed: false,
        is_withdrawn: false,
    };
    
    let key = symbol_short!("testplan");
    env.as_contract(&contract_id, || {
        env.storage().instance().set(&key, &plan);
        let retrieved_plan: SavingsPlan = env.storage().instance().get(&key).unwrap();
        assert_eq!(plan, retrieved_plan);
    });
}

#[test]
fn test_xdr_compatibility_all_plan_types() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    
    env.as_contract(&contract_id, || {
        // Test Flexi
        let flexi_plan = SavingsPlan {
            plan_id: 0,
            plan_type: PlanType::Flexi,
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
            is_withdrawn: false,
        };
        env.storage().instance().set(&0u32, &flexi_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&0u32).unwrap();
        assert_eq!(flexi_plan, retrieved);
        
        // Test Lock
        let lock_plan = SavingsPlan {
            plan_id: 1,
            plan_type: PlanType::Lock(2000000),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
            is_withdrawn: false,
        };
        env.storage().instance().set(&1u32, &lock_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&1u32).unwrap();
        assert_eq!(lock_plan, retrieved);
        
        // Test Goal
        let goal_plan = SavingsPlan {
            plan_id: 2,
            plan_type: PlanType::Goal(
                symbol_short!("vacation"),
                3_000_000,
                1u32
            ),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
            is_withdrawn: false,
        };
        env.storage().instance().set(&2u32, &goal_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&2u32).unwrap();
        assert_eq!(goal_plan, retrieved);
        
        // Test Group
        let group_plan = SavingsPlan {
            plan_id: 3,
            plan_type: PlanType::Group(
                200,
                false,
                3u32,
                8_000_000
            ),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
            is_withdrawn: false,
        };
        env.storage().instance().set(&3u32, &group_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&3u32).unwrap();
        assert_eq!(group_plan, retrieved);
    });
}

#[test]
fn test_completed_plan() {
    let plan = SavingsPlan {
        plan_id: 5,
        plan_type: PlanType::Goal(
            symbol_short!("house"),
            10_000_000,
            2u32
        ),
        balance: 10_000_000,
        start_time: 1000000,
        last_deposit: 2000000,
        last_withdraw: 0,
        interest_rate: 650,
        is_completed: true,
        is_withdrawn: false,
    };
    
    assert!(plan.is_completed);
    assert_eq!(plan.balance, 10_000_000);
}

#[test]
fn test_plan_type_patterns() {
    // Test that we can extract values from each plan type variant
    let lock_plan = PlanType::Lock(1234567);
    if let PlanType::Lock(timestamp) = lock_plan {
        assert_eq!(timestamp, 1234567);
    }
    
    let goal_plan = PlanType::Goal(symbol_short!("car"), 2_000_000, 3u32);
    if let PlanType::Goal(cat, amount, contrib) = goal_plan {
        assert_eq!(cat, symbol_short!("car"));
        assert_eq!(amount, 2_000_000);
        assert_eq!(contrib, 3u32);
    }
    
    let group_plan = PlanType::Group(999, true, 1u32, 5_000_000);
    if let PlanType::Group(id, public, contrib, amount) = group_plan {
        assert_eq!(id, 999);
        assert!(public);
        assert_eq!(contrib, 1u32);
        assert_eq!(amount, 5_000_000);
    }
}

// ========== Lock Save Withdrawal Tests ==========

#[test]
fn test_withdraw_lock_save_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user data
    let user = User {
        total_balance: 1_000_000,
        savings_count: 1,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Set up lock savings plan with maturity time in the past
    let maturity_time = 1000000u64;
    let lock_plan = SavingsPlan {
        plan_id,
        plan_type: PlanType::Lock(maturity_time),
        balance: 500_000,
        start_time: 900000,
        last_deposit: 900000,
        last_withdraw: 0,
        interest_rate: 800,
        is_completed: false,
        is_withdrawn: false,
    };
    let plan_key = DataKey::SavingsPlan(user_addr.clone(), plan_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&plan_key, &lock_plan);
    });
    
    // Set ledger timestamp to after maturity
    env.ledger().with_mut(|li| {
        li.timestamp = maturity_time + 100;
    });
    
    // Perform withdrawal
    let withdrawn = client.withdraw_lock_save(&user_addr, &plan_id);
    assert_eq!(withdrawn, 500_000);
    
    // Verify plan is marked as withdrawn
    env.as_contract(&contract_id, || {
        let updated_plan: SavingsPlan = env.storage().persistent().get(&plan_key).unwrap();
        assert!(updated_plan.is_withdrawn);
        assert_eq!(updated_plan.balance, 0);
        assert!(updated_plan.is_completed);
        assert_eq!(updated_plan.last_withdraw, maturity_time + 100);
    });
    
    // Verify user balance is updated
    env.as_contract(&contract_id, || {
        let updated_user: User = env.storage().persistent().get(&user_key).unwrap();
        assert_eq!(updated_user.total_balance, 500_000);
    });
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_withdraw_lock_save_not_matured() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user data
    let user = User {
        total_balance: 1_000_000,
        savings_count: 1,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Set up lock savings plan with maturity time in the future
    let maturity_time = 2000000u64;
    let lock_plan = SavingsPlan {
        plan_id,
        plan_type: PlanType::Lock(maturity_time),
        balance: 500_000,
        start_time: 1000000,
        last_deposit: 1000000,
        last_withdraw: 0,
        interest_rate: 800,
        is_completed: false,
        is_withdrawn: false,
    };
    let plan_key = DataKey::SavingsPlan(user_addr.clone(), plan_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&plan_key, &lock_plan);
    });
    
    // Set ledger timestamp to before maturity
    env.ledger().with_mut(|li| {
        li.timestamp = maturity_time - 100;
    });
    
    // This should panic with LockNotMatured error (error code 4)
    client.withdraw_lock_save(&user_addr, &plan_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_withdraw_lock_save_already_withdrawn() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user data
    let user = User {
        total_balance: 500_000,
        savings_count: 1,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Set up lock savings plan that's already been withdrawn
    let maturity_time = 1000000u64;
    let lock_plan = SavingsPlan {
        plan_id,
        plan_type: PlanType::Lock(maturity_time),
        balance: 0,
        start_time: 900000,
        last_deposit: 900000,
        last_withdraw: 1000100,
        interest_rate: 800,
        is_completed: true,
        is_withdrawn: true,
    };
    let plan_key = DataKey::SavingsPlan(user_addr.clone(), plan_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&plan_key, &lock_plan);
    });
    
    // Set ledger timestamp to after maturity
    env.ledger().with_mut(|li| {
        li.timestamp = maturity_time + 200;
    });
    
    // This should panic with AlreadyWithdrawn error (error code 5)
    client.withdraw_lock_save(&user_addr, &plan_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_withdraw_lock_save_user_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Don't set up any user data - user doesn't exist
    
    // This should panic with UserNotFound error (error code 1)
    client.withdraw_lock_save(&user_addr, &plan_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_withdraw_lock_save_plan_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user data
    let user = User {
        total_balance: 1_000_000,
        savings_count: 0,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Don't set up any plan - plan doesn't exist
    
    // This should panic with PlanNotFound error (error code 2)
    client.withdraw_lock_save(&user_addr, &plan_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_withdraw_lock_save_wrong_plan_type() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user data
    let user = User {
        total_balance: 1_000_000,
        savings_count: 1,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Set up a Flexi plan instead of Lock plan
    let flexi_plan = SavingsPlan {
        plan_id,
        plan_type: PlanType::Flexi,
        balance: 500_000,
        start_time: 1000000,
        last_deposit: 1000000,
        last_withdraw: 0,
        interest_rate: 500,
        is_completed: false,
        is_withdrawn: false,
    };
    let plan_key = DataKey::SavingsPlan(user_addr.clone(), plan_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&plan_key, &flexi_plan);
    });
    
    // This should panic with PlanNotFound error (error code 2)
    // because it's not a Lock plan
    client.withdraw_lock_save(&user_addr, &plan_id);
}

#[test]
fn test_withdraw_lock_save_balance_update() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(NesteraContract, ());
    let client = NesteraContractClient::new(&env, &contract_id);
    
    let user_addr = Address::generate(&env);
    let plan_id = 1u64;
    
    // Set up user with multiple savings
    let initial_balance = 2_000_000i128;
    let lock_balance = 750_000i128;
    let user = User {
        total_balance: initial_balance,
        savings_count: 2,
    };
    let user_key = DataKey::User(user_addr.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&user_key, &user);
    });
    
    // Set up lock savings plan
    let maturity_time = 1000000u64;
    let lock_plan = SavingsPlan {
        plan_id,
        plan_type: PlanType::Lock(maturity_time),
        balance: lock_balance,
        start_time: 900000,
        last_deposit: 900000,
        last_withdraw: 0,
        interest_rate: 800,
        is_completed: false,
        is_withdrawn: false,
    };
    let plan_key = DataKey::SavingsPlan(user_addr.clone(), plan_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&plan_key, &lock_plan);
    });
    
    // Set ledger timestamp to after maturity
    env.ledger().with_mut(|li| {
        li.timestamp = maturity_time + 100;
    });
    
    // Perform withdrawal
    let withdrawn = client.withdraw_lock_save(&user_addr, &plan_id);
    assert_eq!(withdrawn, lock_balance);
    
    // Verify user balance is correctly decremented
    env.as_contract(&contract_id, || {
        let updated_user: User = env.storage().persistent().get(&user_key).unwrap();
        assert_eq!(updated_user.total_balance, initial_balance - lock_balance);
        assert_eq!(updated_user.total_balance, 1_250_000);
    });
}

