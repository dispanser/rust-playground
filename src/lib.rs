// we declare primes as a publicly accessible module.
// this is necessary to allow users of our "lib" to actually call a method.
pub mod primes;

pub fn lib_method() -> () {
    println!("lib method here!");
}