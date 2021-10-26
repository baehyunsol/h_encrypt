fn hash_small1(n: u64) -> u64 {
    (n % 1_009 + 100_101) * (n % 1_013 + 100_202) / 0x80 % 0x10_000
}


fn hash_small2(n: u64) -> u64 {
    (n % 1_019 + 100_303) * (n % 1_021 + 100_404) / 0x80 % 0x10_000
}


fn hash_small3(n: u64) -> u64 {
    (n % 1_031 + 100_505) * (n % 1_033 + 100_606) / 0x80 % 0x10_000
}


fn hash_small4(n: u64) -> u64 {
    (n % 1_039 + 100_707) * (n % 1_049 + 100_808) / 0x80 % 0x10_000
}


pub fn hash1(n: u64) -> u64 {
    hash_small1(n) +
    hash_small2(n) * 0x10_000 +
    hash_small3(n) * 0x100_000_000 +
    hash_small4(n) * 0x1_000_000_000_000
}


pub fn hash_string1(s: &Vec<u8>) -> u64 {

    let mut last = 0;

    for c in s.iter() {
        last = hash1(*c as u64) ^ hash1(last);
    }

    hash1(last)
}


fn hash_small5(n: u64) -> u64 {
    (n % 2_003 + 200_101) * (n % 2_011 + 200_202) / 0x80 % 0x10_000
}


fn hash_small6(n: u64) -> u64 {
    (n % 2_017 + 200_303) * (n % 2_027 + 200_404) / 0x80 % 0x10_000
}


fn hash_small7(n: u64) -> u64 {
    (n % 2_029 + 200_505) * (n % 2_039 + 200_606) / 0x80 % 0x10_000
}


fn hash_small8(n: u64) -> u64 {
    (n % 2_053 + 200_707) * (n % 2_063 + 200_808) / 0x80 % 0x10_000
}


pub fn hash2(n: u64) -> u64 {
    hash_small5(n) +
    hash_small6(n) * 0x10_000 +
    hash_small7(n) * 0x100_000_000 +
    hash_small8(n) * 0x1_000_000_000_000
}


pub fn hash_string2(s: &Vec<u8>) -> u64 {

    let mut last = 0;

    for c in s.iter() {
        last = hash2(*c as u64) ^ hash2(last);
    }

    hash2(last)
}