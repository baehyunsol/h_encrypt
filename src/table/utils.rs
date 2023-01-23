use crate::table::{apply, compose_table, Table};

pub fn apply_higher_order<T: Clone + Default>(vec: &Vec<T>, table: &Table) -> Vec<T> {
    assert_eq!(vec.len(), 256);

    let mut result = vec![T::default(); 256];

    for i in 0..256 {
        result[i] = vec[apply(table, i as u8) as usize].clone();
    }

    result
}

pub fn compose_tables(v1: &Vec<Table>, v2: &Vec<Table>) -> Vec<Table> {
    assert_eq!(v1.len(), v2.len());

    (0..v1.len()).map(|i| compose_table(&v1[i], &v2[i])).collect()
}

#[cfg(test)]
mod tests {
    use super::apply_higher_order;
    use crate::table::seeds::RANDOM_TABLES;

    #[test]
    fn higher_order_correct() {
        let new_tables = apply_higher_order(&RANDOM_TABLES.to_vec(), &RANDOM_TABLES[0]);

        for table in new_tables.iter() {
            assert_eq!(table.0.len(), 256);
        }

    }

}