use std::{fs, io::prelude::*, str::FromStr, error::Error};

#[derive(PartialEq)]
pub enum Urgency {
    Low,
    Normal,
    Full
}

#[derive(PartialEq)]
pub enum ChargingState {
    Discharging,
    Charging
}

impl Urgency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Urgency::Low => "critical",
            Urgency::Normal => "idkguys",
            Urgency::Full => "normal",
        }
    }
}

impl ChargingState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChargingState::Discharging => "discharging",
            ChargingState::Charging => "charging"
        }
    }
}

impl FromStr for ChargingState {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "charging" => Ok(ChargingState::Charging),
            "discharging" => Ok(ChargingState::Discharging),
            _ => Err("Unknown Charging State".into())
        }
    }
}

const BATTERY_PATH: &'static str = "/sys/class/power_supply/BAT0/";

pub fn get_battery_percentage() -> Result<i32, Box<dyn Error>> {
    // Full Cap
    let mut full_cap_file = fs::File::open(BATTERY_PATH.to_string() + "energy_full")?;
    let mut full_cap = String::new();
    full_cap_file.read_to_string(&mut full_cap)?;

    let full_cap : i32 = full_cap.trim().parse()?;

    // Current Cap

    let mut curr_cap_file = fs::File::open(BATTERY_PATH.to_string() + "energy_now")?;
    let mut curr_cap = String::new();
    curr_cap_file.read_to_string(&mut curr_cap)?;

    let curr_cap : i32 = curr_cap.trim().parse()?;

    let percentage = (curr_cap as i64) * 100 / (full_cap as i64);

    Ok(percentage as i32)
}

pub fn get_battery_state() -> Result<ChargingState, Box<dyn Error>> {
    let mut status_file = fs::File::open(BATTERY_PATH.to_string() + "status")?;
    let mut status = String::new();
    status_file.read_to_string(&mut status)?;

    let charge_state = ChargingState::from_str(status.trim().to_lowercase().as_str())?;

    Ok(charge_state)
}
