pub trait Device {
    fn is_on(&self) -> bool;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn get_volume(&self) -> u32;
    fn set_volume(&mut self, value: u32);
    fn print_status(&self);
}

pub struct TV {
    on: bool,
    volume: u32,
}

impl TV {
    pub fn new() -> TV {
        TV {
            on: false,
            volume: 10,
        }
    }
}

impl Device for TV {
    fn is_on(&self) -> bool {
        self.on
    }

    fn turn_on(&mut self) {
        self.on = true;
    }

    fn turn_off(&mut self) {
        self.on = false;
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }

    fn set_volume(&mut self, value: u32) {
        match value {
            x if x > 100  => self.volume = 100,
            _ => self.volume = value
        }
    }

    fn print_status(&self) {
        println!(
            "TV Device, {}, volume: {}",
            if self.on { "turned on" } else { "turned off" },
            self.volume
        );
    }
}

pub struct Radio {
    on: bool,
    volume: u32,
}

impl Radio {
    pub fn new() -> Radio {
        Radio {
            on: false,
            volume: 20,
        }
    }
}

impl Device for Radio {
    fn is_on(&self) -> bool {
        self.on
    }

    fn turn_on(&mut self) {
        self.on = true;
    }

    fn turn_off(&mut self) {
        self.on = false;
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }

    fn set_volume(&mut self, value: u32) {
        match value {
            x if x > 100  => self.volume = 100,
            _ => self.volume = value
        }
    }

    fn print_status(&self) {
        println!(
            "Radio Device, {}, volume: {}",
            if self.on { "turned on" } else { "turned off" },
            self.volume
        );
    }
}
