# Kitty pallet design

- Calls
    - fn deposit_event
    - pub fn create_kitty(origin)

- Storages
    - pub Kitties get(fn get_kitty_by_id): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) kitty_id => Option<Kitty>
    - NextKittyId: u32

- Types
    - struct Kitty(pub [u8; 16])

- Events
    - KittyCreated
        - owner: AccountId
        - kitty_id: u32
        - kitty: Kitty

