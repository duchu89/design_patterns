mod device;
mod remote;

use device::{Device, TV, Radio};
use remote::{BasicRemote, ProRemote};

pub fn test() {
    test_device(&mut TV::new());
    test_device(&mut Radio::new());
}

pub fn test_device<'a>(test_device: &mut dyn Device) {
    let basic_remote = BasicRemote::new(test_device);

    basic_remote.power();
    basic_remote.volume_up();
    basic_remote.volume_up();
    basic_remote.volume_down();
    test_device.print_status();

    let pro_remote = ProRemote::new(test_device);

    pro_remote.mute();
    test_device.print_status();
}