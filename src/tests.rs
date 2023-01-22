use super::*;
use super::table::*;

#[test]
fn enc_dec_id() {

    for bytes in many_tables().iter() {

        for password in many_tables().iter() {
            assert_eq!(decrypt_bytes(&encrypt_bytes(&bytes.0, &password.0), &password.0), bytes.0);
        }

    }

}

#[test]
fn output_random() {

    for password in many_tables().iter() {
        assert!(is_random(&encrypt_bytes(&[0;256], &password.0)));
        assert!(is_random(&encrypt_bytes(&[1;256], &password.0)));
        assert!(is_random(&encrypt_bytes(&[2;256], &password.0)));
        assert!(is_random(&encrypt_bytes(&(0..=255).collect::<Vec<u8>>(), &password.0)));
    }

}