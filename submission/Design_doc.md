# Kitty pallet design

- Calls
    - fn deposit_event
    - pub fn create_kitty(origin)

- Storages
    - pub Kitties get(fn get_kitty_by_id): map hasher(blake2_128_concat) u32 => Option<Kitty<T>>
    - NextKittyId: u32

- Types
    - struct Kitty<T> { dna: [u8;  16], owner: T }

- Events
    - KittyCreated
        - owner: AccountId
        - kitty_id: u32
        - kitty: Kitty

