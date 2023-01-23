use std::env;
use std::fs::File;
use std::io::{Read, Write};
use h_encrypt::{encrypt_bytes, encrypt_stream, decrypt_bytes, decrypt_stream};

const MAJOR_VERSION: usize = 0;
const MINOR_VERSION: usize = 2;
const PATCH_VERSION: usize = 3;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {

        if args[1] == "--help".to_string() || args[1] == "-h".to_string() {
            print_help();
        }

        else if args[1] == "--version".to_string() || args[1] == "-v".to_string() {
            println!("{}.{}.{}", MAJOR_VERSION, MINOR_VERSION, PATCH_VERSION);
        }

        else if args[1] == "--encrypt".to_string() || args[1] == "-e".to_string() {

            if args.len() < 5 {
                println!("Invalid number of arguments!\n\n");
                print_help();
                return;
            }

            match File::open(&args[2]) {
                Ok(mut f) => {

                    match f.metadata() {
                        Ok(m) => if m.len() > 2_000_000 {  // 32 MB
                            match encrypt_stream(&args[2], &args[3], &args[4].as_bytes(), 1_000_000) {
                                Err(e) => {
                                    println!("Encryption Error: {:?}", e);
                                },
                                _ => {
                                    println!("File Encryption Successful");
                                }
                            }
                        } else {
                            let mut buf = vec![0; m.len() as usize];

                            match f.read(&mut buf) {
                                Err(e) => {
                                    println!("File IO Error with {}: {:?}", args[2], e);
                                }
                                _ => {}
                            }

                            let enc = encrypt_bytes(&buf, &args[4].as_bytes());

                            match File::create(&args[3]) {
                                Ok(mut ff) => {

                                    match ff.write_all(&enc) {
                                        Err(e) => {
                                            println!("File IO Error with {}: {:?}", args[3], e);
                                        }
                                        Ok(_) => {
                                            println!("File Encryption Successful");
                                        }
                                    }

                                }
                                Err(e) => {
                                    println!("File IO Error with {}: {:?}", args[3], e);
                                }
                            }

                        }
                        Err(e) => {
                            println!("File IO Error with {}: {:?}", args[2], e);
                        }
                    }

                }
                Err(e) => {
                    println!("File IO Error with {}: {:?}", args[2], e);
                }
            }

        }

        else if args[1] == "--decrypt".to_string() || args[1] == "-d".to_string() {

            if args.len() < 5 {
                println!("Invalid number of arguments!\n\n");
                print_help();
                return;
            }

            match File::open(&args[2]) {
                Ok(mut f) => {

                    match f.metadata() {
                        Ok(m) => if m.len() > 2_000_000 {  // 32 MB
                            match decrypt_stream(&args[2], &args[3], &args[4].as_bytes(), 1_000_000) {
                                Err(e) => {
                                    println!("Decryption Error: {:?}", e);
                                },
                                _ => {
                                    println!("File Decryption Successful");
                                }
                            }
                        } else {
                            let mut buf = vec![0; m.len() as usize];

                            match f.read(&mut buf) {
                                Err(e) => {
                                    println!("File IO Error with {}: {:?}", args[2], e);
                                }
                                _ => {}
                            }

                            let enc = match decrypt_bytes(&buf, &args[4].as_bytes()) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("Decryption Error: {:?}", e);
                                    return;
                                }
                            };

                            match File::create(&args[3]) {
                                Ok(mut ff) => {

                                    match ff.write_all(&enc) {
                                        Err(e) => {
                                            println!("File IO Error with {}: {:?}", args[3], e);
                                        }
                                        Ok(_) => {
                                            println!("File Decryption Successful");
                                        }
                                    }

                                }
                                Err(e) => {
                                    println!("File IO Error with {}: {:?}", args[3], e);
                                }
                            }

                        }
                        Err(e) => {
                            println!("File IO Error with {}: {:?}", args[2], e);
                        }
                    }

                }
                Err(e) => {
                    println!("File IO Error with {}: {:?}", args[2], e);
                }
            }

        }

        else {
            println!("Invalid Arguments!\n\n");
            print_help();
        }

    }

    else {
        print_help();
    }

}

fn print_help() {
    println!(
"h_encrypt {}.{}.{}
    encryption/decryption utility written from scratch (including the algorithm).

    -h / --help
        show this message

    -v / --version
        show version

    -e / --encrypt <path 1> <path 2> <password>
        read <path 1>, encrypt the read data with <password>, and write the encrypted data to <path 2>

    -d / --decrypt <path 1> <path 2> <password>
        read <path 1>, decrypt the read data with <password>, and write the decrypted data to <path 2>
",
    MAJOR_VERSION,
    MINOR_VERSION,
    PATCH_VERSION
    );
}