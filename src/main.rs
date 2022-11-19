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

use usbguard_notify::dbus::Devices1Proxy;
use usbguard_notify::notification::NotificationApplyDevicePolicy;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    NotificationApplyDevicePolicy::new().show();
    run().await?;
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let conn = Connection::system().await?;
    let proxy = Devices1Proxy::new(&conn).await?;
    // debugging to figure out zbus
    // TODO: Find out how to extract needed information and match that information
    // proxy.apply_device_policy(todo!(), 0, false); // allow device
    let devices = proxy.list_devices("allow").await?;
    for result in devices {
        let device_id = result.0;
        let device_rule = result.1;
        println!("{}: {}", device_id, device_rule)
    }
    Ok(())
}
