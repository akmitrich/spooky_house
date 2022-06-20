use spooky_house::house::ReportState;
use spooky_house::spooky;
use spooky_house::house;
use spooky_house::devices;

fn main() {
    let mut spooky = spooky::Spooky::default();
    let mut house = house::House::default();
    house.add_room("R1").unwrap();
    house.add_room("Kitchen").unwrap();
    house.add_device("R1", "Socket1").unwrap();
    spooky.add_socket("R1", "Socket1", devices::Socket::new(220_f64, 0_f64, false));
    house.add_device("R1", "Thermo").unwrap();
    spooky.add_thermometer("R1", "Thermo", devices::Thermometer::new(23.3));
    house.add_device("Kitchen", "Thermo").unwrap();
    spooky.add_thermometer("Kitchen", "Thermo", devices::Thermometer::new(120_f64));
    println!("{:?}", spooky);
    println!("Report:");
    println!("{}", house.generate_report(&spooky));
}