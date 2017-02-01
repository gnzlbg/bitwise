extern crate rustc_version;
use rustc_version::{version, version_meta, Channel};

fn main() {
    // Assert we haven't travelled back in time
    assert!(version().major >= 1);
    
    // Set cfg flags depending on release channel
    match version_meta().channel {
        Channel::Nightly => {
            println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
        },
        _ => {}
    };
}
