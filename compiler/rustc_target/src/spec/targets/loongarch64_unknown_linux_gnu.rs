use crate::spec::{
    Arch, CodeModel, LlvmAbi, SanitizerSet, Target, TargetMetadata, TargetOptions, base,
};

pub(crate) fn target() -> Target {
    let oldworld = option_env!("RUSTC_LOONGARCH_OLDWORLD") == Some("1");
    Target {
        llvm_target: "loongarch64-unknown-linux-gnu".into(),
        metadata: TargetMetadata {
            description: Some("LoongArch64 Linux, LP64D ABI (kernel 5.19, glibc 2.36)".into()),
            tier: Some(2),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: if oldworld {
            "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
        } else {
            "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128"
        }
        .into(),
        arch: Arch::LoongArch64,
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: if oldworld { "loongarch64" } else { "generic" }.into(),
            features: "+f,+d,+lsx,+relax".into(),
            llvm_abiname: LlvmAbi::Lp64d,
            max_atomic_width: Some(64),
            mcount: "_mcount".into(),
            supported_sanitizers: SanitizerSet::ADDRESS
                | SanitizerSet::CFI
                | SanitizerSet::LEAK
                | SanitizerSet::MEMORY
                | SanitizerSet::THREAD,
            supports_xray: true,
            direct_access_external_data: Some(false),
            ..base::linux_gnu::opts()
        },
    }
}
