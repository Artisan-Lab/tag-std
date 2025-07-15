src_dir="../target/safety-tool/lib"
dst_file="libsafety_macro.so"
found=( "$src_dir"/libsafety_macro*.so )

DIR_LIB         := ../target/safety-tool/lib
SAFETY_MACRO_SO := $(wildcard $(DIR_LIB)/libsafety_macro*.so)
SAFETY_MACRO    := libsafety_macro.so

all: pin-init

$(SAFETY_MACRO):
	@if [ -z '$(SAFETY_MACRO_SO)' ]; then \
	    echo "ERROR: $(DIR_LIB)/libsafety_macro*.so not found" >&2; \
	    exit 1; \
	fi
	cp -f $(firstword $(SAFETY_MACRO_SO)) $@

libsafety_lib.rlib: $(SAFETY_MACRO)
	RUSTC_BOOTSTRAP=1 rustc --edition=2021 src/lib.rs --crate-name=safety_lib --crate-type lib \
		-L ../target/safety-tool/lib \
		--target aarch64-unknown-none-softfloat \
		-Z unstable-options \
		--sysroot=/dev/null \
		--extern safety_macro=$(SAFETY_MACRO) \
		--extern core=/home/gh-zjp-CN/tag-std/linux/rust/libcore.rmeta \
		--extern compiler_builtins=/home/gh-zjp-CN/tag-std/linux/rust/libcompiler_builtins.rmeta 


pin-init: libsafety_lib.rlib
	cd ../../linux && RUSTC_BOOTSTRAP=1 \
		rustc --edition=2021 -Zbinary_dep_depinfo=y -Astable_features -Dnon_ascii_idents \
			-Dunsafe_op_in_unsafe_fn -Wmissing_docs -Wrust_2018_idioms -Wunreachable_pub -Wclippy::all \
			-Wclippy::as_ptr_cast_mut -Wclippy::as_underscore -Wclippy::cast_lossless -Wclippy::ignored_unit_patterns \
			-Wclippy::mut_mut -Wclippy::needless_bitwise_bool -Aclippy::needless_lifetimes -Wclippy::no_mangle_with_rust_abi \
			-Wclippy::ptr_as_ptr -Wclippy::ptr_cast_constness -Wclippy::ref_as_ptr -Wclippy::undocumented_unsafe_blocks \
			-Wclippy::unnecessary_safety_comment -Wclippy::unnecessary_safety_doc -Wrustdoc::missing_crate_level_docs \
			-Wrustdoc::unescaped_backticks -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n \
			-Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n \
			-Wclippy::float_arithmetic --target=aarch64-unknown-none-softfloat -Cforce-unwind-tables=n \
			-Zbranch-protection=bti,pac-ret -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y \
			-Cforce-frame-pointers=y -Cdebuginfo=1 @./include/generated/rustc_cfg \
			--extern pin_init_internal --extern macros --cfg kernel --emit=dep-info=rust/.pin_init.o.d \
			--emit=obj=rust/pin_init.o --emit=metadata=rust/libpin_init.rmeta --crate-type rlib -L./rust \
			--crate-name pin_init rust/pin-init/src/lib.rs --sysroot=/dev/null \
			-Zcrate-attr="feature(register_tool)" -Zcrate-attr="register_tool(rapx)" \
			-L /home/gh-zjp-CN/tag-std/safety-tool/safety-lib/ \
			--extern=safety=/home/gh-zjp-CN/tag-std/safety-tool/safety-lib/libsafety_lib.rlib

clean:
	rm -rf $(SAFETY_MACRO)
	rm -rf libsafety_lib.rlib

.PHONY: all clean pin-init
