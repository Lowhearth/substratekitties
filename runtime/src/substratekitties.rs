use support::{decl_storage, decl_module, StorageValue, dispatch::Result, StorageMap};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{As, Hash};


pub trait Trait: balances::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64
}


decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {        
        Kitties: map T::AccountId => Kitty<T::Hash, T::Balance>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

      fn create_cat(origin) -> Result {
        let _sender = ensure_signed(origin)?;
        let has_of_zero = <T as system::Trait>::Hashing::hash_of(&0);
        let zero_balance = <T::Balance as As<u64>>::sa(0);        
        let cat = Kitty {
          id: has_of_zero,
          dna: has_of_zero,
          price: zero_balance,
          gen: 0,
        };

        <Kitties<T>>::insert(_sender, cat);

        Ok(())
      }
        // Declare public functions here
    }
}