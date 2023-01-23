use crate::table::*;
use std::collections::HashSet;

// not strict
// not formal
// `f` need not be a `Table`
pub fn is_random(f: &Vec<u8>) -> bool {
    let mut inc = 0;
    let mut big = 0;

    for i in 0..255 {

        if f[i + 1] > f[i] {
            inc += 1;
        }

        if f[i] > i as u8 {
            big += 1;
        }

    }

    if inc < 64 || inc > 192 || big < 64 || big > 192 {
        return false;
    }

    for i in 0..4 {
        let sum = f[(0 + i * 64)..(64 + i * 64)].iter().map(|n| *n as u32).sum::<u32>() / 64;

        if sum < 64 || sum > 192 {
            return false;
        }

    }

    let filtered = f[0..128].iter().filter(|n| *n % 3 == 0).collect::<Vec<&u8>>().len();

    if filtered < 21 || filtered > 85 {
        return false;
    }

    true
}

// def_table :=
//   forall (f: Table) (a b: u8), a != b -> f(a) != f(b);
fn def_table(f: &Table) {
    assert_eq!(f.0.len(), 256);
    let mut outputs = HashSet::with_capacity(256);

    for n in f.0.iter() {
        assert!(!outputs.contains(n));
        outputs.insert(*n);
    }

}

#[test]
fn def_table_proof() {

    for table in many_tables().iter() {
        def_table(table);
    }

}

// def_inverse :=
//   forall (f: Table) (a: u8), f(Inverse(f)(a)) = a;
fn def_inverse(f: &Table) {

    for a in 0..=255 {
        assert_eq!(apply(f, apply(&inverse_table(f), a)), a);
    }

}

#[test]
fn def_inverse_proof() {

    for table in many_tables().iter() {
        def_inverse(table);
    }

}

// def_compose :=
//   forall (f g: Table) (a: u8), Compose(f, g)(a) = f(g(a));
fn def_compose(f: &Table, g: &Table) {

    for a in 0..=255 {
        assert_eq!(apply(&compose_table(f, g), a), apply(f, apply(g, a)));
    }

}

#[test]
fn def_compose_proof() {

    for f in many_tables().iter() {

        for g in many_tables().iter() {
            def_compose(f, g);
        }

    }

}

// def_decompose :=
//   forall (f g h: Table), Compose(f, g) = h -> Decompose(h, f) = g;
fn def_decompose(f: &Table, g: &Table) {
    let h = compose_table(f, g);
    let new_g = decompose_table(&h, f);

    assert_eq!(g, &new_g);
}

#[test]
fn def_decompose_proof() {

    for f in many_tables().iter() {

        for g in many_tables().iter() {
            def_decompose(f, g);
        }

    }

}

/*
// Totalness
total_table :=
  forall (a: u8) (f: Table), exists (b: u8), f(a) = b;
total_inverse :=
  forall (f: Table), exists (g: Table), Inverse(f) = g;
total_compose :=
  forall (f g: Table), exists (h: Table), Compose(f, g) = h;
total_decompose :=
  forall (f g: Table), exists (h: Table), Decompose(f, g) = h;

Proofs: Rust code compiles and runs without getting stuck
*/

/*
// Uniqueness
unique_table :=
  forall (f: Table) (a b: u8), a != b -> f(a) != f(b);
Proof ::
  by def_table

unique_inverse :=
  forall (f g: Table), f != g -> Inverse(f) != Inverse(g);
Proof ::
  assume exists (f1 f2: Table), f1 != f2 and Inverse(f1) = Inverse(f2)
  exists (n: u8), f1(n) != f2(n)
    let f1(n) = m1, f2(n) = m2
  by def_inverse, Inverse(f1)(m1) = n, Inverse(f2)(m2) = n
  by unique_table, m1 = m2
    Contradiction

unique_compose_r :=
  forall (f g h: Table), g != h -> Compose(f, g) != Compose(f, h);
Proof ::
  assume exists (g1 g2: Table), g1 != g2 and Compose(f, g1) = Compose(f, g2)
  exists (n: u8), g1(n) != g2(n)
    let g1(n) = m1, g2(n) = m2
  by def_compose, f(g1(n)) = f(g2(n))
  f(m1) = f(m2)
  by unique_table, m1 = m2
    Contradiction

unique_compose_l :=
  forall (f g h: Table), g != h -> Compose(g, f) != Compose(h, f);
unique_decompose_r :=
  forall (f g h: Table), g != h -> Decompose(f, g) != Decompose(f, h);
unique_decompose_l :=
  forall (f g h: Table), g != h -> Decompose(g, f) != Decompose(h, f);
*/

// compose_commutative :=
//   exists (f g: Table), Compose(f, g) != Compose(g, f);
#[test]
fn compose_commutative_proof() {

    // if the lemma is false, it'll never break
    loop {
        let f = random_table();
        let g = random_table();

        if compose_table(&f, &g) != compose_table(&g, &f) {
            break;
        }

    }

}

// compose_associativity :=
//   forall (f g h: Table), Compose(f, Compose(g, h)) = Compose(Compose(f, g), h);
fn compose_associativity(f: &Table, g: &Table, h: &Table) {
    assert_eq!(compose_table(f, &compose_table(g, h)), compose_table(&compose_table(f, g), h));
}

#[test]
fn compose_associativity_proof() {

    for f in many_tables().iter() {

        for g in many_tables().iter() {

            for h in many_tables().iter() {
                compose_associativity(f, g, h);
            }

        }

    }

}

// compose_id :=
//   forall (f g: Table) (a: u8), f(a) = a -> Compose(f, g) = g and Compose(g, f) = g;
fn compose_id(g: &Table) {
    let f = id_table();

    assert_eq!(&compose_table(&f, g), g);
    assert_eq!(&compose_table(g, &f), g);
}

#[test]
fn compose_id_proof() {

    for g in many_tables().iter() {
        compose_id(g);
    }

}
