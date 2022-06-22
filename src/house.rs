#![allow(unused, dead_code)]
use std::fmt;
use std::collections::{btree_map::Keys, btree_set::Iter, BTreeMap, BTreeSet};

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

pub trait ReportState {
    fn report_state(&self) -> String;
}

pub trait GetDevice {
    fn get_device(&self, room: &str, device: &str) -> Option<&dyn ReportState>;
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
        match self.rooms.get_mut(room_name) {
            Some(room) => {
                if room.insert(unique_name.into()) {
                    Ok(())
                } else {
                    Err(HouseError::DeviceNameClash)
                }
            }
            None => Err(HouseError::NoRoomName),
        }
    }

    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> bool {
        match self.rooms.get_mut(room_name) {
            Some(room) => room.remove(device_name.into()),
            None => false,
        }
    }

    pub fn get_device_list_in_room(&self, room_name: &str) -> Option<Iter<String>> {
        self.rooms.get(room_name).map(|room| room.iter())
    }

    pub fn generate_report(&self, info_provider: &impl GetDevice) -> String {
        let mut lines = vec![];
        lines.push(String::from("General report on House:"));
        for room in self.get_room_name_list() {
            lines.push(format!("\t Room: {}", room));
            for device in self.get_device_list_in_room(room).unwrap() {
                match info_provider.get_device(room, device) {
                    Some(report_state) => 
                        lines.push(format!("\t\t {} => {}", device, report_state.report_state())),
                    None => lines.push(format!("Error! Cannot find info about {}-{}", room, device)),
                }
            }
        }
        lines.join("\n")
    }
}

impl fmt::Display for HouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HouseError::RoomNameClash => write!(f, "Room Name Clash"),
            HouseError::DeviceNameClash => write!(f, "Device Name Clash"),
            HouseError::NoRoomName => write!(f, "No such Room"),
            HouseError::NoDeviceName => write!(f, "No such Device"),
        }
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
        let mut device_list = h.get_device_list_in_room("R2").unwrap();
        assert!(device_list
            .next()
            .map_or(false, |d| d.starts_with("Socket")));
        assert!(device_list
            .next()
            .map_or(false, |d| d.starts_with("Thermo")));
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
        assert!(h.remove_device("R1", "Socket2"));
        assert!(!h.remove_device("R1", "no such device"));
        assert!(!h.remove_device("No such room", "Irrelevant"));
        let mut device_list = h.get_device_list_in_room("R1").unwrap();
        assert!(device_list
            .next()
            .map_or(false, |device| device.eq("Socket1")));
        assert!(device_list.next().is_none());
    }
}
