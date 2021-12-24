use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_create_name() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(NameModule::create_name(Origin::signed(1), b"test".to_vec()));
        // Read pallet storage and assert an expected result.
        assert_eq!(NameModule::name_owner(b"test".to_vec()), Some(1));
    });
}

#[test]
fn correct_error_for_too_long() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            NameModule::create_name(Origin::signed(1), b"too_looooooooong_test_name".to_vec()),
            Error::<Test>::TooLong
        );
    });
}

#[test]
fn correct_error_for_name_is_taken() {
    new_test_ext().execute_with(|| {
        assert_ok!(NameModule::create_name(Origin::signed(1), b"test".to_vec()));

        assert_noop!(
            NameModule::create_name(Origin::signed(2), b"test".to_vec()),
            Error::<Test>::NameIsTaken
        );
    });
}

#[test]
fn it_works_set_file_name() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(NameModule::create_name(
            Origin::signed(1),
            b"monero".to_vec()
        ));

        assert_ok!(NameModule::set_file_name(
            Origin::signed(1),
            b"monero".to_vec(),
            b"QmcWpgfueHJiPzKSodUSPdvt2v9X7t5s8KdvfC3y6gm1Zc".to_vec()
        ));

        // Read pallet storage and assert an expected result.
        assert_eq!(
            NameModule::file_name(b"monero".to_vec()),
            Some(b"QmcWpgfueHJiPzKSodUSPdvt2v9X7t5s8KdvfC3y6gm1Zc".to_vec())
        );
    });
}

#[test]
fn correct_error_for_set_file_name() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            NameModule::set_file_name(
                Origin::signed(1),
                b"monero".to_vec(),
                b"QmcWpgfueHJiPzKSodUSPdvt2v9X7t5s8KdvfC3y6gm1Zc".to_vec()
            ),
            Error::<Test>::NameNotFound
        );

        assert_ok!(NameModule::create_name(
            Origin::signed(1),
            b"monero".to_vec()
        ));

        assert_noop!(
            NameModule::set_file_name(
                Origin::signed(2),
                b"monero".to_vec(),
                b"QmcWpgfueHJiPzKSodUSPdvt2v9X7t5s8KdvfC3y6gm1Zc".to_vec()
            ),
            Error::<Test>::SignerIsNotTheOwner
        );
    });
}