#[cfg(test)]
mod properties;

pub mod utils;
mod seeds;

#[cfg(test)]
pub use properties::is_random;

pub use seeds::RANDOM_TABLES;

/*

type Table := u8 -> u8;
fn Inverse: Table -> Table;
fn Compose: Table -> Table -> Table;
fn Decompose: Table -> Table -> Table;

// Definitions
def_table :=
  forall (f: Table) (a b: u8), a != b -> f(a) != f(b);
def_inverse :=
  forall (f: Table) (a: u8), f(Inverse(f)(a)) = a;
def_compose :=
  forall (f g: Table) (a: u8), Compose(f, g)(a) = f(g(a));
def_decompose :=
  forall (f g h: Table), Compose(f, g) = h -> Decompose(h, f) = g;

// Uniqueness
unique_table :=
  forall (f: Table) (a b: u8), a != b -> f(a) != f(b);
unique_inverse :=
  forall (f g: Table), f != g -> Inverse(f) != Inverse(g);
unique_compose_r :=
  forall (f g h: Table), g != h -> Compose(f, g) != Compose(f, h);
unique_compose_l :=
  forall (f g h: Table), g != h -> Compose(g, f) != Compose(h, f);
unique_decompose_r :=
  forall (f g h: Table), g != h -> Decompose(f, g) != Decompose(f, h);
unique_decompose_l :=
  forall (f g h: Table), g != h -> Decompose(g, f) != Decompose(h, f);

// Composition Properties
compose_commutative :=
  exists (f g: Table), Compose(f, g) != Compose(g, f);
compose_associativity :=
  forall (f g h: Table), Compose(f, Compose(g, h)) = Compose(Compose(f, g), h);
compose_id :=
  forall (f g: Table) (a: u8), f(a) = a -> Compose(f, g) = g and Compose(g, f) = g;

// TODO: more properties
// connectivity: for f: Table, let g(0) = 0, g(1) = f(0), g(2) = f(f(0)), g(3) = f(f(f(0))), ...
//    if g is a Table, f is fully connected
*/

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Table(pub Vec<u8>);

pub fn apply(table: &Table, n: u8) -> u8 {
    table.0[n as usize]
}

pub fn inverse_table(table: &Table) -> Table {
    let mut result = vec![0;256];

    for (i, c) in table.0.iter().enumerate() {
        result[*c as usize] = i as u8;
    }

    Table(result)
}

// f(g(a))
pub fn compose_table(f: &Table, g: &Table) -> Table {
    Table((0..256).map(|i| f.0[g.0[i] as usize]).collect())
}

// h(a) = f(g(a))
pub fn decompose_table(h: &Table, f: &Table) -> Table {
    compose_table(&inverse_table(f), h)
}

#[cfg(test)]
pub fn random_table() -> Table {
    let mut result = Vec::with_capacity(256);
    let mut unused = (0..=255).collect::<Vec<u8>>();

    for _ in 0..256 {
        result.push(unused.swap_remove(rand::random::<usize>() % unused.len()));
    }

    if !is_random(&result) {
        random_table()
    }

    else {
        Table(result)
    }

}

// it simulates `forall (f: Table)` when testing
// Don't use it as a static var
//   -> so that it tests with different random tables every time
#[cfg(test)]
pub fn many_tables() -> Vec<Table> {
    let unrandom = vec![
        id_table(),
        shifted_table(1),
        shifted_table(2),
        shifted_table(3),
        symmetric_table()
    ];

    let random = (0..24).map(|_| random_table()).collect::<Vec<Table>>();

    vec![random, unrandom].concat()
}

#[cfg(test)]
pub fn id_table() -> Table {
    Table((0..=255).collect())
}

#[cfg(test)]
fn shifted_table(n: usize) -> Table {
    Table((0..256).map(|i| ((i + n) % 256) as u8).collect())
}

#[cfg(test)]
fn symmetric_table() -> Table {
    Table((0..=255).map(|i| 255 - i).collect())
}