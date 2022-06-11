// cargo run --example unwind
// RUST_BACKTRACE=1 cargo run --example unwind

fn main() {
    pirate_share(100, 0);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
