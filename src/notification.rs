/*
 * File: notification.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use notify_rust::{Notification, NotificationHandle};

pub struct NotificationApplyDevicePolicy {
    pub notification: NotificationHandle,
    pub device_id: u32,
}

impl NotificationApplyDevicePolicy {
    pub fn new() -> NotificationApplyDevicePolicy {
        NotificationApplyDevicePolicy {
            notification: Notification::new()
                .summary("USBGuard")
                // dbus -> get device and insert into body
                .body("todo!()")
                .icon("usbguard-icon")
                .action("allow", "Allow")
                .action("deny", "Deny")
                .show()
                .unwrap(),
            device_id: 0,
        }
    }
    pub fn show(self) {
        self.notification.wait_for_action(|action| {
            if action == "allow" {
                // dbus -> allow device
                todo!();
            }
        });
    }
}

impl Default for NotificationApplyDevicePolicy {
    fn default() -> Self {
        NotificationApplyDevicePolicy::new()
    }
}
