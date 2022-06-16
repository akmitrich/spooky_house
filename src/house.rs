#![allow(unused, dead_code)]
use std::collections::{BTreeMap, btree_map::{Keys}, BTreeSet, btree_set::Iter};

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

trait ReportState {
    fn report_state(&self) -> String;
}

trait GetDevice {
    fn get_device(&self, room: &str, device: &str) -> &dyn ReportState;
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

    pub fn add_device(&mut self, room_name: &str, unique_name: &str) -> HouseResult {
        todo!()
    }

    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Option<String> {
        todo!()
    }

    pub fn get_device_list_in_room(&self, room_name: &str) -> Iter<String> {
        todo!()
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

    #[test]
    fn test_add_device() {
        let mut h = House::default();
        assert!(h.add_room("R1").is_ok());
        assert!(h.add_room("R2").is_ok());
        assert!(h.add_device("R1", "Socket1").is_ok());
        assert!(h.add_device("R1", "Socket2").is_ok());
        assert!(h.add_device("R2", "Thermo").is_ok());
        assert!(h.add_device("R2", "Socket1").is_ok());
        let result = h.add_device("R1", "Socket2");
        if let Err(e) = result {
            if let HouseError::DeviceNameClash = e {
                assert!(true);
            } else {
                panic!("Error should be DeviceClashError.");
            }
        } else {
            panic!("There is the same device in the room but result is Ok.");
        }
        if let Err(e) = h.add_device("No such room", "Device") {
            if let HouseError::NoRoomName = e {
                assert!(true);
            } else {
                panic!("Error should be NoRoomName");
            }
        } else {
            panic!("There is no room with this name.")
        }
        let mut device_list = h.get_device_list_in_room("R2");
        assert!(device_list.next().map_or(false, |d| d.starts_with("Socket")));
        assert!(device_list.next().map_or(false, |d| d.starts_with("Thermo")));
        assert!(device_list.next().is_none());
    }

    #[test]
    fn test_remove_device() {
        let mut h = House::default();
        assert!(h.add_room("R1").is_ok());
        assert!(h.add_room("R2").is_ok());
        assert!(h.add_device("R1", "Socket1").is_ok());
        assert!(h.add_device("R1", "Socket2").is_ok());
        assert!(h.add_device("R2", "Thermo").is_ok());
        assert!(h.remove_device("R1", "Socket2").map_or(false, |device| device.eq("Socket2")));
        assert!(h.remove_device("R1", "no such device").is_none());
        assert!(h.remove_device("No such room", "Irrelevant").is_none());
        let mut device_list = h.get_device_list_in_room("R1");
        assert!(device_list.next().map_or(false, |device| device.eq("Socket1")));
        assert!(device_list.next().is_none());
    }
}