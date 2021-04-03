use crypto::hashes::{Digest, blake2b};
use iota_streams::{
    app_channels::api::tangle::Address,
    core::Result,
};
use rand::{
    distributions::Uniform,
    Rng,
    thread_rng,
};


pub fn random_seed() -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9".as_bytes();
    thread_rng()
        .sample_iter(Uniform::new(0, alphabet.len()))
        .take(81)
        .map(|i| alphabet[i] as char)
        .collect()
}


pub fn get_hash(link: &Address) ->  Result<String>  {
    let total = [link.appinst.as_ref(), link.msgid.as_ref()].concat();
    let hash = blake2b::Blake2b256::digest(&total);
    Ok(hex::encode(&hash))
}