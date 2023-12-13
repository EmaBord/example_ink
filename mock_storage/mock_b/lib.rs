#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod mock_b {
    use interfaces::IStorage;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MockStorageB {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl MockStorageB {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: false }
        }


        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    impl IStorage for MockStorageB{

        /// Cambia el valor almacenado en el contrato a `value`.
        #[ink(message)]
        fn set(&mut self, value: u128) {
            
        }

        /// Retorna el valor actual del contrato.
        #[ink(message)]
        fn retrieve(&self) -> u128 {
            0
        }
        
    }
 
}

