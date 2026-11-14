#![feature(stdarch_loongarch)]

pub fn exported_function() -> i32 {
    42
}

pub extern "C" fn fp64_add_one(value: f64) -> f64 {
    value + 1.0
}

pub extern "C" fn fp32_add_one(value: f32) -> f32 {
    value + 1.0
}

pub fn instruction_barriers() {
    core::arch::loongarch64::ibar::<0>();
    core::arch::loongarch64::dbar::<0>();
}
