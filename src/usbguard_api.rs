/*
 * File: usbguard_api.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("usbguard/src/Library/public/usbguard/Interface.hpp");
        pub fn getParameter(name: &str) -> String;
    }
}
