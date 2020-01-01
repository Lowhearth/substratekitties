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
        Kitties: map T::Hash => Kitty<T::Hash, T::Balance>;
        //Kitties: map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner: map T::Hash => T::AccountId;
        OwnerKitty: map T::AccountId => T::Hash;
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

      fn create_cat(origin) -> Result {
        let _sender = ensure_signed(origin)?;
        let nonce = <Nonce<T>>::get();
        let random_seed = <system::Module<T>>::random_seed();
        let random_hash = (random_seed, &_sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);
        <Nonce<T>>::mutate(|n| *n += 1);
        let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);
        let zero_balance = <T::Balance as As<u64>>::sa(0);        
        
        let cat = Kitty {
          id: random_hash,
          dna: random_hash,
          price: zero_balance,
          gen: 0,
        };
        <KittyOwner<T>>::insert(&random_hash, &_sender );
        <Kitties<T>>::insert(&random_hash, cat);
        <OwnerKitty<T>>::insert(&_sender, &random_hash);

        Ok(())
      }
        // Declare public functions here
    }
}