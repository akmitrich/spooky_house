#![allow(unused, dead_code)]
use std::collections::{BTreeMap, btree_map::Keys};

type Room = BTreeMap<String, String>;
#[derive(Debug)]
pub enum HouseError {
    RoomNameClash,
    DeviceNameClash,
    NoRoomName,
    NoDeviceName,
}
pub type HouseResult = Result<(), HouseError>;

#[derive(Debug, Default)]
pub struct House {
    rooms: BTreeMap<String, Room>,
}

trait DeviceInfo {
    fn device_info(&self) -> String;
}

trait GetDevice {
    fn get_device(&self, room: &str, device: &str) -> &dyn DeviceInfo;
}

impl House {
    pub fn add_room(&mut self, unique_name: &str) -> HouseResult {
        if self.rooms.contains_key(unique_name) {
            return Err(HouseError::RoomNameClash);
        }
        assert_eq!(None, self.rooms.insert(unique_name.into(), Room::new()));
        Ok(())
    }

    pub fn get_room(&self, room_name: &str) -> Option<&Room> {
        todo!()
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room> {
        todo!()
    }

    pub fn get_room_name_list(&self) -> Keys<String, Room> {
        todo!()
    }
}