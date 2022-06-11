use std::time::Duration;

// panic config in Cargo.toml:
// [profile.dev]
// panic = "abort"
//
// [profile.release]
// panic = "unwind"
//
// this will unwind the stack when program panic
// RUST_BACKTRACE=1 cargo run --example abort_2 --release
//
// this will exit the process
// cargo run --example abort_2

fn main() {
    std::thread::spawn(|| {
        panic!("thread panic!")
    });
    std::thread::sleep(Duration::from_secs(10));
}
