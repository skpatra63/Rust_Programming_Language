fn main() {
    let mut missiles = 8; // Fix error by doing: let mut missiles = 8
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready; // Error!
    println!("{} missiles left", missiles);
}
