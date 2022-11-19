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
use usbguard_notify::notification::NotificationChangePolicy;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    NotificationChangePolicy::new().show();
    run().await?;
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let conn = Connection::system().await?;
    let device = Devices1Proxy::new(&conn).await?;
    // debugging to figure out zbus
    // TODO: Find out how to extract needed information and match that information
    // device.apply_device_policy(todo!(), 0, false); // allow device
    let id = device.receive_device_policy_changed().await?;
    println!("{:?}", id);
    Ok(())
}
