mod generator;
use crate::generator::*;

fn main() {
    let pass = genpwd(50, STRENTH::Strong);
    println!("{}", pass);
}
