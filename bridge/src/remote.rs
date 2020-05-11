use crate::device::Device;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Remote<'a, T: Device + 'a> {
    fn get_device(&self) -> Rc<RefCell<&'a mut T>>;

    fn power(&self) {
        println!("Remote: power on/off");
        let device = self.get_device();
        if device.borrow().is_on() {
            device.borrow_mut().turn_off();
        } else {
            device.borrow_mut().turn_on();
        }
    }

    fn volume_up(&self) {
        println!("Remote: volume up");
        let device = self.get_device();
        let mut device_mut = device.borrow_mut();
        let volume = device_mut.get_volume();
        device_mut.set_volume(volume + 5);
    }

    fn volume_down(&self) {
        println!("Remote: volume down");
        let device = self.get_device();
        let mut device_mut = device.borrow_mut();
        let volume = device_mut.get_volume();
        device_mut.set_volume(volume - 5);
    }
}

pub struct BasicRemote<'a, T: Device> {
    device: Rc<RefCell<&'a mut T>>,
}

impl<'a, T: Device> BasicRemote<'a, T> {
    pub fn new(device: &'a mut T) -> BasicRemote<T> {
        BasicRemote {
            device: Rc::new(RefCell::new(device)),
        }
    }
}

impl<'a, T: Device> Remote<'a, T> for BasicRemote<'a, T> {
    fn get_device(&self) -> Rc<RefCell<&'a mut T>> {
        Rc::clone(&self.device)
    }
}

pub struct ProRemote<'a, T: Device> {
    device: Rc<RefCell<&'a mut T>>,
}

impl<'a, T: Device> ProRemote<'a, T> {
    pub fn new(device: &'a mut T) -> ProRemote<T> {
        ProRemote {
            device: Rc::new(RefCell::new(device)),
        }
    }

    pub fn mute(&self) {
        println!("Pro remote: muting");
        self.device.borrow_mut().set_volume(0);
    }
}

impl<'a, T: Device> Remote<'a, T> for ProRemote<'a, T> {
    fn get_device(&self) -> Rc<RefCell<&'a mut T>> {
        Rc::clone(&self.device)
    }
}
