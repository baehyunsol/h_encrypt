mod hash;


use hash::{hash1, hash2, hash_string1, hash_string2};


// u64_to_bytes(459524) =u64_to_bytes(4 * 256^0 + 3 * 256^1 + 7 * 256^2) -> [4, 3, 7, 0, 0, 0, 0, 0]
fn u64_to_bytes(mut n: u64) -> Vec<u8> {

    let mut result = vec![];

    for _ in 0..8 {
        result.push((n % 256) as u8);
        n /= 256;
    }

    result
}


fn gen_key(password: &Vec<u8>) -> Vec<u8> {

    let mut key1 = hash_string1(password);
    let mut key2 = hash_string2(password);
    let mut tmp_key;

    let key_length = (key1 % 128 + key2 % 128 + 128) as usize;

    let mut result = Vec::with_capacity(key_length);
    result.push(u64_to_bytes(key1));
    result.push(u64_to_bytes(key2));

    for _ in 0..key_length - 2 {
        // key1, key2 = key2, hash1(key1 ^ key2);
        tmp_key = key1;
        key1 = key2;
        key2 = hash1(tmp_key ^ key1);
        result.push(u64_to_bytes(key2));
    }

    result.concat()
}


pub fn encrypt(message: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {

    let mut cipher_text = encrypt_unit(message, password);

    for _ in 0..4 {
        cipher_text = encrypt_unit(&cipher_text, password);
    }

    cipher_text
}


fn encrypt_unit(message: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {

    // modifies the password so that it generates a different key everytime
    let mut password_suffix = {
        let message_prefix_length = if message.len() > 64 {64} else {message.len()};

        u64_to_bytes(hash_string1(&message[..message_prefix_length].to_vec()))
    };

    let password_hash = u64_to_bytes(hash2(hash_string1(password)));

    let mut password = password.clone();
    
    password.extend(&password_suffix);

    let key = gen_key(&password);

    let cipher_text = message.iter().enumerate().map(
        |ic| // (index, character)
        key[ic.0 % key.len()] ^ ic.1    
    ).collect::<Vec<u8>>();

    // password_suffix must be written on the cipher_text
    // but it's not good idea to write it down without any modification
    for i in 0..8 {
        password_suffix[i] ^= password_hash[i];
    }

    password_suffix.extend(&cipher_text);

    password_suffix
}


pub fn decrypt(message: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {

    let mut plain_text = decrypt_unit(message, password);

    for _ in 0..4 {
        plain_text = decrypt_unit(&plain_text, password);
    }

    plain_text
}


fn decrypt_unit(message: &Vec<u8>, password: &Vec<u8>) -> Vec<u8> {

    let cipher_text = message[8..].to_vec();
    let mut password_suffix = message[..8].to_vec();
    let password_hash = u64_to_bytes(hash2(hash_string1(password)));

    for i in 0..8 {
        password_suffix[i] ^= password_hash[i];
    }

    let mut password = password.clone();
    password.extend(&password_suffix);

    let key = gen_key(&password);

    let plain_text = cipher_text.iter().enumerate().map(
        |ic| // (index, character)
        key[ic.0 % key.len()] ^ ic.1
    ).collect::<Vec<u8>>();

    plain_text
}