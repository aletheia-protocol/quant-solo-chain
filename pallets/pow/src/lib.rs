#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, dispatch::DispatchResult};
	use frame_system::pallet_prelude::*;
	use sp_core::H256;
	use sp_io::hashing::blake2_256;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	//TODO all this should be generated with benchmarks

	pub trait WeightInfo {
		fn submit_proof() -> Weight;
	}

	pub struct DefaultWeightInfo;

	impl WeightInfo for DefaultWeightInfo {
		fn submit_proof() -> Weight {
			Weight::from_parts(10_000, 0)
		}
	}


	#[pallet::storage]
	#[pallet::getter(fn latest_nonce)]
	pub type LatestNonce<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn last_successful_hash)]
	pub type LastSuccessfulHash<T> = StorageValue<_, H256, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProofSubmitted {
			who: T::AccountId,
			nonce: u64,
			hash: H256,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidProof,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::submit_proof())]
		pub fn submit_proof(origin: OriginFor<T>, nonce: u64, proof: H256) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Generate the hash for the submitted nonce and account
			let data = (who.clone(), nonce, proof);
			let generated_hash = blake2_256(&codec::Encode::encode(&data));

			// Define a simple target for the PoW
			let target: H256 = H256::repeat_byte(0xF0);

			// Check if the generated hash meets the target
			if H256::from(generated_hash) <= target {

				// Store the nonce and hash
				LatestNonce::<T>::put(nonce);
				LastSuccessfulHash::<T>::put(H256::from(proof));

				// Emit an event
				Self::deposit_event(Event::ProofSubmitted {
					who,
					nonce,
					hash: H256::from(generated_hash),
				});

				Ok(())
			} else {
				Err(Error::<T>::InvalidProof.into())
			}
		}
	}
}