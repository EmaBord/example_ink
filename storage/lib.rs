#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod storage {
    use ink::storage::Mapping;
    use istorage::IStorage;

    #[ink(storage)]
    pub struct Storage {
        value: u128,
        projects: Mapping<AccountId, Project>,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct Project {
        name: ink::prelude::string::String,
        description: ink::prelude::string::String,
        owner: AccountId,
        status: u128,
        available_credits: u128,
        total_credits: u128,
    }
    
    impl Storage {
        /// El constructor inicializa el valor inicial del contrato en 0.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 , projects: ink::storage::Mapping::default()}
        }

        fn _retrieve(&self) -> u128 {
            self.value
        }

        fn _set(&mut self, value: u128) {
            self.value = value;
        }
    }
    impl IStorage for Storage{
        /// Cambia el valor almacenado en el contrato a `value`.
        #[ink(message)]
        fn set(&mut self, value: u128) {
            self._set(value)
        }

        /// Retorna el valor actual del contrato.
        #[ink(message)]
        fn retrieve(&self) -> u128 {
            self._retrieve()
        }
        
    }


    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn test_new() {
            let storage = Storage::new();
            assert_eq!(storage.retrieve(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn test_set() {
            let mut storage = Storage::new();
            assert_eq!(storage.retrieve(), 0);
            storage.set(10);
            assert_eq!(storage.retrieve(), 10);
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

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn test_new_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor_storage = StorageRef::new();
            let account = ink_e2e::alice();
            let name = "storage";
            let value = 0;
            let storage_limit_deposit = None;

            // When
            let contract_storage_address= client
                .instantiate(name, &account, constructor_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            // construimos el mensaje
            let message_retrieve = build_message::<StorageRef>(contract_storage_address.clone())
                .call(|storage| storage.retrieve());

            // ejecutamos el mensaje o la llamada
            let retrieve_result = client.call_dry_run(&account, &message_retrieve, value, storage_limit_deposit).await;
            assert!(matches!(retrieve_result.return_value(), 0));

            Ok(())
        }
        
        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn test_set_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor_storage = StorageRef::new();
            let account = ink_e2e::alice();
            let name = "storage";
            let value = 0;
            let storage_limit_deposit = None;

            // When
            let contract_storage_address= client
                .instantiate(name, &account, constructor_storage, value, storage_limit_deposit)
                .await
                .expect("instantiate failed")
                .account_id;

            // When
            let message_set = build_message::<StorageRef>(contract_storage_address.clone())
                .call(|storage| storage.set(10));
            let _flip_result = client
                .call(&account, message_set, value, storage_limit_deposit)
                .await
                .expect("failed");

            // Then
            // construimos el mensaje
            let message_retrieve = build_message::<StorageRef>(contract_storage_address.clone())
                .call(|storage| storage.retrieve());

            // ejecutamos el mensaje o la llamada
            let retrieve_result = client.call_dry_run(&account, &message_retrieve, value, storage_limit_deposit).await;
            assert!(matches!(retrieve_result.return_value(), 10));

            Ok(())
        }
    }
    
}
