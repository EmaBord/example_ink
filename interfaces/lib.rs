#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::trait_definition]
pub trait IStorage{

    #[ink(message)]
    fn set(&mut self, value: u128);

    #[ink(message)]
    fn retrieve(&self) -> u128;

}