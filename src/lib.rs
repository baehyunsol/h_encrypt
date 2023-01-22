mod table;
mod hash;

#[cfg(test)]
mod tests;

use table::utils::*;
use table::Table;
use table::RANDOM_TABLES;

fn gen_tables(bytes: &[u8], password: &[u8]) -> Vec<Vec<Table>> {
    // from `hash(bytes)` and `hash(password)`
    let hashes: Vec<usize> = todo!();

    let mut seeds = RANDOM_TABLES.to_vec();

    for i in hashes.into_iter() {
        let new_tables = apply_higher_order(&RANDOM_TABLES, &RANDOM_TABLES[i]);
        seeds = compose_tables(&seeds, &new_tables);
    }

    vec![
        seeds[0..83].to_vec(),
        seeds[83..168].to_vec(),
        seeds[168..=255].to_vec()
    ]
}

pub fn encrypt_bytes(bytes: &[u8], password: &[u8]) -> Vec<u8> {
    let tables = gen_tables(bytes, password);

    bytes.iter().enumerate().map(
        |(i, c)|
        table::apply(
            &tables[2][i % tables[2].len()],
            table::apply(
                &tables[1][i % tables[1].len()],
                table::apply(&tables[0][i % tables[0].len()], *c)
            )
        )
    ).collect()
}

pub fn decrypt_bytes(bytes: &[u8], password: &[u8]) -> Vec<u8> {
    todo!()
}