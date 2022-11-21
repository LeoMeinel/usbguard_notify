/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::error::Error;

use usbguard_notify::notification::NotificationApplyDevicePolicy;
use usbguard_notify::usbguard_api::ffi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    NotificationApplyDevicePolicy::new().show();
    run().await?;
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    // TODO: Find out how to extract needed information and match that information
    //       Get device_name, device_id, target
    //       apply_device_policy via usbguard_api
    // proxy.apply_device_policy(todo!(), 0, false); // allow device
    ffi::getParameter("allow");
    Ok(())
}
