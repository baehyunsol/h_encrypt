use crate::{encrypt_bytes, decrypt_bytes, encrypt_stream, decrypt_stream};
use crate::table::{many_tables, is_random};
use std::fs::{File, self};
use std::io::{Read, Write};

pub fn test_files() -> Vec<String> {
    vec![
        "./test_data/lorem.txt".to_string(),
        "./test_data/lorem2.txt".to_string(),
        "./test_data/empty.txt".to_string(),
        "./test_data/hangul.txt".to_string(),
    ]
}

pub fn encrypted_files() -> Vec<String> {
    vec![
        "./test_data/lorem.txt.e".to_string(),
        "./test_data/lorem2.txt.e".to_string(),
        "./test_data/empty.txt.e".to_string(),
        "./test_data/hangul.txt.e".to_string(),
    ]
}

pub fn passwords() -> Vec<Vec<u8>> {
    vec![
        vec!['p' as u8, 'a' as u8, 's' as u8, 's' as u8, 'w' as u8, 'o' as u8, 'r' as u8, 'd' as u8],
        vec![],
        vec![0],
        many_tables()[0].0.clone()
    ]
}

#[test]
fn enc_dec_id() {

    for bytes in many_tables().iter() {

        for password in many_tables().iter() {
            assert_eq!(decrypt_bytes(&encrypt_bytes(&bytes.0, &password.0), &password.0).unwrap(), bytes.0);
        }

    }

}

#[test]
fn stream_bytes_id() {
    let test_files = test_files();
    let encrypted_files = encrypted_files();
    let passwords = passwords();

    for i in 0..test_files.len() {
        let mut f = File::open(&test_files[i]).unwrap();
        let mut original_data = vec![0; 1048576];
        let count = f.read(&mut original_data).unwrap();
        original_data = original_data[0..count].to_vec();

        for password in passwords.iter() {
            let encrypted = encrypt_bytes(&original_data, password);
            let mut save_enc = File::create("./tmp.e").unwrap();
            save_enc.write_all(&encrypted).unwrap();

            let mut decrypted = decrypt_bytes(&encrypted, password).unwrap();
            assert_eq!(decrypted, original_data);

            encrypt_stream(&test_files[i], &encrypted_files[i], password, 32).unwrap();  // I'm intentionally giving a small `block_size`
            let mut read_enc = File::open(&encrypted_files[i]).unwrap();
            decrypted = vec![0; 1048576];
            let count = read_enc.read(&mut decrypted).unwrap();
            decrypted = decrypted[0..count].to_vec();
            decrypted = decrypt_bytes(&decrypted, password).unwrap();
            assert_eq!(decrypted, original_data);

            decrypt_stream(&encrypted_files[i], "./tmp2.e", password, 32).unwrap();
            let mut read_dec = File::open("./tmp2.e").unwrap();
            decrypted = vec![0; 1048576];
            let count = read_dec.read(&mut decrypted).unwrap();
            decrypted = decrypted[0..count].to_vec();
            assert_eq!(decrypted, original_data);

            decrypt_stream("./tmp.e", "./tmp2.e", password, 32).unwrap();
            let mut read_dec = File::open("./tmp2.e").unwrap();
            decrypted = vec![0; 1048576];
            let count = read_dec.read(&mut decrypted).unwrap();
            decrypted = decrypted[0..count].to_vec();
            assert_eq!(decrypted, original_data);
        }

    }

    fs::remove_file("./tmp.e").unwrap();
    fs::remove_file("./tmp2.e").unwrap();
}

#[test]
fn output_random() {

    for password in many_tables().iter() {

        for i in 0..=255 {
            assert!(is_random(&encrypt_bytes(&[i;256], &password.0)));
        }

        assert!(is_random(&encrypt_bytes(&(0..=255).collect::<Vec<u8>>(), &password.0)));
    }

}