#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::trait_definition]
pub trait FlipperTrait {
    /// Flips the current value of the implementer's boolean.
    #[ink(message)]
    fn flip(&mut self);
    /// Returns the current value of the implementer's boolean.
    #[ink(message)]
    fn get(&self) -> bool;
}