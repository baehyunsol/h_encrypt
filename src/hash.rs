use crate::err::EncError;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn salt(s: &[u8]) -> Vec<u8> {
    vec![
        vec![(s.len() % 256) as u8; 4],
        vec![if s.len() > 0 { s[0] } else { ' ' as u8 }; 4],
        vec![if s.len() > 0 { s[s.len() - 1] } else { ' ' as u8 }; 4],
        vec!['s' as u8, 'a' as u8, 'l' as u8, 't' as u8],
        s.to_vec()
    ].concat()
}

fn hash(s: &[u8]) -> u128 {
    let mut result = 0;

    for (i, c) in s.iter().enumerate() {
        result *= 273;
        result += i as u128 % 17;
        result += *c as u128;
        result %= 0x1_0000__0000_0000_0000__0000_0000_0000;
    }

    result
}

// the hash function makes sense only when `string` is long enough
pub fn hash_string(string: &[u8]) -> Vec<usize> {
    let mut hashed_string = if string.len() < 32 { hash(&salt(string)) } else { hash(string) };
    let mut result = Vec::with_capacity(28);

    while hashed_string > 0 {
        result.push((hashed_string % 256) as usize);
        hashed_string /= 256;
    }

    result
}

pub fn hash_stream<P: AsRef<Path>>(file_name: P, buf_size: usize) -> Result<Vec<usize>, EncError> {
    let mut result: u128 = 0;
    let mut index = 0;

    let mut file = File::open(file_name)?;

    // the hash function makes sense only when `string` is long enough
    if file.metadata()?.len() < 32 {
        let mut buf = vec![0; 32];
        let count = file.read(&mut buf)?;

        return Ok(hash_string(&buf[0..count]));
    }

    let mut buf = vec![0; buf_size];

    loop {
        let count = file.read(&mut buf)?;

        for c in buf[0..count].iter() {
            result *= 273;
            result += index as u128 % 17;
            result += *c as u128;
            result %= 0x1_0000__0000_0000_0000__0000_0000_0000;
            index += 1;
        }

        if count == 0 {
            break;
        }

    }

    let mut result_vec = Vec::with_capacity(28);

    while result > 0 {
        result_vec.push((result % 256) as usize);
        result /= 256;
    }

    Ok(result_vec)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn hash_cons() {
        let files = crate::tests::test_files();

        for file in files.iter() {
            let mut f = File::open(file.clone()).unwrap();
            let mut buf = vec![0; 0x1000_000];
            let count = f.read(&mut buf).unwrap();

            assert_eq!(hash_string(&buf[0..count]), hash_stream(file, 64).unwrap());
        }

    }

}