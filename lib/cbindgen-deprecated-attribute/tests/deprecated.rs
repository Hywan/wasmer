use cbindgen::{Builder, Language};
use cbindgen_deprecated_attribute::cbindgen_deprecated;

/// Foobar!
#[cbindgen_deprecated(since = "4.2", note = "Hel\"lo")]
#[doc("yolo")]
#[no_mangle]
pub extern "C" fn foo() -> i32 {
    42
}

#[test]
fn test() {
    let mut output = Vec::new();

    Builder::new()
        .with_src("tests/deprecated.rs")
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate C bindings")
        .write(&mut output);

    dbg!(unsafe { String::from_utf8_unchecked(output) });
}
