//! Basic TMCL implementation
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use defmt::{info};

use embassy_rp::usb::{Driver, Instance};
use embassy_usb::class::cdc_acm::{BufferedReceiver, Sender};
use embassy_usb::driver::EndpointError;
use embedded_io_async::{Write, Read};

// ToDo: Duplicated from main.rs
const USB_CDC_PACKET_SIZE: u16 = 64;
const TMCL_PACKET_SIZE: usize = 9;
const TMCL_HOST_ADDRESS: u8 = 0xFF;

#[allow(dead_code)]
#[allow(unreachable_code)]
struct TMCLRequest {
    device_addr : u8,
    opcode      : u8,
    index       : u8,
    motor       : u8,
    value       : u32,
    checksum    : u8
}

struct TMCLReply {
    host_addr : u8,
    device_addr : u8,
    opcode : u8,
    status : TMCLReplyStatus,
    value : u32,
}

impl TMCLRequest  {
    fn new(bytes : &[u8; TMCL_PACKET_SIZE]) -> TMCLRequest {
        TMCLRequest {
            device_addr: bytes[0],
            opcode: bytes[1],
            index: bytes[2],
            motor: bytes[3],
            value: u32::from_be_bytes(bytes[4..=7].try_into().unwrap()),
            checksum: bytes[8]
        }
    }

    fn is_checksum_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn calculate_checksum(&self) -> u8 {
        let mut checksum : u8 = 0;
        checksum = checksum.wrapping_add(self.device_addr);
        checksum = checksum.wrapping_add(self.opcode);
        checksum = checksum.wrapping_add(self.index);
        checksum = checksum.wrapping_add(self.motor);
        checksum = checksum.wrapping_add((self.value >> 24) as u8);
        checksum = checksum.wrapping_add((self.value >> 16) as u8);
        checksum = checksum.wrapping_add((self.value >>  8) as u8);
        checksum = checksum.wrapping_add((self.value      ) as u8);

        checksum
    }
}

#[derive(Clone, Copy)]
enum TMCLReplyStatus {
    ChecksumError = 1,
    Ok = 100,
}

impl TMCLReply {
    fn new(host_addr: u8, request: &TMCLRequest) -> TMCLReply {
        TMCLReply {
            host_addr: host_addr,
            device_addr: request.device_addr,
            opcode: request.opcode,
            status: TMCLReplyStatus::Ok,
            value: request.value
        }
    }

    fn calculate_checksum(&self) -> u8 {
        let mut checksum : u8 = 0;
        checksum = checksum.wrapping_add(self.host_addr);
        checksum = checksum.wrapping_add(self.device_addr);
        checksum = checksum.wrapping_add(self.opcode);
        checksum = checksum.wrapping_add(self.status as u8);
        checksum = checksum.wrapping_add((self.value >> 24) as u8);
        checksum = checksum.wrapping_add((self.value >> 16) as u8);
        checksum = checksum.wrapping_add((self.value >>  8) as u8);
        checksum = checksum.wrapping_add((self.value      ) as u8);

        checksum
    }

    fn serialize(&self) -> [u8; TMCL_PACKET_SIZE] {
        [
            self.host_addr,
            self.device_addr,
            self.opcode,
            self.status as u8,
            (self.value >> 24) as u8,
            (self.value >> 16) as u8,
            (self.value >>  8) as u8,
            (self.value      ) as u8,
            self.calculate_checksum()
        ]
    }
}


pub async fn tmcl_usbhandler<'d, T: Instance + 'd>(
    usb_rx: &mut BufferedReceiver<'d, Driver<'d, T>>,
    usb_tx: &mut Sender<'d, Driver<'d, T>>
) -> Result<(), EndpointError>
{
    info!("Wait for USB connection");
    usb_rx.wait_connection().await;
    info!("Connected");

    // Start the receive loop
    let mut buf = [0; TMCL_PACKET_SIZE as usize];
    //let mut reply_buf = [65; USB_CDC_PACKET_SIZE as usize];
    loop {
        let bytes = usb_rx.read(&mut buf).await?;

        // ToDo: In the proper protocol we'd need to time out, not instantly drop. Since this is USB this will work out for now
        if bytes < 9 {
            continue;
        }

        info!("Received USB TMCL packet");
        let request = TMCLRequest::new(&buf);
        let mut reply = TMCLReply::new(TMCL_HOST_ADDRESS, &request);

        if !request.is_checksum_valid()
        {
            info!("Invalid TMCL checksum");
            reply.status = TMCLReplyStatus::ChecksumError;
            usb_tx.write(&reply.serialize()).await?;
            continue;
        }

        info!("Handling TMCL request");
        // Todo: Handle request

        // Send the reply
        usb_tx.write(&reply.serialize()).await?;
    }
}
