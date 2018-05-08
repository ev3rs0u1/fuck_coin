extern crate fuck_coin;

use fuck_coin::Coin;
use std::{thread, time};

fn main() {
    let mut coin = Coin::new("BTC", 60);
    coin.get_symbol_rate("CNY");

    loop {
        coin.get_symbol_price();
        coin.calculate_cny_price();

        println!("{:#?}\n", coin);
        thread::sleep(time::Duration::from_secs(coin.delay))
    }
}