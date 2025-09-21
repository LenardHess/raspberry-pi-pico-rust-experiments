//! Basic TMCL implementation
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use defmt::{info};

use tmcl::{TMCLRequest, TMCLStack, TMCL_PACKET_SIZE};
use embassy_rp::usb::{Driver, Instance};
use embassy_usb::class::cdc_acm::{BufferedReceiver, Sender};
use embassy_usb::driver::EndpointError;
use embedded_io_async::{Write, Read};

// ToDo: Duplicated from main.rs
const USB_CDC_PACKET_SIZE: u16 = 64;

const TMCL_HOST_ADDRESS: u8 = 0xFF;

#[allow(dead_code)]
#[allow(unreachable_code)]
pub async fn tmcl_usbhandler<'d, T: Instance + 'd>(
    usb_rx: &mut BufferedReceiver<'d, Driver<'d, T>>,
    usb_tx: &mut Sender<'d, Driver<'d, T>>
) -> Result<(), EndpointError>
{
    info!("Wait for USB connection");
    usb_rx.wait_connection().await;
    info!("Connected");

    // Create the TMCL stack
    let tmcl = TMCLStack {
        host_address: TMCL_HOST_ADDRESS,
        device_address: 1,
        module_id: 255,
    };

    // Start the receive loop
    let mut buf = [0; TMCL_PACKET_SIZE as usize];
    loop {
        let bytes = usb_rx.read(&mut buf).await?;

        // ToDo: In the proper protocol we'd need to time out, not instantly drop. Since this is USB this will work out for now
        if bytes < 9 {
            continue;
        }

        info!("Received USB TMCL packet");
        let request = TMCLRequest::new(&buf);
        let reply = tmcl.process(&request);

        // Send the reply
        if let Some(reply) = reply {
            usb_tx.write(&reply.serialize()).await?;
        } else {
            info!("USB TMCL packet not addressed to us");
        }
    }
}

#[unsafe(no_mangle)] // Don't mangle the function name to allow tmcl crate to extern-link to it // ToDo: Can you do this without no_mangle?
fn tmcl_device_specific_info(index : u8) -> Option<u32> {
    match index {
        200 => Some(0xdeadbeef),
        _ => None
    }
}
