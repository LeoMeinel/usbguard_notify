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
        include!(<usbguard/DeviceManager.hpp>);
        include!(<usbguard/Rule.hpp>);
        type Device;
        pub fn applyDevicePolicy(id: u32, target: u8);
        pub fn getDevice(id: u32) -> SharedPtr<Device>;
        pub fn getDeviceList(); 
    }
}
