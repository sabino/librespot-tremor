extern crate pkg_config;
extern crate cc;

use std::path::PathBuf;

fn main() {
    match pkg_config::probe_library("vorbisidec") {
        Ok(_) => return,
        Err(..) => {}
    };

    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let tremor_include = root.join("tremor/include");
    let ogg_include = PathBuf::from(std::env::var("DEP_OGG_INCLUDE").unwrap());

    println!("cargo:include={}", tremor_include.display());

    cc::Build::new()
                .file("tremor/mdct.c")
                .file("tremor/block.c")
                .file("tremor/window.c")
                .file("tremor/synthesis.c")
                .file("tremor/info.c")
                .file("tremor/floor1.c")
                .file("tremor/floor0.c")
                .file("tremor/vorbisfile.c")
                .file("tremor/res012.c")
                .file("tremor/mapping0.c")
                .file("tremor/registry.c")
                .file("tremor/codebook.c")
                .file("tremor/sharedbook.c")
                .include(&tremor_include)
                .include(&ogg_include)
                .compile("libtremor.a");
}
