extern crate znn_sdk_rust as znn;

use znn::wallet::{keyfile::KeyFile, keystore::KeyStore};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    keyfile_example().await;
}

async fn keyfile_example() {
    let entropy: [u8; 32] = [
        188, 130, 125, 10, 0, 167, 35, 84, 220, 228, 196, 74, 89, 72, 82, 136, 80, 11, 73, 56, 47,
        155, 168, 138, 1, 99, 81, 120, 123, 123, 21, 202,
    ];
    let ks: KeyStore = KeyStore::from_entropy(entropy.to_vec()).unwrap();
    dbg!(&ks);
    let kf = KeyFile::encrypt(ks, "pwd".to_string()).await.unwrap();
    dbg!(&kf);
    assert_eq!(1, 1);
}
