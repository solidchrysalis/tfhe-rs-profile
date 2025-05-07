extern crate tfhe;

use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32Array, FheUint32, FheBool};

fn main() {
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let clear_xs = (0..3 as u32).collect::<Vec<_>>();
    let clear_ys = vec![2u32; 3];

    // Encrypted 1D array with values
    // [0, 1, 2]
    let xs = FheUint32Array::try_encrypt((clear_xs.as_slice(), vec![3, 1]), &client_key).unwrap();
    // Encrypted 1D array with values
    // [2,  2,  2]
    let ys = FheUint32Array::try_encrypt((clear_ys.as_slice(), vec![3, 1]), &client_key).unwrap();

    let bools = [true, true, true, true]
        .into_iter()
        .map(|b| FheBool::try_encrypt(*b, &client_key))
        .collect::<Vec<_>>();

    assert_eq!(xs.num_dim(), 2);
    assert_eq!(xs.shape(), &[3, 1]);
    assert_eq!(ys.num_dim(), 2);
    assert_eq!(ys.shape(), &[3, 1]);

    let r = &xs * &ys;
    let r_result: Vec<u32> = r.decrypt(&client_key);
    assert_eq!(r_result, vec![0, 2, 4]);

    let sum: u32 = 0u32;
    let mut sum_encrypt= FheUint32::dot_product(&bools, &r);
    let sum_result = sum_encrypt.decrypt(&client_key);
    assert_eq!(sum_result, 6);
}

