//@ only-loongarch64
//@ only-linux
//@ check-pass
//@ compile-flags: --target loongarch64-unknown-linux-gnu
//@ needs-llvm-components: loongarch

#![crate_type = "lib"]

const _: [(); 3] = [();
    cfg!(target_feature = "f") as usize
        + cfg!(target_feature = "d") as usize
        + cfg!(target_feature = "lsx") as usize
];
