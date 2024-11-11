// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

extern crate upower_dbus;

use upower_dbus::{UPowerProxy, DeviceProxy, BatteryState};

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let upower = UPowerProxy::new(&connection).await?;

        for bat in upower.enumerate_devices().await? {
            let device = DeviceProxy::new(&connection, bat).await?;
            println!("Device: {:?}", device.native_path().await?);
            println!("\tType: {:?}", device.type_().await?);
            println!("\tPercentage: {:?}", device.percentage().await?);
            println!("\tBatteryLevel: {:?}", device.battery_level().await?);
            println!("\tWarningLevel: {:?}", device.warning_level().await?);
            println!("\tIconName: {:?}", device.icon_name().await?);
            println!("\tIsPresent: {:?}", device.is_present().await?);
            println!("\tOnline: {:?}", device.online().await?);
            println!("\tState: {:?}", device.state().await?);
            if let BatteryState::Charging = device.state().await? {
                println!("\tTimeToFull: {:?}", device.time_to_full().await?);
            }
            if let BatteryState::Discharging = device.state().await? {
                println!("\tTimeToEmpty: {:?}", device.time_to_empty().await?);
            }
        };

        Ok(())
    })
}
