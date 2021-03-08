#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    // use ink_env::AccountId;
    use ink_storage::collections::HashMap as StorageHashMap;
    // use ink_env::AccountId;

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        allowance:StorageHashMap<(AccountId,AccountId),Balance>,
    }
    #[ink(event)]
    pub struct Transfer
    {
        #[ink(topic)]
        from:Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value:Balance,
    }
    #[ink(event)]
    pub struct Approved
    {
        #[ink(topic)]
        onwer:Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        value:Balance,
    }
    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error{
        InsufficientBalance,
        InsufficientAllowance,
    }
    pub type Result<T> = core::result::Result<T, Error>;
    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller=Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, initial_supply);
            Self::env()
                .emit_event(
                    Transfer{
                        from:None,
                        to:Some(caller),
                        value:initial_supply,
                    }
                );

            Self {
                total_supply: initial_supply,
                balances,
                allowance:StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_from_to(self.env().caller(), to, value)
        }

        #[ink(message)]
        pub fn alloc_allowance(&mut self,owner:AccountId,spender:AccountId,value:Balance)->Result<()> {
            //1.find owner's balance，
            //2.judge value >balance error:InfluenceBalance,ignor InfluenceAllowance
            //3.insert into allowance
            //4.send event
            //5.return ok()
            let owner_balance=self.balance_of_or_zero(&owner);
            if owner_balance<value{
                return Err(Error::InsufficientBalance)
            }
            self.allowance.insert((owner,spender),value);
            self.env().emit_event(
                Approved{
                    onwer:Some(owner),
                    spender:Some(spender),
                    value:value,
                }
            );
            Ok(())
        }
        #[ink(message)]
        pub fn allowance_of(&self,owner:AccountId,spender:AccountId)->Balance{
           self.allowance.get(&(owner,spender)).copied().unwrap_or(0)
        }
        #[ink(message)]
        pub fn transfer_from_spend_to(&mut self,from:AccountId,spender:AccountId,to:AccountId,value:Balance) -> Result<()>{
            let from_allowance=self.allowance.get(&(from,spender)).copied().unwrap_or(0);
            if from_allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(from,to,value);
            self.allowance.insert((from,spender),from_allowance-value);
            Ok(())
        }

            #[ink(message)]
        pub fn transfer_from(&mut self,from:AccountId,to:AccountId,value:Balance) -> Result<()>{
            //1.find from,to
            // let owner=self.env().caller();
            let from_allowance=self.allowance.get(&(from,to)).copied().unwrap_or(0);
            if from_allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(from,to,value);
            self.allowance.insert((from,to),from_allowance-value);
            Ok(())
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }

            // Update the sender's balance.
            self.balances.insert(from, from_balance - value);

            // Update the receiver's balance.
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(
                Transfer{
                    from:Some(from),
                    to:Some(to),
                    value:value,
                }
            );

            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).copied().unwrap_or(0)
                // *self.balances.get(owner).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            println!("{}",contract.total_supply());
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn allowance_works() {
            let mut contract = Erc20::new(100);
            let owner=AccountId::from([0x1; 32]);
            let spender=AccountId::from([0x2; 32]);
            let to=AccountId::from([0x3; 32]);
            //1.onwer->to:1,owner:99,to :1
            //2.ownre alloc spender:11,owner,awan-spender:11,spender 余额0
            //3.owner 转spend而配置：1，owner-spender：10，spender余额1
            //4.owner spender to :2,owner-spend:8,spend余额1，to：余额9

            // assert_eq!(owner, Self::env().caller());

            assert_eq!(contract.balance_of(owner), 100);
            assert_eq!(contract.balance_of(spender), 0);
            assert_eq!(contract.balance_of(to), 0);

            //1.onwer->to:1,owner:99,to :1
            contract.transfer(to,1);
            assert_eq!(contract.balance_of(owner), 99);
            assert_eq!(contract.balance_of(spender), 0);
            assert_eq!(contract.balance_of(to), 1);

            //2.ownre alloc spender:11,owner,awan-spender:11,spender 余额0
            contract.alloc_allowance(owner,spender,11);
            assert_eq!(contract.balance_of(owner), 99);
            assert_eq!(contract.balance_of(spender), 0);
            assert_eq!(contract.balance_of(to), 1);
            assert_eq!(contract.allowance_of(owner,spender),11);

            //3.owner 转spend而配置：1，owner-spender：10，spender余额1
            contract.transfer_from(owner,spender,1);
            assert_eq!(contract.balance_of(owner), 98);//this 98 wrong
            assert_eq!(contract.balance_of(spender), 1);//this 1 wrong
            assert_eq!(contract.balance_of(to), 1);
            assert_eq!(contract.allowance_of(owner,spender),10);//this 10 wrong

            //4.owner spender to :2,owner-spend:8,spend余额1，to：余额9
            contract.transfer_from_spend_to(owner,spender,to,1);
            assert_eq!(contract.balance_of(owner), 97);//this 98 wrong
            assert_eq!(contract.balance_of(spender), 1);//this 1 wrong
            assert_eq!(contract.balance_of(to), 2);
            assert_eq!(contract.allowance_of(owner,spender),9);//this 10 wrong
        }
        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }

    }
}
