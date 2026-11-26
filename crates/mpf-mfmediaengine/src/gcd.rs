/// `get_gcd` from `dlls/mfmediaengine/main.c`: Euclid's algorithm, so `get_gcd(0, 0)` is 0.
pub fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let m = a % b;
        a = b;
        b = m;
    }
    a
}
