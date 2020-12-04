#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure};
use frame_system::ensure_signed;
use sp_std::vec::Vec;



/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as PoEModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfered(AccountId,Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
	     /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked.
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it.
        NotProofOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Get the block number from the FRAME System module.
            let current_block = <frame_system::Module<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
		}
		 /// Allow the owner to revoke their claim.
		 #[weight = 10_000]
		 fn revoke_claim(origin, proof: Vec<u8>) {
			 // Check that the extrinsic was signed and get the signer.
			 // This function will return an error if the extrinsic is not signed.
			 // https://substrate.dev/docs/en/knowledgebase/runtime/origin
			 let sender = ensure_signed(origin)?;
 
			 // Verify that the specified proof has been claimed.
			 ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
 
			 // Get owner of the claim.
			 let (owner, _) = Proofs::<T>::get(&proof);
 
			 // Verify that sender of the current call is the claim owner.
			 ensure!(sender == owner, Error::<T>::NotProofOwner);
 
			 // Remove claim from storage.
			 Proofs::<T>::remove(&proof);
 
			 // Emit an event that the claim was erased.
			 Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
		 }
		 #[weight = 10_000]
		 fn transfer_claim(origin,transfer_user:T::AccountId,proof:Vec<u8>){
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
			let (owner, _) = Proofs::<T>::get(&proof);
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();
			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (transfer_user, current_block));
			Self::deposit_event(RawEvent::ClaimTransfered(sender, proof));
		 }
	}
}