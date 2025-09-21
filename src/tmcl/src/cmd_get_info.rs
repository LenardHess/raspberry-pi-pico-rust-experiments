
use crate::{TMCLRequest, TMCLReply, TMCLStack, TMCLReplyStatus};

pub(crate) fn cmd_get_info(tmcl_stack : &TMCLStack, request : &TMCLRequest, reply : &mut TMCLReply) -> () {
    let info = match request.index {
        // Module ID
        0 => Some(tmcl_stack.module_id as u32),

        // Device-specific area
        200..=240 =>  device_specific_info(request.index),

        // Unknown value
        _ => None
    };

    match info {
        Some(v) => reply.value=v,
        None => reply.status = TMCLReplyStatus::InvalidType
    }
}

fn device_specific_info(index : u8) -> Option<u32> {
    if cfg!(feature = "get_info_device_specific")
    {
        #[allow(unused_unsafe)]
        return unsafe { tmcl_device_specific_info(index) };
    } else {
        None
    }
}

#[cfg(feature = "get_info_device_specific")]
unsafe extern "Rust" {
    fn tmcl_device_specific_info(index : u8) -> Option<u32>;
}

#[cfg(not(feature = "get_info_device_specific"))]
#[unsafe(no_mangle)]
fn tmcl_device_specific_info(index : u8) -> Option<u32> { None }