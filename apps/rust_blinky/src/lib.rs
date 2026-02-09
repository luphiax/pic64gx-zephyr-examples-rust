#![no_std]

use zephyr::raw;
use zephyr::device::gpio::GpioToken;

const SLEEP_TIME_MS: i32 = 1000;

#[no_mangle]
pub extern "C" fn rust_main() {
    
    let mut led0 = zephyr::devicetree::aliases::led0::get_instance()
        .expect("Missing DT alias led0");


    let mut token = unsafe { GpioToken::get_instance().expect("Failed to get GPIO token") };

    unsafe {
        led0.configure(&mut token, raw::ZR_GPIO_OUTPUT_ACTIVE);
    }

    loop {
        unsafe { led0.toggle_pin(&mut token); }
        unsafe { raw::k_msleep(SLEEP_TIME_MS); }
    }
}

