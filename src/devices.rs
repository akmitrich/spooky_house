#![allow(unused, dead_code)]
use crate::house::ReportState;

#[derive(Debug, PartialEq)]
pub struct Socket {
    voltage: f64,
    current: f64,
    on: bool,
}

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    temperature: f64,
}

impl ReportState for Socket {
    fn report_state(&self) -> String {
        todo!()
    }
}

impl ReportState for Thermometer {
    fn report_state(&self) -> String {
        todo!()
    }
}

impl Socket {
    pub fn new(voltage: f64, current: f64) -> Self {
        Self {
            voltage,
            current,
            on: false,
        }
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.voltage = voltage;
    }

    pub fn set_current(&mut self, current: f64) {
        self.current = current;
    }

    pub fn get_current_power(&self) -> f64 {
        self.current * self.voltage
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn switch(&mut self, on: bool) {
        self.on = on;
    }
}

impl Thermometer {
    pub fn new(temperature: f64) -> Self {
        Self { temperature }
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature;
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}
