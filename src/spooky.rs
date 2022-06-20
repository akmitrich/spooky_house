#![allow(unused, dead_code)]
use std::collections::BTreeMap;
use crate::devices::{Thermometer, Socket};
use crate::house::{ReportState, GetDevice};

type DeviceKey = (String, String);

#[derive(Debug, Default)]
pub struct Spooky {
    thermometers: BTreeMap<DeviceKey, Thermometer>,
    sockets: BTreeMap<DeviceKey, Socket>,
}

impl GetDevice for Spooky {
    fn get_device(&self, room: &str, device: &str) -> Option<&dyn ReportState> {
        let key: (String, String) = (room.into(), device.into());
        if let Some(thermometer) = self.thermometers.get(&key) {
            Some(thermometer as &dyn ReportState)
        } else if let Some(socket) = self.sockets.get(&key) {
            Some(socket as &dyn ReportState)
        } else {
            None
        }
    }
}

impl Spooky {
    pub fn add_thermometer(&mut self, room_name: &str, device_name: &str, thermometer: Thermometer) {
        let key: (String, String) = (room_name.into(), device_name.into());
        self.thermometers.insert(key, thermometer);
    }
 
    pub fn add_socket(&mut self, room_name: &str, device_name: &str, socket: Socket) {
        let key: (String, String) = (room_name.into(), device_name.into());
        self.sockets.insert(key, socket);
    }
}