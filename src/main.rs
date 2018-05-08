extern crate stackmas;

use stackmas::Stackmas;

fn main() {
    let mut stak = Stackmas::new();
    stak.push(1);
    stak.push(3);
    stak.push(5);
    for i in stak {
        println!("{:?}", i);
    }
}