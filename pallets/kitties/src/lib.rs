#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
    decl_module, decl_storage, decl_event, StorageValue, StorageDoubleMap,
    traits::Randomness, RuntimeDebug
};
use frame_system;
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty<T> {
    dna: [u8; 16],
    owner: T,
}
// pub struct Kitty([u8; 16]);

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		KittyCreated(AccountId, u32, Kitty<AccountId>),
	}
);

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        // pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Option<Kitty<T::AccountId>>;
        pub NextKittyId get(fn next_kitty_id): u32;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create_kitty(origin) {
            let sender = ensure_signed(origin)?;

            // generate a random 128bit value
            let payload = (
                <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
                &sender,
                <frame_system::Module<T>>::extrinsic_index(),
            );

            let dna = payload.using_encoded(blake2_128);


            // create kitty
            let kitty = Kitty {
                dna,
                owner: sender.clone()
            };
            let kitty_id = Self::next_kitty_id();

            <Kitties<T>>::insert(kitty_id, kitty.clone());

            NextKittyId::put(kitty_id + 1);

            Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty));
        }
    }
}
