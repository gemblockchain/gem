#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type Name = Vec<u8>;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// The maximum length a name may be.
    type MaxLength: Get<usize>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Config> as NameModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        /// Total number of names
        TotalNames get(fn total_names): u128 = 0;
        /// Store for the owner of the name.
        /// Name => Owner
        NameOnwer get(fn name_owner): map hasher(twox_64_concat) Vec<u8> => Option<T::AccountId>;
        /// Store for the data of the name.
        /// Name => Data
        DataName get(fn data_name): map hasher(twox_64_concat) Vec<u8> => Vec<u8>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Create name.
        NameCreate(Name, AccountId),
        /// Changing owner for name.
        ChangingOwner(Name, AccountId),
        /// Set file for name.
        SetData(Name),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config> {
        /// A name is too long.
        TooLong,
        /// This name has already been created.
        NameIsTaken,
        /// Name not found.
        NameNotFound,
        /// Signer is not the owner.
        SignerIsNotTheOwner,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        // The maximum length a name may be.
        const MaxLength: u32 = T::MaxLength::get() as u32;

        #[weight = 10_000]
        pub fn create_name(origin, name: Vec<u8>) -> dispatch::DispatchResult {
            ensure!(name.len() <= T::MaxLength::get(), Error::<T>::TooLong);
            ensure!(<NameOnwer<T>>::get(&name).is_none(), Error::<T>::NameIsTaken);

            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let signer = ensure_signed(origin)?;

            // Update storage.
            <NameOnwer<T>>::insert(&name, &signer);
            <TotalNames>::mutate(|total| *total += 1);

            // Emit an event.
            Self::deposit_event(RawEvent::NameCreate(name, signer));
            // Return a successful DispatchResult
            Ok(())
        }

        #[weight = 10_000]
        pub fn change_owner(origin, name: Vec<u8>, new_owner: T::AccountId) -> dispatch::DispatchResult {
            ensure!(<NameOnwer<T>>::get(&name).is_some(), Error::<T>::NameNotFound);

            let signer = ensure_signed(origin)?;
            let owner = <NameOnwer<T>>::get(&name).unwrap();

            ensure!(signer == owner, Error::<T>::SignerIsNotTheOwner);

            // Update storage.
            <NameOnwer<T>>::mutate(&name, |owner| *owner = Some(new_owner));

            // Emit an event.
            Self::deposit_event(RawEvent::ChangingOwner(name, signer));
            // Return a successful DispatchResult
            Ok(())
        }

        #[weight = 10_000]
        pub fn set_data(origin, name: Vec<u8>, new_data: Vec<u8>) -> dispatch::DispatchResult {
            ensure!(<NameOnwer<T>>::get(&name).is_some(), Error::<T>::NameNotFound);

            let signer = ensure_signed(origin)?;
            let owner = <NameOnwer<T>>::get(&name).unwrap();

            ensure!(signer == owner, Error::<T>::SignerIsNotTheOwner);

            // Update storage.
            <DataName>::mutate(&name, |data| *data = new_data);

            // Emit an event.
            Self::deposit_event(RawEvent::SetData(name));
            // Return a successful DispatchResult
            Ok(())
        }
    }
}
