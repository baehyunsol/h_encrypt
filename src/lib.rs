mod table;
mod hash;
mod utils;
mod err;

#[cfg(test)]
mod tests;

use hash::{hash_string, hash_stream};
use table::{Table, apply, inverse_table};
use utils::{gen_tables, get_suffix_and_prefix_length};
pub use err::EncError;

pub fn encrypt_bytes(bytes: &[u8], password: &[u8]) -> Vec<u8> {
    let mut bytes_hash = hash_string(bytes);

    while bytes_hash.len() < 64 {
        bytes_hash.push(rand::random::<u8>() as usize);
    }

    let tables = gen_tables(password, &bytes_hash);

    let (prefix_length, suffix_length) = get_suffix_and_prefix_length(&tables);
    let mut result: Vec<u8> = Vec::with_capacity(
        bytes.len() + prefix_length + 64 + suffix_length  // data + prefix + bytes_hash + suffix
    );

    for _ in 0..prefix_length {
        result.push(rand::random::<u8>());
    }

    for (i, c) in bytes.iter().enumerate() {
        result.push(
            apply(
                &tables[2][i % tables[2].len()],
                apply(
                    &tables[1][i % tables[1].len()],
                    apply(&tables[0][i % tables[0].len()], *c)
                )
            )
        );
    }

    for _ in 0..suffix_length {
        result.push(rand::random::<u8>());
    }

    for b in bytes_hash.into_iter() {
        result.push(b as u8);
    }

    result
}

pub fn decrypt_bytes(bytes: &[u8], password: &[u8]) -> Result<Vec<u8>, EncError> {

    if bytes.len() < 64 {
        return Err(EncError::DecryptionTooShort);
    }

    let bytes_hash: Vec<usize> = bytes[(bytes.len() - 64)..bytes.len()].iter().map(|n| *n as usize).collect();
    let mut tables = gen_tables(password, &bytes_hash);
    let (prefix_length, suffix_length) = get_suffix_and_prefix_length(&tables);

    tables = tables.iter().map(
        |tables| tables.iter().map(
            |table| inverse_table(table)
        ).collect::<Vec<Table>>()
    ).collect();

    if bytes.len() < 64 + suffix_length {
        return Err(EncError::DecryptionTooShort);
    }

    Ok(bytes[prefix_length..(bytes.len() - 64 - suffix_length)].iter().enumerate().map(
        |(i, c)|
        apply(
            &tables[0][i % tables[0].len()],
            apply(
                &tables[1][i % tables[1].len()],
                apply(&tables[2][i % tables[2].len()], *c)
            )
        )
    ).collect())
}

use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;

/// read raw data from `read_from`, encrypt it, and write the encrypted data to `write_to`
pub fn encrypt_stream<P: AsRef<Path>, Q: AsRef<Path>>(read_from: P, write_to: Q, password: &[u8], block_size: usize) -> Result<(), EncError> {
    let mut bytes_hash = hash_stream(&read_from, block_size)?;

    while bytes_hash.len() < 64 {
        bytes_hash.push(rand::random::<u8>() as usize);
    }

    let tables = gen_tables(password, &bytes_hash);

    let (prefix_length, suffix_length) = get_suffix_and_prefix_length(&tables);
    let mut read_buf = vec![0; block_size];
    let mut write_buf = Vec::with_capacity(block_size + prefix_length);

    for _ in 0..prefix_length {
        write_buf.push(rand::random::<u8>());
    }

    let mut file_read = File::open(read_from)?;
    let mut file_write = File::options().create(true).truncate(true).write(true).open(write_to)?;
    let mut index = 0;

    loop {
        let count = file_read.read(&mut read_buf)?;

        for c in read_buf[0..count].iter() {
            write_buf.push(
                apply(
                    &tables[2][index % tables[2].len()],
                    apply(
                        &tables[1][index % tables[1].len()],
                        apply(&tables[0][index % tables[0].len()], *c)
                    )
                )
            );
            index += 1;
        }

        file_write.write_all(&write_buf)?;
        write_buf = Vec::with_capacity(block_size);

        if count == 0 {
            break;
        }

    }

    for _ in 0..suffix_length {
        write_buf.push(rand::random::<u8>());
    }

    for b in bytes_hash.into_iter() {
        write_buf.push(b as u8);
    }

    file_write.write_all(&write_buf)?;

    Ok(())
}

/// read encrypted data from `read_from`, decrypt it, and write the decrypted data to `write_to`
pub fn decrypt_stream<P: AsRef<Path>, Q: AsRef<Path>>(read_from: P, write_to: Q, password: &[u8], block_size: usize) -> Result<(), EncError> {
    let mut file_read = File::open(&read_from)?;
    let mut file_write = File::options().create(true).truncate(true).write(true).open(write_to)?;
    let file_len = file_read.metadata()?.len();
    let mut bytes_hash = vec![0; 64];

    if file_len < 64 {
        return Err(EncError::DecryptionTooShort);
    }

    file_read.seek(SeekFrom::Start(file_len - 64))?;
    file_read.read(&mut bytes_hash)?;
    let bytes_hash = bytes_hash.into_iter().map(|n| n as usize).collect();

    let mut tables = gen_tables(password, &bytes_hash);
    let (prefix_length, suffix_length) = get_suffix_and_prefix_length(&tables);

    tables = tables.iter().map(
        |tables| tables.iter().map(
            |table| inverse_table(table)
        ).collect::<Vec<Table>>()
    ).collect();

    file_read.seek(SeekFrom::Start(prefix_length as u64))?;
    let mut index = 0;

    if (file_len as usize) < 64 + suffix_length + prefix_length {
        return Err(EncError::DecryptionTooShort);
    }

    let file_end_index = file_len - 64 - suffix_length as u64 - prefix_length as u64;

    let mut read_buf = vec![0; block_size];
    let mut write_buf = Vec::with_capacity(block_size);

    if file_end_index == 0 {
        return Ok(());
    }

    'outer: loop {
        let count = file_read.read(&mut read_buf)?;

        for c in read_buf[0..count].iter() {
            write_buf.push(
                apply(
                    &tables[0][index % tables[0].len()],
                    apply(
                        &tables[1][index % tables[1].len()],
                        apply(&tables[2][index % tables[2].len()], *c)
                    )
                )
            );
            index += 1;

            if index == file_end_index as usize {
                file_write.write_all(&write_buf)?;
                break 'outer;
            }

        }

        file_write.write_all(&write_buf)?;
        write_buf = Vec::with_capacity(block_size);
    }

    Ok(())
}