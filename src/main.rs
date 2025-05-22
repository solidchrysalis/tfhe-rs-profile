extern crate tfhe;

use tfhe::{generate_keys, prelude::{FheDecrypt, FheEncrypt}, set_server_key, ConfigBuilder, FheUint, FheUint8, FheUint8Id};

fn dot_product(a: Vec<FheUint<FheUint8Id>>, b: Vec<FheUint<FheUint8Id>>) -> FheUint<FheUint8Id> {
    let mult: Vec<FheUint<FheUint8Id>> = (0..a.len()).map(|i| a[i].clone() * b[i].clone()).collect();
    let sum = mult.iter().sum();
    return sum;
}

fn main() {
    // We generate a set of client/server keys, using the default parameters:
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);

    // Probably best to use flattened arrays for cache performance. Maybe transpose this one to preserve spatial locality.
    let msg1: Vec<Vec<u8>> = vec![
        vec![1,  2,  3,  4],
        vec![5,  6,  7,  8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    let msg2: Vec<Vec<u8>> = vec![
        vec![1,  1,  1,  1],
        vec![2,  2,  2,  2],
        vec![3, 3, 3, 3],
        vec![4, 4, 4, 4],
    ];
    
    set_server_key(server_key);
    let a = msg1.into_iter().map(|row| row.into_iter().map(|x| FheUint8::encrypt(x, &client_key)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let b = msg2.into_iter().map(|row| row.into_iter().map(|x| FheUint8::encrypt(x, &client_key)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let ax = a.len();
    let bx = b.len();
    let ay = a[0].len();
    let by = b[0].len();

    assert_eq!(ay, bx);
    let mut zeros: Vec<Vec<u8>> = vec![vec![0; bx]; ay];

    let mut local_sum: FheUint8;
    for i in 0..ay {
        let m: Vec<FheUint<FheUint8Id>> = (0..ay).map(|row| a[i][row].clone()).collect();
        for j in 0..bx {
            let n = b[j].clone();
            local_sum = dot_product(m.clone(), n);
            zeros[i][j] = local_sum.decrypt(&client_key);
        }
    }

}
