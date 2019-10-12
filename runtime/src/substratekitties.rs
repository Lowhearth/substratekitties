use support::{decl_storage, decl_module, StorageValue, dispatch::Result, StorageMap};
use system::ensure_signed;


pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Myu32: u32;
        // MyBool: get(my_bool_better): bool;
        Value: map T::AccountId => u64;
        // Declare storage and getter functions here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

      fn set_value(origin, value: u64) -> Result {
        let _sender = ensure_signed(origin)?;
        
        <Value<T>>::insert(_sender, value);

        Ok(())
      }
        // Declare public functions here
    }
}