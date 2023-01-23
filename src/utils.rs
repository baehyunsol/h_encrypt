use crate::table::{Table, RANDOM_TABLES, utils::{apply_higher_order, compose_tables}};
use crate::hash::hash_string;

pub fn gen_tables(password: &[u8], bytes_hash: &Vec<usize>) -> Vec<Vec<Table>> {
    let hashes: Vec<usize> = hash_string(password);
    let mut seeds = RANDOM_TABLES.to_vec();

    for i in hashes.into_iter() {
        let new_tables = apply_higher_order(&RANDOM_TABLES, &RANDOM_TABLES[i]);
        seeds = compose_tables(&seeds, &new_tables);
    }

    for i in bytes_hash.iter() {
        let new_tables = apply_higher_order(&RANDOM_TABLES, &RANDOM_TABLES[*i]);
        seeds = compose_tables(&seeds, &new_tables);
    }

    vec![
        seeds[0..83].to_vec(),
        seeds[83..168].to_vec(),
        seeds[168..=255].to_vec()
    ]
}

pub fn get_suffix_and_prefix_length(tables: &Vec<Vec<Table>>) -> (usize, usize) {
    (
        tables[0][0].0[0] as usize + tables[0][0].0[1] as usize,
        tables[0][0].0[2] as usize + tables[0][0].0[3] as usize
    )
}