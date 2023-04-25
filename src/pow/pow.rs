// A rust based implementation of a basic PoW algorithm

extern crate rand;
use rand::Rng;
use sha3::{Digest, Sha3_256};

const OUT_SIZE: usize = 8;
const IN_SIZE: usize = 32;
const DATA_SIZE: usize = 40;

type Hash = [u8; OUT_SIZE];
type InHash = [u8; IN_SIZE];
type Data = [u8; DATA_SIZE];

pub fn generate_pow(hash: &InHash, difficulty: u64) -> String {
    let target = get_target(difficulty);
    let entropy = get_random_seed();
    let mut data = get_data(&entropy, hash);
    let mut h = [0u8; OUT_SIZE];
    loop {
        compute_hash(&mut h, &data);
        if greater(&h, &target) {
            return hex::encode(data_to_nonce(&data));
        }
        if !next_data(&mut data, entropy.len()) {
            let new_entropy = get_random_seed();
            let new_data = get_data(&new_entropy, hash);
            data.clone_from_slice(&new_data);
        }
    }
}

pub fn benchmark_pow(difficulty: u64) -> String {
    let target = get_target(difficulty);
    let mut data = [0u8; DATA_SIZE];
    let mut h = [0u8; OUT_SIZE];
    loop {
        compute_hash(&mut h, &data);
        if greater(&h, &target) {
            return hex::encode(data_to_nonce(&data));
        }
        if !next_data(&mut data, OUT_SIZE) {
            data = [0u8; DATA_SIZE];
        }
    }
}

fn compute_hash(hash: &mut Hash, data: &Data) {
    let mut sha3 = Sha3_256::new();
    sha3.update(data);
    let digest = sha3.finalize();
    hash.copy_from_slice(&digest[0..8]);
}

fn get_target(difficulty: u64) -> Hash {
    let big: u128 = 1 << 64;
    let target: u128 = big - (big / difficulty as u128);

    // make little endian
    target.to_le_bytes()[0..8].try_into().unwrap()
}

// modify the nonce by incrementing it's bytes
fn next_data(data: &mut Data, max_size: usize) -> bool {
    for i in 0..max_size {
        data[i] = data[i].wrapping_add(1);
        if data[i] != 0 {
            return true;
        }
    }
    false
}

// Compare two hashes (little endian setup)
fn greater(a: &Hash, b: &Hash) -> bool {
    for i in (0..OUT_SIZE).rev() {
        if a[i] == b[i] {
            continue;
        }
        return a[i] > b[i];
    }
    true
}

// Return 8 bytes of entropy to be used the initial nonce
pub fn get_random_seed() -> Hash {
    let mut h: Hash = [0u8; OUT_SIZE];
    for i in 0..OUT_SIZE {
        h[i] = rand::thread_rng().gen();
    }
    h
}

// Constructs the final data which will be hashed. It will be concatenation of
// the entropy and the input hash (data = entropy || InHash)
fn get_data(entropy: &Hash, hash: &InHash) -> Data {
    let mut data = vec![0u8; DATA_SIZE];

    for i in 0..entropy.len() {
        data[i] = entropy[i];
    }

    for i in 0..hash.len() {
        data[i + entropy.len()] = hash[i];
    }

    data.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!(
            "Failed converting into [u8; DATA_SIZE]. Vec was of length {}",
            v.len()
        )
    })
}

// Extract the first 8 bytes from the data, which represent the nonce
fn data_to_nonce(data: &Data) -> Hash {
    let mut nonce: Hash = [0u8; 8];
    nonce.copy_from_slice(&data[0..8]);
    nonce
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn test_generate_pow() {
        let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });

        for i in 0..10 {
            let start = Instant::now();
            let x = generate_pow(&in_hash, 1 << i);
            let duration = start.elapsed();
            println!(
                "Round number: {} - {} // Time: {}ns",
                i,
                &x,
                duration.as_nanos()
            );
        }
    }

    #[test]
    fn test_benchmark_pow() {
        for i in 0..10 {
            let start = Instant::now();
            let x = benchmark_pow(1 << i);
            let duration = start.elapsed();
            println!(
                "Round number: {} - {} // Time: {}ns",
                i,
                &x,
                duration.as_nanos()
            );
        }
    }

    #[test]
    fn test_compute_hash() {
        // generate some random hash in data and set up the scene
        let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });
        let entropy = get_random_seed();

        // get data
        let data = get_data(&entropy, &in_hash);
        assert_eq!(data.len(), DATA_SIZE);

        // compute the  hash
        let mut hash: Hash = [0; OUT_SIZE];
        compute_hash(&mut hash, &data);
    }

    #[test]
    fn test_get_data() {
        // generate some random hash in data and set up the scene
        let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });
        let entropy = get_random_seed();

        // get data
        let data = get_data(&entropy, &in_hash);
        assert_eq!(data.len(), DATA_SIZE);
    }

    #[test]
    fn test_get_target() {
        let diff1: u64 = rand::thread_rng().gen();
        let diff2: u64 = diff1 + 1;
        let tar1 = get_target(diff1);
        let tar2 = get_target(diff2);
        assert_eq!(tar1.len(), OUT_SIZE);
        assert_eq!(tar2.len(), OUT_SIZE);
        assert!(greater(&tar2, &tar1));
    }

    #[test]
    fn test_utils() {
        let mut nd1 = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut nd2 = [
            255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert!(next_data(&mut nd1, 8));
        assert!(!next_data(&mut nd2, 8));

        assert!(super::greater(
            &[0, 0, 0, 0, 0, 0, 0, 2],
            &[9, 0, 0, 0, 0, 0, 0, 1]
        ));
    }
}
