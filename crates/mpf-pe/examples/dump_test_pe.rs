//! Writes the synthetic test PE to disk so `mpf-cli info` can be tried on a
//! real file: `cargo run -p mpf-pe --example dump_test_pe -- /tmp/test.dll`

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/mpf_test.dll".to_string());
    std::fs::write(&path, mpf_pe::testing::build_test_pe64()).expect("write failed");
    println!("wrote {path}");
}
