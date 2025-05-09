extern crate tfhe;

use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32Array};

fn main() {
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let num_elems = 4 * 4;
    let clear_xs = (0..num_elems as u32).collect::<Vec<_>>();
    let clear_ys = vec![1u32; num_elems];

    // Encrypted 2D array with values
    // [[  0,  1,  2,  3]
    //  [  4,  5,  6,  7]
    //  [  8,  9, 10, 11]
    //  [ 12, 13, 14, 15]]
    let xs = FheUint32Array::try_encrypt((clear_xs.as_slice(), vec![4, 4]), &client_key).unwrap();
    // Encrypted 2D array with values
    // [[  1,  1,  1,  1]
    //  [  1,  1,  1,  1]
    //  [  1,  1,  1,  1]
    //  [  1,  1,  1,  1]]
    let ys = FheUint32Array::try_encrypt((clear_ys.as_slice(), vec![4, 4]), &client_key).unwrap();

    let x_dim = xs.shape();
    let y_dim = ys.shape();
    //Only working for 2d arrays rn
    assert_eq!(xs.num_dim(), 2);
    assert_eq!(ys.num_dim(), 2);
    assert_eq!(x_dim.get(1), y_dim.get(0));

    //let test = xs.get_slice(&[1..2, 1..2]).unwrap();
    //let sum = ys.get_slice(&[1..2, 1..2]).unwrap();
    //let r = &test * &sum;
    //let result: Vec<u32> = r.decrypt(&client_key);
    //assert_eq!(result, vec![5]);
}

// Code without slicing
/* 
    // Encrypted 1D array wdith values
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
    */