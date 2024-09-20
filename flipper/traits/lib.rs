#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::trait_definition]
pub trait FlipperTrait {
    /// Flips the current value of the implementer's boolean.
    #[ink(message)]
    fn flip(&mut self);
    /// Returns the current value of the implementer's boolean.
    #[ink(message)]
    fn get(&self) -> bool;
    /// Returns the number of times the implementer's boolean has been flipped.
    #[ink(message)]
    fn get_count(&self) -> u64;
}

#[ink::trait_definition]
pub trait IncrementerTrait {
    /// Increments the current value of the implementer's integer by one.
    #[ink(message)]
    fn inc(&mut self);
    /// Returns the current value of the implementer's integer.
    #[ink(message)]
    fn get(&self) -> u64;
}
