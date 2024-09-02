extern crate electrum_client;

use std::time::Duration;

use electrum_client::{Client, ElectrumApi};

fn main() {
    let client = Client::new("tcp://electrs.cakewallet.com:50001").unwrap();
    let res = client.tweaks_subscribe(859_430, 25, None);
    std::thread::sleep(Duration::from_secs(2));
    let (txid, tweak) = client.tweak_pop().unwrap().unwrap();
    println!("TXID: {txid}; Tweak: {tweak}");
}
