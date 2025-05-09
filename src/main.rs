extern crate tfhe;

use tfhe::{prelude::*, FheUint32};
use tfhe::{generate_keys, set_server_key, ConfigBuilder};

fn main() {
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let clear_xs = (0..3 as u32).collect::<Vec<_>>();
    let clear_ys = vec![2u32; 3];

    // Encrypted 1D array with values
    // [0, 1, 2]
    //let xs = FheUint32Array::try_encrypt((clear_xs.as_slice(), vec![3, 1]), &client_key).unwrap();
    let xxs: Vec<FheUint32> = clear_xs.iter().map(|&x| FheUint32::encrypt(x, &client_key)).collect();
    // Encrypted 1D array with values
    // [2,  2,  2]
    //let ys = FheUint32Array::try_encrypt((clear_ys.as_slice(), vec![3, 1]), &client_key).unwrap();
    let yys: Vec<FheUint32> = clear_ys.iter().map(|&x| FheUint32::encrypt(x, &client_key)).collect();

    let mut result = vec![];

    for n in 0..3 {
        result.push(xxs.get(n).unwrap() * yys.get(n).unwrap());
    }

    let mut sum = FheUint32::encrypt(0u32, &client_key);

    for n in 0..3 {
        sum += result.get(n).unwrap();
    }
    
    let help: u32 = FheUint32::decrypt(&sum, &client_key);

    assert_eq!(help, 6);
}

