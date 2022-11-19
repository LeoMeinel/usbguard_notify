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
#[dbus_proxy(interface = "org.usbguard.Devices1", assume_defaults = true)]
trait Devices1 {
    /// applyDevicePolicy method
    ///
    /// # Arguments
    ///
    /// * `id` - Device id of the device to authorize.
    /// * `target` - Device authorization target in numerical form.
    ///              - values: `0 = Allow` `1 = Block` `2 = Reject`
    /// * `permanent` - A boolean flag specifying whether an allow rule should
    ///                 be appended to the policy.
    /// * `rule_id` - If permanent was set to true, the method will return an
    ///               ID of the rule that was modified or created because of
    ///               this request.
    fn apply_device_policy(&self, id: u32, target: u8, permanent: bool) -> zbus::Result<u32>;

    /// DevicePolicyApplied signal
    ///
    /// Notify about a change of a USB device.
    /// This is a superset of DevicePolicyChanged and will always be thrown
    /// when a device is inserted, authorised, or rejected.
    ///
    /// # Arguments
    ///
    /// * `id` - Device id of the device
    /// * `target_new` - Current authorization target in numerical form.
    ///                  - values: `0 = Allow` `1 = Block` `2 = Reject`
    /// * `device_rule` - Device specific rule.
    /// * `rule_id` - A rule id of the matched rule.
    ///               - reserved values: `4294967294 (UINT32_MAX - 1)`
    /// * `attributes` - A dictionary of device attributes and their values.
    ///
    /// ## attributes
    ///
    /// * id - the USB device ID in the form VID:PID
    /// * name
    /// * serial
    /// * via-port
    /// * hash
    /// * parent-hash
    /// * with-interface
    /// * with-connect-type - either `"hardwired"`, `"hotplug"`, or `""` for unknown
    #[dbus_proxy(signal)]
    fn device_policy_applied(
        &self,
        id: u32,
        target_new: u8,
        device_rule: &str,
        rule_id: u32,
        attributes: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;

    /// DevicePolicyChanged signal
    ///
    /// Notify about a change of a USB device authorization target.
    ///
    /// # Arguments
    ///
    /// * `id` - Device id of the device.
    /// * `target_old` - Previous authorization target in numerical form.
    ///                  - values: `0 = Allow` `1 = Block` `2 = Reject`
    /// * `target_new` - Current authorization target in numerical form.
    /// * `device_rule` - Device specific rule.
    /// * `rule_id` - A rule id of the matched rule.
    ///               - reserved values: `4294967294 (UINT32_MAX - 1)`
    /// * `attributes` - A dictionary of device attributes and their values.
    #[dbus_proxy(signal)]
    fn device_policy_changed(
        &self,
        id: u32,
        target_old: u8,
        target_new: u8,
        device_rule: &str,
        rule_id: u32,
        attributes: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;
}
