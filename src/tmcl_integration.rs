#[used]
#[no_mangle]
static FIRMWARE_VERSION_MAJOR : u16 = 0;
#[used]
#[no_mangle]
static FIRMWARE_VERSION_MINOR : u16 = 1;

use tmcl_macros::{axis_parameter};

#[no_mangle]
#[axis_parameter(index = 42)]
pub fn ap_bar() -> bool {
    true
}
