//@ only-loongarch64
//@ only-linux
//@ ignore-stage0
//@ ignore-cross-compile
//@ needs-llvm-components: loongarch

use std::env;

use run_make_support::object::Endianness;
use run_make_support::object::read::archive::ArchiveFile;
use run_make_support::object::read::elf::FileHeader as _;
use run_make_support::{cmd, dynamic_lib_name, object, rfs, rustc};

type FileHeader = object::elf::FileHeader64<Endianness>;

fn expected_abi_flags() -> u32 {
    if env::var("RUSTC_LOONGARCH_OLDWORLD").as_deref() == Ok("1") { 0x3 } else { 0x43 }
}

fn assert_abi_flags(data: &[u8], expected: u32) {
    let header = FileHeader::parse(&*data).unwrap();
    let flags = header.e_flags(header.endian().unwrap());
    assert_eq!(flags, expected);
}

fn assert_file_abi_flags(path: &str, expected: u32) {
    assert_abi_flags(&rfs::read(path), expected);
}

fn assert_target_features() {
    let output = rustc()
        .target("loongarch64-unknown-linux-gnu")
        .print("target-features")
        .run()
        .stdout_utf8();
    for feature in ["f", "d", "lsx"] {
        assert!(output.lines().any(|line| line.split_whitespace().next() == Some(feature)));
    }
    if expected_abi_flags() == 0x3 {
        let cfg = rustc().target("loongarch64-unknown-linux-gnu").print("cfg").run().stdout_utf8();
        assert!(!cfg.lines().any(|line| line == "target_has_reliable_f16"));
        assert!(!cfg.lines().any(|line| line == "target_has_reliable_f16_math"));
    }
}

fn assert_rlib_abi_flags(path: &str, expected: u32) {
    let data = rfs::read(path);
    let archive = ArchiveFile::parse(&*data).unwrap();
    let mut checked = 0;
    for member in archive.members() {
        let member = member.unwrap();
        if member.name().ends_with(b".rmeta")
            || member.name().ends_with(b".rmeta-link")
            || member.name().ends_with(b".o")
        {
            assert_abi_flags(member.data(&*data).unwrap(), expected);
            checked += 1;
        }
    }
    assert!(checked > 0);
}

fn assert_oldworld_rejects_float_feature_overrides() {
    for feature in
        ["-f", "-d", "-fp64", "+single-float", "-single-float", "+soft-float", "-soft-float"]
    {
        rustc()
            .input("lib.rs")
            .target("loongarch64-unknown-linux-gnu")
            .crate_type("lib")
            .arg(format!("-Ctarget-feature={feature}"))
            .run_fail()
            .assert_exit_code(1)
            .assert_stderr_contains("cannot override ABI-required floating-point features");
    }
}

fn main() {
    let expected = expected_abi_flags();
    assert_target_features();

    rustc()
        .input("lib.rs")
        .target("loongarch64-unknown-linux-gnu")
        .crate_type("lib")
        .emit("obj")
        .output("abi.o")
        .run();
    assert_file_abi_flags("abi.o", expected);

    if expected == 0x3 {
        assert_oldworld_rejects_float_feature_overrides();
    }

    rustc()
        .input("lib.rs")
        .target("loongarch64-unknown-linux-gnu")
        .crate_name("abi_rlib")
        .crate_type("lib")
        .output("libabi_rlib.rlib")
        .run();
    assert_rlib_abi_flags("libabi_rlib.rlib", expected);

    rustc()
        .input("lib.rs")
        .target("loongarch64-unknown-linux-gnu")
        .crate_name("abi_metadata")
        .crate_type("dylib")
        .run();
    assert_file_abi_flags(&dynamic_lib_name("abi_metadata"), expected);

    rustc()
        .input("thread.rs")
        .target("loongarch64-unknown-linux-gnu")
        .extern_("abi_rlib", "libabi_rlib.rlib")
        .output("thread-program")
        .run();
    assert_file_abi_flags("thread-program", expected);
    cmd("./thread-program").run();
}
