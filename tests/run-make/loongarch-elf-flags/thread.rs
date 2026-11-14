extern crate abi_rlib;

fn main() {
    abi_rlib::instruction_barriers();
    assert_eq!(abi_rlib::fp64_add_one(41.0), 42.0);
    assert_eq!(abi_rlib::fp32_add_one(41.0), 42.0);

    let value = std::thread::spawn(|| 42).join().unwrap();
    assert_eq!(value, 42);
}
