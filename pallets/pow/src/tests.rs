use crate::{mock::*, Error, Event};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;

#[test]
fn test_submit_valid_proof() {
    new_test_ext().execute_with(|| {
        // Set the block number to ensure events are recorded
        System::set_block_number(1);

        // Arrange input data
        let account_id: u64 = 1;
        let nonce: u64 = 42;
        let proof = H256::from([0x01; 32]); // A valid proof
        let data = (account_id, nonce, proof);
        let generated_hash = sp_io::hashing::blake2_256(&codec::Encode::encode(&data));

        // Ensure the generated hash meets the target
        let target = H256::repeat_byte(0xF0);
        assert!(H256::from(generated_hash) <= target, "Generated hash does not meet target");

        // Act: Call submit_proof
        assert_ok!(PoW::submit_proof(RuntimeOrigin::signed(account_id), nonce, proof));

        // Assert: Check that storage values are updated correctly
        assert_eq!(PoW::latest_nonce(), nonce);
        assert_eq!(PoW::last_successful_hash(), proof);

        // Assert: Verify that the event was emitted
        let events = System::events();
        assert!(events.iter().any(|record| matches!(
            record.event,
            RuntimeEvent::PoW(Event::ProofSubmitted { who, nonce: n, hash })
            if who == account_id && n == nonce && hash == H256::from(generated_hash)
        )));
    });
}

#[test]
fn test_submit_invalid_proof() {
    new_test_ext().execute_with(|| {
        // Set the block number to ensure events are recorded
        System::set_block_number(1);

        // Arrange input data
        let account_id: u64 = 1;
        let nonce: u64 = 100;
        let invalid_proof = H256::repeat_byte(0x01); // An invalid proof
        let data = (account_id, nonce, invalid_proof);
        let generated_hash = sp_io::hashing::blake2_256(&codec::Encode::encode(&data));


        // Ensure the generated hash does not meet the target
        let target = H256::repeat_byte(0xF0);

        assert!(H256::from(generated_hash) > target, "Generated hash meets target unexpectedly");

        // Act & Assert: Call submit_proof and expect an error
        assert_err!(
            PoW::submit_proof(RuntimeOrigin::signed(account_id), nonce, invalid_proof),
            Error::<Test>::InvalidProof
        );

        // Assert: Ensure that storage was not updated
        assert_eq!(PoW::latest_nonce(), 0);
        assert_eq!(PoW::last_successful_hash(), H256::default());
    });
}

#[test]
fn test_submit_proof_updates_storage_correctly() {
    new_test_ext().execute_with(|| {
        // Set the block number to ensure events are recorded
        System::set_block_number(1);

        // Arrange input data
        let account_id: u64 = 1;
        let nonce: u64 = 100;
        let proof = H256::from([0x02; 32]); // A valid proof
        let data = (account_id, nonce, proof);
        let generated_hash = sp_io::hashing::blake2_256(&codec::Encode::encode(&data));

        // Ensure the generated hash meets the target
        let target = H256::repeat_byte(0xF0);
        assert!(H256::from(generated_hash) <= target, "Generated hash does not meet target");

        // Act: Call submit_proof
        assert_ok!(PoW::submit_proof(RuntimeOrigin::signed(account_id), nonce, proof));

        // Assert: Check that storage values are updated correctly
        assert_eq!(PoW::latest_nonce(), nonce);
        assert_eq!(PoW::last_successful_hash(), proof);
    });
}