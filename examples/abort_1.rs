use std::time::Duration;

fn main() {
    std::thread::spawn(|| {
        let _item = Item {};
        panic!("thread panic!")
    });
    std::thread::sleep(Duration::from_secs(10));
}

struct Item {}

impl Drop for Item {
    fn drop(&mut self) {
        panic!("panic in drop!")
    }
}
