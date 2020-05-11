mod device;
mod remote;

use device::{Device, TV, Radio};
use remote::{Remote, BasicRemote, ProRemote};

pub fn test() {
    test_device(&mut TV::new());
    test_device(&mut Radio::new());
}

pub fn test_device<'a, T: Device>(test_device: &mut T) {
    let basic_remote = BasicRemote::new(test_device);

    basic_remote.power();
    basic_remote.volume_up();
    basic_remote.volume_up();
    basic_remote.volume_down();
    test_device.print_status();

    let pro_remote = ProRemote::new(test_device);

    pro_remote.power();
    pro_remote.volume_up();
    pro_remote.mute();
    test_device.print_status();
}