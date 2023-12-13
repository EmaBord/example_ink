#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod user {
    use interfaces::IStorage;
    #[ink(storage)]
    pub struct User {
        owner: AccountId,
        storage: ink::contract_ref!(IStorage),
    }

    impl User {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(storage: AccountId) -> Self {
            Self { owner:Self::env().caller() , storage: storage.into() }
        }


        #[ink(message)]
        pub fn set(&mut self, value: u128) {
            self.only_owner();
            self.storage.set(value);
        }

        
        #[ink(message)]
        pub fn retrieve(&self) -> u128 {
            self.only_owner();
            self.storage.retrieve()
        }

        #[ink(message)]
        pub fn set_storage(&mut self, storage: AccountId) {
            self.only_owner();
            self.storage = storage.into();
        }

        fn only_owner(&self){
            if self.env().caller() != self.owner {
                panic!("Only owner can call this function");
            }
        }
    }

    


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;
        use mock_a::mock_a::MockStorageARef;
        use mock_b::mock_b::MockStorageBRef;
        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn test_new_e2e_with_mock_a(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor_mock_storage = MockStorageARef::new();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_a";
            let storage_limit_deposit = None;
            let contract_address_mock_storage = client
                .instantiate(name, &account, constructor_mock_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            let constructor_user = UserRef::new(contract_address_mock_storage);
            let account = ink_e2e::bob();
            let name = "user";
            let contract_address_user = client
                .instantiate(name, &account, constructor_user, value, storage_limit_deposit).await
                .expect("error")
                .account_id;
            
    
            let message_retrieve = build_message::<UserRef>(contract_address_user.clone())
                .call(|user| user.retrieve());
            let retrieve_result = client.call_dry_run(&account, &message_retrieve, value, storage_limit_deposit).await;
            assert!(matches!(retrieve_result.return_value(), 11));

            Ok(())
        }

        #[ink_e2e::test]
        async fn test_new_e2e_with_mock_b(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor_mock_storage = MockStorageBRef::new();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_b";
            let storage_limit_deposit = None;
            let contract_address_mock_storage = client
                .instantiate(name, &account, constructor_mock_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            let constructor_user = UserRef::new(contract_address_mock_storage);
            let account = ink_e2e::bob();
            let name = "user";
            let contract_address_user = client
                .instantiate(name, &account, constructor_user, value, storage_limit_deposit).await
                .expect("error")
                .account_id;
            
    
            let message_retrieve = build_message::<UserRef>(contract_address_user.clone())
                .call(|user| user.retrieve());
            let retrieve_result = client.call_dry_run(&account, &message_retrieve, value, storage_limit_deposit).await;
            assert!(matches!(retrieve_result.return_value(), 0));

            Ok(())
        }
        
        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn test_set_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor_mock_storage = MockStorageARef::new();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_a";
            let storage_limit_deposit = None;
            let contract_address_mock_storage = client
                .instantiate(name, &account, constructor_mock_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            let constructor_user = UserRef::new(contract_address_mock_storage);
            let account = ink_e2e::bob();
            let name = "user";
            let contract_address_user = client
                .instantiate(name, &account, constructor_user, value, storage_limit_deposit).await
                .expect("error")
                .account_id;
            
    

            // When
            let message_set = build_message::<UserRef>(contract_address_user.clone())
                .call(|user| user.set(10));
            let _set_result = client
                .call(&account, message_set, value, storage_limit_deposit)
                .await
                .expect("failed");

            let message_retrieve = build_message::<UserRef>(contract_address_user.clone())
                .call(|user| user.retrieve());
            let retrieve_result = client.call_dry_run(&account, &message_retrieve, value, storage_limit_deposit).await;
            assert!(matches!(retrieve_result.return_value(), 11));

            Ok(())
        }
        #[ink_e2e::test]
        async fn test_set_e2e_not_owner(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor_mock_storage = MockStorageARef::new();
            let account = ink_e2e::alice();
            let value = 0;
            let name = "mock_a";
            let storage_limit_deposit = None;
            let contract_address_mock_storage = client
                .instantiate(name, &account, constructor_mock_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            let constructor_user = UserRef::new(contract_address_mock_storage);
            let account = ink_e2e::bob();
            let name = "user";
            let contract_address_user = client
                .instantiate(name, &account, constructor_user, value, storage_limit_deposit).await
                .expect("error")
                .account_id;
            
    

            // When
            let account = ink_e2e::charlie();
            let message_set = build_message::<UserRef>(contract_address_user.clone())
                .call(|user| user.set(10));
            let result = client
                .call(&account, message_set, value, storage_limit_deposit)
                .await;

            match result{
                Err(_) => (),
                Ok(_) => panic!("should not be able to call set")
            }
            Ok(())
        }

    }
    
}
