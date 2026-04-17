use dice_rs::events::*;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordEnum {
    PollRecord(PollRecord),
    GetrandomRecord(GetrandomRecord)
}

pub trait Record {}

pub trait ToRecordEnum {
    unsafe fn to_record(&self) -> RecordEnum;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}

impl Pollfd {
    pub fn from_libc_pollfd(pfd: dice_rs::events::pollfd) -> Self {
        Pollfd {
            fd: pfd.fd,
            events: pfd.events,
            revents: pfd.revents,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollRecord {
    pub fds: Vec<Pollfd>,
    pub timeout: libc::c_int,
    pub ret: libc::c_int,
}

impl Record for PollRecord {}

impl ToRecordEnum for PollEvent {
    // safe as long as poll call is valid, i.e., the array pointer and size are correct
    unsafe fn to_record(&self) -> RecordEnum {
        let fds_vec: Vec<Pollfd> = (0..self.nfds)
            .map(|i| unsafe { *self.fds.add(i as usize) })
            .map(Pollfd::from_libc_pollfd)
            .collect();

        RecordEnum::PollRecord(PollRecord {
            fds: fds_vec,
            timeout: self.timeout,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetrandomRecord {
    pub buf: Vec<u8>,
    pub size: usize,
    pub flags: libc::c_uint,
    pub ret: isize,
}

impl Record for GetrandomRecord {}

impl ToRecordEnum for GetrandomEvent {
    // safe as long as buf is safe
    unsafe fn to_record(&self) -> RecordEnum {
        let buf = slice::from_raw_parts(self.buf as *const u8, self.size).to_vec();
        RecordEnum::GetrandomRecord(GetrandomRecord {
            buf: buf,
            size: self.size,
            flags: self.flags,
            ret: self.ret,
        })
    }
}
