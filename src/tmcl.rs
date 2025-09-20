//! Basic TMCL implementation
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[allow(unused_imports)]
use defmt::{debug, info, warn, error};

pub const TMCL_PACKET_SIZE: usize = 9;

#[allow(dead_code)]
#[allow(unreachable_code)]
pub struct TMCLRequest {
    pub device_addr : u8,
    pub opcode      : u8,
    pub index       : u8,
    pub motor       : u8,
    pub value       : u32,
    pub checksum    : u8
}

pub struct TMCLReply {
    pub host_addr : u8,
    pub device_addr : u8,
    pub opcode : u8,
    pub status : TMCLReplyStatus,
    pub value : u32,
}

impl TMCLRequest  {
    pub fn new(bytes : &[u8; TMCL_PACKET_SIZE]) -> TMCLRequest {
        TMCLRequest {
            device_addr: bytes[0],
            opcode: bytes[1],
            index: bytes[2],
            motor: bytes[3],
            value: u32::from_be_bytes(bytes[4..=7].try_into().unwrap()),
            checksum: bytes[8]
        }
    }

    pub fn is_checksum_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    pub fn calculate_checksum(&self) -> u8 {
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
pub enum TMCLReplyStatus {
    ChecksumError = 1,
    InvalidCommand = 2,
    InvalidType = 3,
    InvalidValue = 4,
    Ok = 100,
}

impl TMCLReply {
    pub fn new(host_addr: u8, request: &TMCLRequest) -> TMCLReply {
        TMCLReply {
            host_addr: host_addr,
            device_addr: request.device_addr,
            opcode: request.opcode,
            status: TMCLReplyStatus::Ok,
            value: request.value
        }
    }

    pub fn calculate_checksum(&self) -> u8 {
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

    pub fn serialize(&self) -> [u8; TMCL_PACKET_SIZE] {
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

pub struct TMCLStack {
    pub host_address : u8,
    pub device_address : u8,
    pub module_id : u8,
}

impl TMCLStack {
    pub fn process(&self, request : &TMCLRequest) -> Option<TMCLReply> {
        let mut reply = TMCLReply::new(self.host_address, request);

        if request.device_addr != self.device_address {
            return None;
        }

        if !request.is_checksum_valid() {
            info!("Invalid TMCL checksum");
            reply.status = TMCLReplyStatus::ChecksumError;
            return Some(reply);
        }

        match request.opcode {
            157 => self.cmd_get_info(request, &mut reply),
            _ => reply.status = TMCLReplyStatus::InvalidCommand
        }

        Some(reply)
    }

    fn cmd_get_info(&self, request : &TMCLRequest, reply : &mut TMCLReply) -> () {
        let info = match request.index {
            // Module ID
            0 => Some(self.module_id as u32),

            // Unknown value
            _ => None
        };

        match info {
            Some(v) => reply.value=v,
            None => reply.status = TMCLReplyStatus::InvalidType
        }
    }
}