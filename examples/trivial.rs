use spooky_house::house;
fn main() {
    let mut h = house::House::default();
    h.add_room("R1").unwrap();
    let r = h.add_room("R2");
    if let Err(_e) = r {
        unreachable!();
    }
    if let Err(_e) = h.add_room("Kitchen") {
        unreachable!();
    }
    if let Err(e) = h.add_room("R2") {
        println!("As expected {:?}", e);
    }
    println!("Start with house: {:?}", h);
    let room_list_iter = h.get_room_name_list();
    println!("List of rooms in the house: {:?}", room_list_iter.collect::<Vec<_>>());
    println!("House still is {:?}", h);
}