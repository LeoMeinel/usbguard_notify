/*
 * File: generated.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use zbus::dbus_proxy;

/// Proxy object for `org.usbguard.Devices1`
#[dbus_proxy(
    interface = "org.usbguard.Devices1",
    default_service = "org.usbguard1",
    default_path = "/org/usbguard1/Devices"
)]
trait Devices1 {
    /// Apply an authorization target to a device.
    ///
    /// # Arguments
    ///
    /// * `id` - Device id of the device to authorize.
    /// * `target` - Device authorization target in numerical form.
    ///              - values: `0 = Allow` `1 = Block` `2 = Reject`
    /// * `permanent` - A boolean flag specifying whether an allow rule should
    ///                 be appended to the policy.
    ///
    /// # Returns
    ///
    /// * `rule_id` - If permanent was set to true, the method will return an
    ///               ID of the rule that was modified or created because of
    ///               this request.
    fn apply_device_policy(&self, id: u32, target: u8, permanent: bool) -> zbus::Result<u32>;

    /// List devices that match the specified query. The query uses the rule
    /// language syntax and the devices are returned as device specific rules.
    /// The target in each rule represents the current authorization state of
    /// the device. Order of the returned devices is not defined and should not
    /// be relied upon.
    ///
    /// # Arguments
    ///
    /// * `query` - A query, in the rule language syntax, for matching devices.
    ///
    /// ## Example queries
    /// * 'match' - Matches any device.
    /// * 'allow' - Matches only authorized devices.
    /// * 'block' - Matches only unauthorized devices.
    /// * 'match with-interface one-of { 03:00:01 03:01:01 }' - Matches any
    ///                                                         device with
    ///                                                         a
    ///                                                         HID/Keyboard
    ///                                                         interface.
    ///
    /// # Returns
    ///
    /// * `devices` - An array of (device_id, device_rule) tuples that match the
    ///               query.
    fn list_devices(&self, query: &str) -> zbus::Result<Vec<(u32, String)>>;
}
