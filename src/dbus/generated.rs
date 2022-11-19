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

/// Proxy object for `org.usbguard1`
#[dbus_proxy(interface = "org.usbguard1", assume_defaults = true)]
trait usbguard1 {
    /// getParameter method
    fn get_parameter(&self, name: &str) -> zbus::Result<String>;

    /// setParameter method
    fn set_parameter(&self, name: &str, value: &str) -> zbus::Result<String>;

    /// ExceptionMessage signal
    #[dbus_proxy(signal)]
    fn exception_message(&self, context: &str, object: &str, reason: &str) -> zbus::Result<()>;

    /// PropertyParameterChanged signal
    #[dbus_proxy(signal)]
    fn property_parameter_changed(
        &self,
        name: &str,
        value_old: &str,
        value_new: &str,
    ) -> zbus::Result<()>;
}

/// Proxy object for `org.usbguard.Policy1`
#[dbus_proxy(interface = "org.usbguard.Policy1", assume_defaults = true)]
trait Policy1 {
    /// appendRule method
    fn append_rule(&self, rule: &str, parent_id: u32, temporary: bool) -> zbus::Result<u32>;

    /// listRules method
    fn list_rules(&self, label: &str) -> zbus::Result<Vec<(u32, String)>>;

    /// removeRule method
    fn remove_rule(&self, id: u32) -> zbus::Result<()>;
}

/// Proxy object for `org.usbguard.Devices1`
#[dbus_proxy(interface = "org.usbguard.Devices1", assume_defaults = true)]
trait Devices1 {
    /// applyDevicePolicy method
    ///     id: Device id of the device to authorize.
    ///     target: Device authorization target in numerical form.
    ///         0 = Allow. 1 = Block. 2 = Reject.
    ///     permanent: A boolean flag specifying whether an allow rule should be appended to the policy.
    ///     rule_id: If permanent was set to true, the method will return an ID of the rule that was modified or created because of this request.
    fn apply_device_policy(&self, id: u32, target: u32, permanent: bool) -> zbus::Result<u32>;

    /// listDevices method
    fn list_devices(&self, query: &str) -> zbus::Result<Vec<(u32, String)>>;

    /// DevicePolicyApplied signal
    ///     id: Device id of the device
    ///     target_new: Current authorization target in numerical form.
    ///         0 = Allow. 1 = Block. 2 = Reject.
    ///     device_rule: Device specific rule.
    ///     rule_id: A rule id of the matched rule. Otherwise a reserved rule id value is used.
    ///         Reserved values are:
    ///             4294967294 (UINT32_MAX - 1) for an implicit rule, e.g.
    ///             ImplicitPolicyTarget or InsertedDevicePolicy.
    ///     attributes: A dictionary of device attributes and their values.
    #[dbus_proxy(signal)]
    fn device_policy_applied(
        &self,
        id: u32,
        target_new: u32,
        device_rule: &str,
        rule_id: u32,
        attributes: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;

    /// DevicePolicyChanged signal
    ///     id: Device id of the device
    ///     target_old: Previous authorization target in numerical form.
    ///         0 = Allow. 1 = Block. 2 = Reject.
    ///     target_new: Current authorization target in numerical form.
    ///     device_rule: Device specific rule.
    ///     rule_id: A rule id of the matched rule. Otherwise a reserved rule id value is used.
    ///         Reserved values are:
    ///             4294967294 (UINT32_MAX - 1) for an implicit rule, e.g.
    ///             ImplicitPolicyTarget or InsertedDevicePolicy.
    ///     attributes: A dictionary of device attributes and their values.
    #[dbus_proxy(signal)]
    fn device_policy_changed(
        &self,
        id: u32,
        target_old: u32,
        target_new: u32,
        device_rule: &str,
        rule_id: u32,
        attributes: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;

    /// DevicePresenceChanged signal
    #[dbus_proxy(signal)]
    fn device_presence_changed(
        &self,
        id: u32,
        event: u32,
        target: u32,
        device_rule: &str,
        attributes: std::collections::HashMap<&str, &str>,
    ) -> zbus::Result<()>;
}
