use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S32".into(),
        llvm_target: "a32".into(),
        pointer_width: 32,
        arch: "a32".into(),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::No),
            linker: Some("ld".into()),
            cpu: "a32".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            main_needs_argc_argv: false,
            atomic_cas: false,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            generate_arange_section: false,
            supports_stack_protector: false,
            ..Default::default()
        },
    }
}
