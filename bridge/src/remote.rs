use crate::device::Device;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BasicRemote<'a> {
    device: Rc<RefCell<&'a mut dyn Device>>,
}

impl<'a> BasicRemote<'a> {
    pub fn new(device: &'a mut dyn Device) -> BasicRemote {
        BasicRemote {
            device: Rc::new(RefCell::new(device)),
        }
    }

    pub fn power(&self) {
        println!("Remote: power on/off");
        if self.device.borrow().is_on() {
            self.device.borrow_mut().turn_off();
        } else {
            self.device.borrow_mut().turn_on();
        }
    }

    pub fn volume_up(&self) {
        println!("Remote: volume up");
        let mut device_mut = self.device.borrow_mut();
        let volume = device_mut.get_volume();
        device_mut.set_volume(volume + 5);
    }

    pub fn volume_down(&self) {
        println!("Remote: volume down");
        let mut device_mut = self.device.borrow_mut();
        let volume = device_mut.get_volume();
        device_mut.set_volume(volume - 5);
    }
}

pub struct ProRemote<'a> {
    device: Rc<RefCell<&'a mut dyn Device>>,
}

impl <'a>ProRemote<'a> {
    pub fn new(device: &'a mut dyn Device) -> ProRemote {
        ProRemote {
            device: Rc::new(RefCell::new(device)),
        }
    }

    pub fn mute(&self) {
        println!("Remote: volume down");
        self.device
            .borrow_mut()
            .set_volume(0);
    }
}