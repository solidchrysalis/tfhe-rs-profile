extern crate tfhe;

use tfhe::array::FheArrayBase;
use tfhe::{prelude::*, ClientKey, FheUint32, FheUint32Slice};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32Array};

fn dot_product(a: FheUint32Slice, b: FheUint32Slice, size: usize, key: &ClientKey) -> FheUint32 {
    let r = a * b;
    let mut result = FheUint32Array::try_encrypt((, vec![1, 1]), key).unwrap();

    for n in 0..size {
        let test = r.get_slice(&[1..2, 0..1]).unwrap();
        
    }

    return result;
}

fn main() {
    let config: tfhe::Config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let num_elems: usize = 4 * 4;
    let clear_xs: Vec<u32> = (0..num_elems as u32).collect::<Vec<_>>();
    let clear_ys: Vec<u32> = vec![1u32; num_elems];

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

    let x_dim: &[usize] = xs.shape();
    let y_dim: &[usize] = ys.shape();
    //Only working for 2d arrays rn
    assert_eq!(xs.num_dim(), 2);
    assert_eq!(ys.num_dim(), 2);
    assert_eq!(x_dim.get(1), y_dim.get(0));

    let x_len: usize = x_dim[0];
    let y_len: usize = y_dim[0];
    let x_wid: usize = x_dim[1];
    let y_wid: usize = y_dim[1];

    for i in 0..x_wid {
        for j in 0..y_len {
            let x_vals = xs.get_slice(&[i..(i+1), 0..x_len]).unwrap();
            let y_vals = ys.get_slice(&[0..y_wid, j..(j+1)]).unwrap();
            let result = dot_product(x_vals, y_vals, y_wid, &client_key);
        }
    }

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