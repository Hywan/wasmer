//! A kind of meta-build.rs that can be configured to do different things.
//!
//! Please try to keep this file as clean as possible.

use generate_emscripten_tests;
use generate_wasi_tests;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use test_generator::{
    build_ignores_from_textfile, emscripten_processor, test_directory, test_directory_module,
    wasi_processor, wast_processor, with_backends, with_test_module, Testsuite,
};

static EMTESTS_ENV_VAR: &str = "WASM_EMSCRIPTEN_GENERATE_EMTESTS";
static WASITESTS_ENV_VAR: &str = "WASM_WASI_GENERATE_WASITESTS";
static WASITESTS_SET_UP_TOOLCHAIN: &str = "WASM_WASI_SET_UP_TOOLCHAIN";
static WASITESTS_GENERATE_ALL: &str = "WASI_TEST_GENERATE_ALL";

fn is_truthy_env(name: &str) -> bool {
    env::var(name).map(|n| n == "1").unwrap_or_default()
}

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=test/ignores.txt");
    println!("cargo:rerun-if-env-changed={}", EMTESTS_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", WASITESTS_ENV_VAR);
    println!("cargo:rerun-if-env-changed={}", WASITESTS_SET_UP_TOOLCHAIN);
    println!("cargo:rerun-if-env-changed={}", WASITESTS_GENERATE_ALL);

    let wasi_versions = if is_truthy_env(WASITESTS_GENERATE_ALL) {
        generate_wasi_tests::ALL_WASI_VERSIONS
    } else {
        generate_wasi_tests::LATEST_WASI_VERSION
    };

    // Install the Rust WASI toolchains for each of the versions
    if is_truthy_env(WASITESTS_SET_UP_TOOLCHAIN) {
        generate_wasi_tests::install_toolchains(wasi_versions);
    }

    // Generate the WASI Wasm files
    if is_truthy_env(WASITESTS_ENV_VAR) {
        generate_wasi_tests::build(wasi_versions);
    }

    // Generate Emscripten Wasm files
    if is_truthy_env(EMTESTS_ENV_VAR) {
        generate_emscripten_tests::build();
    }

    let out_dir = PathBuf::from(
        env::var_os("OUT_DIR").expect("The OUT_DIR environment variable must be set"),
    );
    let ignores = build_ignores_from_textfile("tests/ignores.txt".into())?;

    // Spectests test generation
    let mut spectests = Testsuite {
        buffer: String::new(),
        path: vec![],
        ignores: ignores.clone(),
    };
    let backends = vec!["singlepass", "cranelift", "llvm"];
    with_backends(&mut spectests, &backends, |mut spectests| {
        with_test_module(&mut spectests, "spec", |spectests| {
            let _spec_tests = test_directory(spectests, "tests/spectests", wast_processor)?;
            Ok(())
        })?;
        Ok(())
    })?;

    // Emscripten tests generation
    let mut emtests = Testsuite {
        buffer: String::new(),
        path: vec![],
        ignores: ignores.clone(),
    };
    with_backends(&mut emtests, &backends, |mut emtests| {
        with_test_module(&mut emtests, "emscripten", |emtests| {
            let _emscripten_tests = test_directory(
                emtests,
                "tests/emscripten_resources/emtests",
                emscripten_processor,
            )?;
            Ok(())
        })?;
        Ok(())
    })?;

    // WASI tests generation
    let mut wasitests = Testsuite {
        buffer: String::new(),
        path: vec![],
        ignores: ignores.clone(),
    };
    with_backends(&mut wasitests, &backends, |mut wasitests| {
        with_test_module(&mut wasitests, "wasi", |wasitests| {
            test_directory_module(
                wasitests,
                "tests/wasi_test_resources/unstable",
                wasi_processor,
            )?;
            test_directory_module(
                wasitests,
                "tests/wasi_test_resources/snapshot1",
                wasi_processor,
            )?;
            Ok(())
        })?;
        Ok(())
    })?;

    let spectests_output = out_dir.join("generated_spectests.rs");
    fs::write(&spectests_output, spectests.buffer)?;

    let emtests_output = out_dir.join("generated_emtests.rs");
    fs::write(&emtests_output, emtests.buffer)?;

    let wasitests_output = out_dir.join("generated_wasitests.rs");
    fs::write(&wasitests_output, wasitests.buffer)?;

    // Write out our auto-generated tests and opportunistically format them with
    // `rustfmt` if it's installed.
    // Note: We need drop because we don't want to run `unwrap` or `expect` as
    // the command might fail, but we don't care about it's result.
    drop(Command::new("rustfmt").arg(&spectests_output).status());
    drop(Command::new("rustfmt").arg(&emtests_output).status());
    drop(Command::new("rustfmt").arg(&wasitests_output).status());

    Ok(())
}
