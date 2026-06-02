use dice_rs::events::*;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::slice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordEnum {
    /// randomness
    GetrandomRecord(GetrandomRecord),
    /// polling
    EpollCreate1Record(EpollCreate1Record),
    EpollCtlRecord(EpollCtlRecord),
    EpollWaitRecord(EpollWaitRecord),
    PollRecord(PollRecord),
    EventfdRecord(EventfdRecord),
    /// socket management
    SocketRecord(SocketRecord),
    SocketpairRecord(SocketpairRecord),
    GetsockoptRecord(GetsockoptRecord),
    SetsockoptRecord(SetsockoptRecord),
    BindRecord(BindRecord),
    ConnectRecord(ConnectRecord),
    ShutdownRecord(ShutdownRecord),
    CloseRecord(CloseRecord),
    /// reading/writing
    ReadRecord(ReadRecord),
    WriteRecord(WriteRecord),
    ReadvRecord(ReadvRecord),
    WritevRecord(WritevRecord),
    RecvmsgRecord(RecvmsgRecord),
    SendmsgRecord(SendmsgRecord),
    RecvfromRecord(RecvfromRecord),
    SendtoRecord(SendtoRecord),
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketRecord {
    pub domain: libc::c_int,
    pub type_: libc::c_int,
    pub protocol: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for SocketEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::SocketRecord(SocketRecord {
            domain: self.domain,
            type_: self.type_,
            protocol: self.protocol,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRecord {
    pub sockfd: libc::c_int,
    pub addr: Vec<u8>,       // stores the bytes of the sockaddr
    pub addrlen: socklen_t,
    pub ret: libc::c_int,
}

impl ToRecordEnum for ConnectEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        // convert the sockaddr pointer to a byte vector
        let addr = if !self.addr.is_null() && self.addrlen > 0 {
            std::slice::from_raw_parts(self.addr as *const u8, self.addrlen as usize).to_vec()
        } else {
            Vec::new()
        };

        RecordEnum::ConnectRecord(ConnectRecord {
            sockfd: self.sockfd,
            addr,
            addrlen: self.addrlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetsockoptRecord {
    pub sockfd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,
    pub optval: Vec<u8>,
    pub optlen: socklen_t,
    pub ret: libc::c_int,
}

impl ToRecordEnum for SetsockoptEvent {
    // safe as long as optval is valid for optlen bytes
    unsafe fn to_record(&self) -> RecordEnum {
        let optval = if self.optval.is_null() || self.optlen == 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(
                self.optval as *const u8,
                self.optlen as usize,
            )
                .to_vec()
        };

        RecordEnum::SetsockoptRecord(SetsockoptRecord {
            sockfd: self.sockfd,
            level: self.level,
            optname: self.optname,
            optval,
            optlen: self.optlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindRecord {
    pub sockfd: libc::c_int,

    pub sa_family: sa_family_t,
    pub sa_data: Vec<u8>,

    pub addrlen: socklen_t,
    pub ret: libc::c_int,
}

impl ToRecordEnum for BindEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let (sa_family, sa_data) = if self.addr.is_null() {
            (0, Vec::new())
        } else {
            let addrlen = self.addrlen as usize;

            let raw = std::slice::from_raw_parts(
                self.addr as *const u8,
                addrlen,
            )
                .to_vec();

            ((*self.addr).sa_family, raw)
        };

        RecordEnum::BindRecord(BindRecord {
            sockfd: self.sockfd,
            sa_family,
            sa_data,
            addrlen: self.addrlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetsockoptRecord {
    pub sockfd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,

    pub optval: Vec<u8>,
    pub optlen: socklen_t,

    pub ret: libc::c_int,
}

impl ToRecordEnum for GetsockoptEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let optlen = if self.optlen.is_null() {
            0
        } else {
            *self.optlen
        };

        let optval = if self.optval.is_null() || optlen == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(
                self.optval as *const u8,
                optlen as usize,
            )
                .to_vec()
        };

        RecordEnum::GetsockoptRecord(GetsockoptRecord {
            sockfd: self.sockfd,
            level: self.level,
            optname: self.optname,
            optval,
            optlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendtoRecord {
    pub sockfd: libc::c_int,

    pub buf: Vec<u8>,
    pub len: usize,

    pub flags: libc::c_int,

    pub sa_family: sa_family_t,
    pub dest_addr: Vec<u8>,
    pub addrlen: socklen_t,

    pub ret: isize,
}

impl ToRecordEnum for SendtoEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let buf = if self.buf.is_null() || self.len == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(
                self.buf as *const u8,
                self.len,
            )
                .to_vec()
        };

        let (sa_family, dest_addr) = if self.dest_addr.is_null() {
            (0, Vec::new())
        } else {
            let raw = std::slice::from_raw_parts(
                self.dest_addr as *const u8,
                self.addrlen as usize,
            )
                .to_vec();

            ((*self.dest_addr).sa_family, raw)
        };

        RecordEnum::SendtoRecord(SendtoRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            sa_family,
            dest_addr,
            addrlen: self.addrlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IovecRecord {
    pub data: Vec<u8>,
    pub len: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritevRecord {
    pub fd: libc::c_int,
    pub iov: Vec<IovecRecord>,
    pub iovcnt: libc::c_int,
    pub ret: isize,
}

impl ToRecordEnum for WritevEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let iov = if self.iov.is_null() || self.iovcnt <= 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(
                self.iov,
                self.iovcnt as usize,
            )
                .iter()
                .map(|iov| {
                    let data = if iov.iov_base.is_null() || iov.iov_len == 0 {
                        Vec::new()
                    } else {
                        std::slice::from_raw_parts(
                            iov.iov_base as *const u8,
                            iov.iov_len,
                        )
                            .to_vec()
                    };

                    IovecRecord {
                        data,
                        len: iov.iov_len,
                    }
                })
                .collect()
        };

        RecordEnum::WritevRecord(WritevRecord {
            fd: self.fd,
            iov,
            iovcnt: self.iovcnt,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecvfromRecord {
    pub sockfd: libc::c_int,
    pub buf: Vec<u8>,
    pub len: usize,
    pub flags: libc::c_int,
    pub src_addr: Vec<u8>,
    pub addrlen: socklen_t,
    pub ret: isize,
}

impl ToRecordEnum for RecvfromEvent {
    // safe as long as pointers are valid
    unsafe fn to_record(&self) -> RecordEnum {
        let buf = slice::from_raw_parts(
            self.buf as *const u8,
            self.ret as usize,
        )
            .to_vec();

        let addrlen = if self.addrlen.is_null() {
            0
        } else {
            *self.addrlen
        };

        let src_addr = if self.src_addr.is_null() || addrlen == 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(
                self.src_addr as *const u8,
                addrlen as usize,
            )
                .to_vec()
        };

        RecordEnum::RecvfromRecord(RecvfromRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            src_addr,
            addrlen,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownRecord {
    pub sockfd: libc::c_int,
    pub how: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for ShutdownEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::ShutdownRecord(ShutdownRecord {
            sockfd: self.sockfd,
            how: self.how,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseRecord {
    pub fd: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for CloseEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::CloseRecord(CloseRecord {
            fd: self.fd,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpollEventRecord {
    pub events: u32,
    pub data: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpollWaitRecord {
    pub epfd: libc::c_int,
    pub events: Vec<EpollEventRecord>,
    pub maxevents: libc::c_int,
    pub timeout: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for EpollWaitEvent {
    // safe as long as events pointer is valid
    unsafe fn to_record(&self) -> RecordEnum {
        let events = if self.events.is_null() || self.ret <= 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(self.events, self.ret as usize)
                .iter()
                .map(|ev| EpollEventRecord {
                    events: ev.events,
                    data: ev.data.u64_,
                })
                .collect()
        };

        RecordEnum::EpollWaitRecord(EpollWaitRecord {
            epfd: self.epfd,
            events,
            maxevents: self.maxevents,
            timeout: self.timeout,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpollCreate1Record {
    pub flags: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for EpollCreate1Event {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::EpollCreate1Record(EpollCreate1Record {
            flags: self.flags,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpollCtlEventRecord {
    pub events: u32,
    pub data: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpollCtlRecord {
    pub epfd: libc::c_int,
    pub op: libc::c_int,
    pub fd: libc::c_int,
    pub event: Option<EpollCtlEventRecord>,
    pub ret: libc::c_int,
}

impl ToRecordEnum for EpollCtlEvent {
    // safe as long as event pointer is valid
    unsafe fn to_record(&self) -> RecordEnum {
        let event = if self.event.is_null() {
            None
        } else {
            Some(EpollCtlEventRecord {
                events: (*self.event).events,
                data: (*self.event).data.u64_,
            })
        };

        RecordEnum::EpollCtlRecord(EpollCtlRecord {
            epfd: self.epfd,
            op: self.op,
            fd: self.fd,
            event,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgHdrRecord {
    pub name: Vec<u8>,
    pub namelen: libc::socklen_t,
    pub iov: Vec<Vec<u8>>, // iovec data inlined
    pub control: Vec<u8>,
    pub controllen: usize,
    pub flags: libc::c_int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendmsgRecord {
    pub sockfd: libc::c_int,
    pub msg: Option<MsgHdrRecord>,
    pub flags: libc::c_int,
    pub ret: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecvmsgRecord {
    pub sockfd: libc::c_int,
    pub msg: Option<MsgHdrRecord>,
    pub flags: libc::c_int,
    pub ret: isize,
}

/// Helper to safely copy a raw msghdr pointer into an owned MsgHdrRecord
unsafe fn copy_msghdr(ptr: *const msghdr) -> Option<MsgHdrRecord> {
    if ptr.is_null() {
        return None;
    }

    let hdr = &*ptr;

    // Copy socket address
    let name = if hdr.msg_name.is_null() || hdr.msg_namelen == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(hdr.msg_name as *const u8, hdr.msg_namelen as usize)
            .to_vec()
    };

    // Copy iovec array
    let iov = if hdr.msg_iov.is_null() || hdr.msg_iovlen == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(hdr.msg_iov, hdr.msg_iovlen)
            .iter()
            .map(|iov| {
                if iov.iov_base.is_null() || iov.iov_len == 0 {
                    Vec::new()
                } else {
                    slice::from_raw_parts(iov.iov_base as *const u8, iov.iov_len)
                        .to_vec()
                }
            })
            .collect()
    };

    // Copy control buffer
    let control = if hdr.msg_control.is_null() || hdr.msg_controllen == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(hdr.msg_control as *const u8, hdr.msg_controllen).to_vec()
    };

    Some(MsgHdrRecord {
        name,
        namelen: hdr.msg_namelen,
        iov,
        control,
        controllen: hdr.msg_controllen,
        flags: hdr.msg_flags,
    })
}

impl ToRecordEnum for SendmsgEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::SendmsgRecord(SendmsgRecord {
            sockfd: self.sockfd,
            msg: copy_msghdr(self.msg),
            flags: self.flags,
            ret: self.ret,
        })
    }
}

impl ToRecordEnum for RecvmsgEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::RecvmsgRecord(RecvmsgRecord {
            sockfd: self.sockfd,
            msg: copy_msghdr(self.msg),
            flags: self.flags,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventfdRecord {
    pub initval: libc::c_uint,
    pub flags: libc::c_int,
    pub ret: libc::c_int,
}

impl ToRecordEnum for EventfdEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        RecordEnum::EventfdRecord(EventfdRecord {
            initval: self.initval,
            flags: self.flags,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketpairRecord {
    pub domain: libc::c_int,
    pub type_: libc::c_int,
    pub protocol: libc::c_int,
    pub sv: Option<[libc::c_int; 2]>,
    pub ret: libc::c_int,
}

impl ToRecordEnum for SocketpairEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let sv = if self.sv.is_null() {
            None
        } else {
            Some([
                *self.sv.offset(0),
                *self.sv.offset(1),
            ])
        };

        RecordEnum::SocketpairRecord(SocketpairRecord {
            domain: self.domain,
            type_: self.type_,
            protocol: self.protocol,
            sv,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRecord {
    pub fd: libc::c_int,
    pub buf: Vec<u8>,
    pub count: usize,
    pub ret: isize,
}

impl ToRecordEnum for WriteEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let buf = if self.buf.is_null() || self.count == 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(self.buf as *const u8, self.count)
                .to_vec()
        };

        RecordEnum::WriteRecord(WriteRecord {
            fd: self.fd,
            buf,
            count: self.count,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadRecord {
    pub fd: libc::c_int,
    pub buf: Vec<u8>,
    pub count: usize,
    pub ret: isize,
}

impl ToRecordEnum for ReadEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let buf = if self.buf.is_null() || self.ret <= 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(self.buf as *const u8, self.ret as usize)
                .to_vec()
        };

        RecordEnum::ReadRecord(ReadRecord {
            fd: self.fd,
            buf,
            count: self.count,
            ret: self.ret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadvRecord {
    pub fd: libc::c_int,
    pub iov: Vec<Vec<u8>>, // inlined iovec data
    pub iovcnt: libc::c_int,
    pub ret: isize,
}

impl ToRecordEnum for ReadvEvent {
    unsafe fn to_record(&self) -> RecordEnum {
        let iov = if self.iov.is_null() || self.iovcnt <= 0 {
            Vec::new()
        } else {
            // Only copy up to the number of bytes actually read (ret)
            let mut remaining = self.ret;
            slice::from_raw_parts(self.iov, self.iovcnt as usize)
                .iter()
                .map(|iov| {
                    if iov.iov_base.is_null() || iov.iov_len == 0 || remaining <= 0 {
                        Vec::new()
                    } else {
                        let len = std::cmp::min(iov.iov_len as isize, remaining) as usize;
                        remaining -= len as isize;
                        slice::from_raw_parts(iov.iov_base as *const u8, len).to_vec()
                    }
                })
                .collect()
        };

        RecordEnum::ReadvRecord(ReadvRecord {
            fd: self.fd,
            iov,
            iovcnt: self.iovcnt,
            ret: self.ret,
        })
    }
}
