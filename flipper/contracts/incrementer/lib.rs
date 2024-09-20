#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {
    use dyn_traits::IncrementerTrait;

    #[ink(storage)]
    pub struct Incrementer {
        value: u64,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: u64) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }
    }

    impl IncrementerTrait for Incrementer {
        #[ink(message)]
        fn inc(&mut self) {
            self.value = self.value.checked_add(1).unwrap();
        }

        #[ink(message)]
        fn get(&self) -> u64 {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::new_default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
    }
}
