/*
 * File: build.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[allow(unused_must_use)]
fn main() {
    println!("cargo:rerun-if-changed=src/usbguard_api.rs");
    println!("cargo:rerun-if-changed=include/usbguard/src/Library/public/usbguard/Interface.hpp");
    // FIXME: Find out how to link against ./include/usbguard/src/Library/public/usbguard/Interface.hpp
    println!("cargo:rustc-link-lib=usbguard");
    // FIXME: Find out how to link against /usr/lib/usbguard implicitly
    cxx_build::bridge("src/usbguard_api.rs")
        .cpp(true)
        .link_lib_modifier("usbguard");
}
