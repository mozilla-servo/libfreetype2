/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(feature = "build-native-freetype")]
extern crate cmake;
#[cfg(feature = "build-native-freetype")]
extern crate pkg_config;

#[cfg(feature = "build-native-freetype")]
fn main() {
    use cmake::Config;
    use std::env;

    let target = env::var("TARGET").unwrap();
    if !target.contains("eabi") &&
        !target.contains("android") &&
        pkg_config::Config::new().atleast_version("18.5.12").find("freetype2").is_ok() {
        return
    }

    let mut config = Config::new("freetype2");
    if let Ok(s) = env::var("FREETYPE_CMAKE_GENERATOR") {
        config.generator(s);
    }
    let dst = config
        .define("WITH_BZip2", "OFF")
        .define("WITH_HarfBuzz", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_ZLIB", "OFF")
        .profile("Release")
        .build();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:outdir={}", out_dir);
}

#[cfg(not(feature = "build-native-freetype"))]
fn main() {}
