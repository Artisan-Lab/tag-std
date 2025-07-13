use assert_cmd::Command;
use expect_test::expect_file;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

static LD_LIBRARY_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let output = Command::new("rustc").arg("--print=sysroot").output().unwrap();
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        panic!("Failed to run `rustc --print=sysroot`:\nstdout={stdout}\nstderr={stderr}")
    }
    let path = PathBuf::from(stdout.trim()).join("lib");
    assert!(path.exists());
    path
});

#[derive(Clone, Copy)]
struct CompilationOptions<'a> {
    args: &'a [&'a str],
    envs: &'a [(&'a str, &'a str)],
    stop: bool,
    discharges_all_properties: bool,
}

impl Default for CompilationOptions<'_> {
    fn default() -> Self {
        Self {
            args: &["--crate-type=lib"],
            envs: &[],
            stop: true,
            discharges_all_properties: false,
        }
    }
}

impl CompilationOptions<'_> {
    fn discharges_all_properties() -> Self {
        CompilationOptions { discharges_all_properties: true, ..Default::default() }
    }
}

const STOP_COMPILATION: &str = "STOP_COMPILATION";
const DISCHARGES_ALL_PROPERTIES: &str = "DISCHARGES_ALL_PROPERTIES";

fn compile(file: &str, opts: CompilationOptions) -> (&'static str, std::process::Output) {
    let exe = env!("CARGO_PKG_NAME");
    let mut cmd = Command::cargo_bin(exe).unwrap();

    cmd.arg(file).args(opts.args).env("LD_LIBRARY_PATH", &*LD_LIBRARY_PATH);

    cmd.envs(opts.envs.iter().copied());

    if opts.stop {
        cmd.env(STOP_COMPILATION, "1");
    } else {
        cmd.env_remove(STOP_COMPILATION);
    }

    if opts.discharges_all_properties {
        cmd.env(DISCHARGES_ALL_PROPERTIES, "1");
    } else {
        cmd.env_remove(DISCHARGES_ALL_PROPERTIES);
    }

    let output = cmd.output().unwrap();
    (exe, output)
}

fn should_panic(file: &str, outfile: &str, opts: CompilationOptions) {
    let (exe, output) = compile(file, opts);
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    if output.status.success() {
        panic!("`{exe} {file}` should panic:\nstdout={stdout}\nstderr={stderr}")
    }
    let out = format!("stdout=\n{stdout}\nstderr=\n{stderr}");
    let out = strip_current_path(&out);
    expect_file![outfile].assert_eq(&out);
}

fn strip_current_path(s: &str) -> String {
    let mut path = Path::new(".").canonicalize().unwrap().to_str().unwrap().to_owned();
    path.push(std::path::MAIN_SEPARATOR);
    s.replace(&path, "")
}

fn testcase(name: &str) -> [String; 2] {
    let file = format!("./tests/snippets/{name}.rs");
    let outfile = format!("snapshots/{name}.txt");
    [file, outfile]
}

#[test]
fn unsafe_calls_panic_assign() {
    let [file, outfile] = &testcase("unsafe_calls_panic_assign");
    should_panic(file, outfile, Default::default());
}

#[test]
fn unsafe_calls_panic_assign_fn_ptr() {
    let [file, outfile] = &testcase("unsafe_calls_panic_assign_fn_ptr");
    should_panic(file, outfile, Default::default());
}

#[test]
fn unsafe_calls_panic_no_tag() {
    let [file, outfile] = &testcase("unsafe_calls_panic_no_tag");
    should_panic(file, outfile, Default::default());
}

#[test]
fn unsafe_calls_panic_method() {
    let [file, outfile] = &testcase("unsafe_calls_panic_method");
    should_panic(file, outfile, Default::default());
}

#[test]
fn unsafe_calls_panic_discharge_all_tagged_less() {
    let [file, outfile] = &testcase("unsafe_calls_panic_discharge_all_tagged_less");
    should_panic(file, outfile, CompilationOptions::discharges_all_properties());
}

#[test]
fn unsafe_calls_panic_discharge_all_tagged_less_fine() {
    let [file, outfile] = &testcase("unsafe_calls_panic_discharge_all_tagged_less_fine");
    // NOTE: for unset DISCHARGES_ALL_PROPERTIES,
    // it's fine if non Memo properties are discharged or not.
    fine(file, outfile, CompilationOptions::default());
}

#[test]
fn unsafe_calls_panic_discharge_all_tagged_more() {
    // FIXME: distinguish discharge and definition tags.
    // cc https://github.com/os-checker/tag-std/issues/17
    //
    // let [file, outfile] = &testcase("unsafe_calls_panic_discharge_all_tagged_more");
    // should_panic(file, outfile, CompilationOptions::discharges_all_properties());
}

fn fine(file: &str, outfile: &str, opts: CompilationOptions) {
    let (exe, output) = compile(file, opts);
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    if !output.status.success() {
        panic!("Failed to run `{exe} {file}`:\nstdout={stdout}\nstderr={stderr}")
    }
    let out = format!("stdout=\n{stdout}\nstderr=\n{stderr}");
    let out = strip_current_path(&out);
    expect_file![outfile].assert_eq(&out);
}

#[test]
fn unsafe_calls_method() {
    let [file, outfile] = &testcase("unsafe_calls_method");
    fine(file, outfile, Default::default());
}

#[test]
fn unsafe_calls_discharge_all() {
    let [file, outfile] = &testcase("unsafe_calls_discharge_all");
    fine(file, outfile, CompilationOptions::discharges_all_properties());
}

#[test]
fn unsafe_calls_with_dep() {
    let opts = compile_libunsafe_calls();

    let [file, outfile] = &testcase("unsafe_calls_with_dep");
    fine(file, outfile, opts);
}

fn compile_libunsafe_calls() -> CompilationOptions<'static> {
    static INIT: LazyLock<CompilationOptions<'static>> = LazyLock::new(|| {
        let [file, outfile] = &testcase("unsafe_calls");
        fine(
            file,
            outfile,
            CompilationOptions {
                args: &["--crate-type=lib", "-otarget/libunsafe_calls.rlib"],
                stop: false,
                ..Default::default()
            },
        );
        CompilationOptions {
            args: &["--crate-type=lib", "--extern=unsafe_calls=target/libunsafe_calls.rlib"],
            stop: false,
            ..Default::default()
        }
    });
    *INIT
}

#[test]
fn unsafe_calls_panic_with_dep() {
    let opts = compile_libunsafe_calls();

    let [file, outfile] = &testcase("unsafe_calls_panic_with_dep");
    should_panic(file, outfile, opts);
}
