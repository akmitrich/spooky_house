#![allow(unused, dead_code)]
use std::collections::{BTreeMap, btree_map::Keys, BTreeSet};

type Room = BTreeSet<String>;
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
        self.rooms.get(room_name)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Option<Room> {
        self.rooms.remove(room_name)
    }

    pub fn get_room_name_list(&self) -> Keys<String, Room> {
        self.rooms.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_room() {
        let mut h = House::default();
        assert!(h.add_room("R1").is_ok());
        assert!(h.add_room("R2").is_ok());
        assert!(h.add_room("R2").map_or(true, |_| false));
        assert!(h.get_room("R1").map_or(false, |room| room.is_empty()));
        assert!(h.get_room("No such room").is_none());
    }

    #[test]
    fn test_remove_room() {
        let mut h = House::default();
        assert!(h.add_room("R1").is_ok());
        assert!(h.add_room("R2").is_ok());
        assert!(h.remove_room("R2").map_or(false, |room| room.is_empty()));
        assert!(h.remove_room("No such room").is_none());
    }

    #[test]
    fn test_room_list() {
        let mut h = House::default();
        assert!(h.get_room_name_list().next().is_none());
        assert!(h.add_room("R1").is_ok());
        assert!(h.add_room("R2").is_ok());
        let mut room_list = h.get_room_name_list();
        assert!(room_list.next().map_or(false, |room| room.eq("R1")));
        assert!(room_list.next().map_or(false, |room| room.eq("R2")));
        assert!(room_list.next().is_none());
    }
}