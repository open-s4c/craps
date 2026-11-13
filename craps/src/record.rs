#![cfg_attr(
    feature = "abomonation",
    allow(non_local_definitions)
)]

use dice_rs::events::*;
use std::ffi::CStr;
use std::fmt::Debug;
use std::slice;

#[cfg(not(any(
    feature = "serde-json",
    feature = "abomonation",
)))]
compile_error!(
    "one serialization feature must be enabled: \
     `serde-json` or `abomonation`"
);

#[cfg(any(
    all(feature = "serde-json", feature = "abomonation"),
))]
compile_error!(
    "serialization features are mutually exclusive: \
     enable only one of `serde-json` or `abomonation`"
);

#[cfg(any(
    feature = "serde-json",
    feature = "abomonation",
))]
pub(crate) use RecordEnum as ReplayRecord;

macro_rules! record_type {
    ($item:item) => {
        #[derive(Debug, Clone)]
        #[cfg_attr(
            feature = "serde-json",
            derive(serde::Serialize, serde::Deserialize)
        )]
        #[cfg_attr(
            feature = "abomonation",
            derive(abomonation_derive::Abomonation)
        )]
        $item
    };
}

record_type! {
    pub enum RecordEnum {
        /// randomness
        GetrandomRecord(GetrandomRecord),
        SyscallGetrandomRecord(SyscallGetrandomRecord),
        /// polling
        EpollCreate1Record(EpollCreate1Record),
        EpollCtlRecord(EpollCtlRecord),
        EpollWaitRecord(EpollWaitRecord),
        PollRecord(PollRecord),
        EventfdRecord(EventfdRecord),
        PpollRecord(PpollRecord),
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
        OpenatRecord(OpenatRecord),
        MmapRecord(MmapRecord),
        RecvRecord(RecvRecord),
        SendRecord(SendRecord),
        FeofRecord(FeofRecord),
        FerrorRecord(FerrorRecord),
        FgetcRecord(FgetcRecord),
        FgetposRecord(FgetposRecord),
        FgetsRecord(FgetsRecord),
        FputcRecord(FputcRecord),
        FputsRecord(FputsRecord),
        FreadRecord(FreadRecord),
        FseekRecord(FseekRecord),
        FsetposRecord(FsetposRecord),
        FtellRecord(FtellRecord),
        FwriteRecord(FwriteRecord),
        /// time
        GettimeofdayRecord(GettimeofdayRecord),
        ClockGettimeRecord(ClockGettimeRecord),
        TimeRecord(TimeRecord),
    }
}

pub(crate) trait ToRecordEnum {
    fn to_record(&self) -> Option<RecordEnum>;
}

record_type! {
    pub struct Pollfd {
        pub fd: libc::c_int,
        pub events: libc::c_short,
        pub revents: libc::c_short,
    }
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

record_type! {
    pub struct PollRecord {
        pub fds: Vec<Pollfd>,
        pub timeout: libc::c_int,
        pub ret: libc::c_int,
    }
}

impl ToRecordEnum for PollEvent {
    // safe as long as poll call is valid, i.e., the array pointer and size are correct
    fn to_record(&self) -> Option<RecordEnum> {
        let fds_vec: Vec<Pollfd> = (0..self.nfds)
            .map(|i| unsafe { *self.fds.add(i as usize) })
            .map(Pollfd::from_libc_pollfd)
            .collect();

        Some(RecordEnum::PollRecord(PollRecord {
            fds: fds_vec,
            timeout: self.timeout,
            ret: self.ret,
        }))
    }
}

record_type! {
    pub struct GetrandomRecord {
        pub buf: Vec<u8>,
        pub size: usize,
        pub flags: libc::c_uint,
        pub ret: isize,
    }
}

impl ToRecordEnum for GetrandomEvent {
    // safe as long as buf is safe
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = unsafe { slice::from_raw_parts(self.buf as *const u8, self.size) }.to_vec();
        Some(RecordEnum::GetrandomRecord(GetrandomRecord {
            buf: buf,
            size: self.size,
            flags: self.flags,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct SocketRecord {
    pub domain: libc::c_int,
    pub type_: libc::c_int,
    pub protocol: libc::c_int,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SocketEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::SocketRecord(SocketRecord {
            domain: self.domain,
            type_: self.type_,
            protocol: self.protocol,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct ConnectRecord {
    pub sockfd: libc::c_int,
    pub addr: Vec<u8>,       // stores the bytes of the sockaddr
    pub addrlen: socklen_t,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for ConnectEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        // convert the sockaddr pointer to a byte vector
        let addr = if !self.addr.is_null() && self.addrlen > 0 {
            unsafe { slice::from_raw_parts(self.addr as *const u8, self.addrlen as usize) }.to_vec()
        } else {
            Vec::new()
        };

        Some(RecordEnum::ConnectRecord(ConnectRecord {
            sockfd: self.sockfd,
            addr,
            addrlen: self.addrlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct SetsockoptRecord {
    pub sockfd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,
    pub optval: Vec<u8>,
    pub optlen: socklen_t,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SetsockoptEvent {
    // safe as long as optval is valid for optlen bytes
    fn to_record(&self) -> Option<RecordEnum> {
        let optval = if self.optval.is_null() || self.optlen == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.optval as *const u8,
                self.optlen as usize,
            ) }
                .to_vec()
        };

        Some(RecordEnum::SetsockoptRecord(SetsockoptRecord {
            sockfd: self.sockfd,
            level: self.level,
            optname: self.optname,
            optval,
            optlen: self.optlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct BindRecord {
    pub sockfd: libc::c_int,

    pub sa_family: sa_family_t,
    pub sa_data: Vec<u8>,

    pub addrlen: socklen_t,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for BindEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let (sa_family, sa_data) = if self.addr.is_null() {
            (0, Vec::new())
        } else {
            let addrlen = self.addrlen as usize;

            let raw = unsafe { slice::from_raw_parts(
                self.addr as *const u8,
                addrlen,
            ) }
                .to_vec();

            (unsafe { *self.addr }.sa_family, raw)
        };

        Some(RecordEnum::BindRecord(BindRecord {
            sockfd: self.sockfd,
            sa_family,
            sa_data,
            addrlen: self.addrlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct GetsockoptRecord {
    pub sockfd: libc::c_int,
    pub level: libc::c_int,
    pub optname: libc::c_int,

    pub optval: Vec<u8>,
    pub optlen: socklen_t,

    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for GetsockoptEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let optlen = if self.optlen.is_null() {
            0
        } else {
            unsafe { *self.optlen }
        };

        let optval = if self.optval.is_null() || optlen == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.optval as *const u8,
                optlen as usize,
            ) }
                .to_vec()
        };

        Some(RecordEnum::GetsockoptRecord(GetsockoptRecord {
            sockfd: self.sockfd,
            level: self.level,
            optname: self.optname,
            optval,
            optlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct SendtoRecord {
    pub sockfd: libc::c_int,

    pub buf: Vec<u8>,
    pub len: usize,

    pub flags: libc::c_int,

    pub sa_family: sa_family_t,
    pub dest_addr: Vec<u8>,
    pub addrlen: socklen_t,

    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SendtoEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = if self.buf.is_null() || self.len == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.buf as *const u8,
                self.len,
            ) }
                .to_vec()
        };

        let (sa_family, dest_addr) = if self.dest_addr.is_null() {
            (0, Vec::new())
        } else {
            let raw = unsafe { slice::from_raw_parts(
                self.dest_addr as *const u8,
                self.addrlen as usize,
            ) }
                .to_vec();

            (unsafe { *self.dest_addr }.sa_family, raw)
        };

        Some(RecordEnum::SendtoRecord(SendtoRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            sa_family,
            dest_addr,
            addrlen: self.addrlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct IovecRecord {
    pub data: Vec<u8>,
    pub len: usize,
}}

record_type! {
pub(crate) struct WritevRecord {
    pub fd: libc::c_int,
    pub iov: Vec<IovecRecord>,
    pub iovcnt: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for WritevEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let iov = if self.iov.is_null() || self.iovcnt <= 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.iov,
                self.iovcnt as usize,
            ) }
                .iter()
                .map(|iov| {
                    let data = if iov.iov_base.is_null() || iov.iov_len == 0 {
                        Vec::new()
                    } else {
                        unsafe { slice::from_raw_parts(
                            iov.iov_base as *const u8,
                            iov.iov_len,
                        ) }
                            .to_vec()
                    };

                    IovecRecord {
                        data,
                        len: iov.iov_len,
                    }
                })
                .collect()
        };

        Some(RecordEnum::WritevRecord(WritevRecord {
            fd: self.fd,
            iov,
            iovcnt: self.iovcnt,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct RecvfromRecord {
    pub sockfd: libc::c_int,
    pub buf: Vec<u8>,
    pub len: usize,
    pub flags: libc::c_int,
    pub src_addr: Vec<u8>,
    pub addrlen: socklen_t,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for RecvfromEvent {
    // safe as long as pointers are valid
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = unsafe { slice::from_raw_parts(
            self.buf as *const u8,
            self.ret as usize,
        ) }
            .to_vec();

        let addrlen = if self.addrlen.is_null() {
            0
        } else {
            unsafe { *self.addrlen }
        };

        let src_addr = if self.src_addr.is_null() || addrlen == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.src_addr as *const u8,
                addrlen as usize,
            ) }
                .to_vec()
        };

        Some(RecordEnum::RecvfromRecord(RecvfromRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            src_addr,
            addrlen,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct ShutdownRecord {
    pub sockfd: libc::c_int,
    pub how: libc::c_int,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for ShutdownEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::ShutdownRecord(ShutdownRecord {
            sockfd: self.sockfd,
            how: self.how,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct CloseRecord {
    pub fd: libc::c_int,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for CloseEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::CloseRecord(CloseRecord {
            fd: self.fd,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct EpollEventRecord {
    pub events: u32,
    pub data: u64,
}}

record_type! {
pub(crate) struct EpollWaitRecord {
    pub epfd: libc::c_int,
    pub events: Vec<EpollEventRecord>,
    pub maxevents: libc::c_int,
    pub timeout: libc::c_int,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for EpollWaitEvent {
    // safe as long as events pointer is valid
    fn to_record(&self) -> Option<RecordEnum> {
        let events = if self.events.is_null() || self.ret <= 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(self.events, self.ret as usize) }
                .iter()
                .map(|ev| EpollEventRecord {
                    events: ev.events,
                    data: unsafe { ev.data.u64_ },
                })
                .collect()
        };

        Some(RecordEnum::EpollWaitRecord(EpollWaitRecord {
            epfd: self.epfd,
            events,
            maxevents: self.maxevents,
            timeout: self.timeout,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct EpollCreate1Record {
    pub flags: libc::c_int,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for EpollCreate1Event {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::EpollCreate1Record(EpollCreate1Record {
            flags: self.flags,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct EpollCtlEventRecord {
    pub events: u32,
    pub data: u64,
}}

record_type! {
pub(crate) struct EpollCtlRecord {
    pub epfd: libc::c_int,
    pub op: libc::c_int,
    pub fd: libc::c_int,
    pub event: Option<EpollCtlEventRecord>,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for EpollCtlEvent {
    // safe as long as event pointer is valid
    fn to_record(&self) -> Option<RecordEnum> {
        let event = if self.event.is_null() {
            None
        } else {
            let event = unsafe { *self.event };
            let data = event.data;
            Some(EpollCtlEventRecord {
                events: event.events,
                data: unsafe { data.u64_ },
            })
        };

        Some(RecordEnum::EpollCtlRecord(EpollCtlRecord {
            epfd: self.epfd,
            op: self.op,
            fd: self.fd,
            event,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct MsgHdrRecord {
    pub name: Vec<u8>,
    pub namelen: libc::socklen_t,
    pub iov: Vec<Vec<u8>>, // iovec data inlined
    pub control: Vec<u8>,
    pub controllen: usize,
    pub flags: libc::c_int,
}}

record_type! {
pub(crate) struct SendmsgRecord {
    pub sockfd: libc::c_int,
    pub msg: Option<MsgHdrRecord>,
    pub flags: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

record_type! {
pub(crate) struct RecvmsgRecord {
    pub sockfd: libc::c_int,
    pub msg: Option<MsgHdrRecord>,
    pub flags: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

/// Helper to safely copy a raw msghdr pointer into an owned MsgHdrRecord
fn copy_msghdr(ptr: *const msghdr) -> Option<MsgHdrRecord> {
    if ptr.is_null() {
        return None;
    }

    let hdr = & unsafe { *ptr };

    // Copy socket address
    let name = if hdr.msg_name.is_null() || hdr.msg_namelen == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(hdr.msg_name as *const u8, hdr.msg_namelen as usize) }
            .to_vec()
    };

    // Copy iovec array
    let iov = if hdr.msg_iov.is_null() || hdr.msg_iovlen == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(hdr.msg_iov, hdr.msg_iovlen) }
            .iter()
            .map(|iov| {
                if iov.iov_base.is_null() || iov.iov_len == 0 {
                    Vec::new()
                } else {
                    unsafe { slice::from_raw_parts(iov.iov_base as *const u8, iov.iov_len) }
                        .to_vec()
                }
            })
            .collect()
    };

    // Copy control buffer
    let control = if hdr.msg_control.is_null() || hdr.msg_controllen == 0 {
        Vec::new()
    } else {
        unsafe { slice::from_raw_parts(hdr.msg_control as *const u8, hdr.msg_controllen) }.to_vec()
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
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::SendmsgRecord(SendmsgRecord {
            sockfd: self.sockfd,
            msg: copy_msghdr(self.msg),
            flags: self.flags,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

impl ToRecordEnum for RecvmsgEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::RecvmsgRecord(RecvmsgRecord {
            sockfd: self.sockfd,
            msg: copy_msghdr(self.msg),
            flags: self.flags,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct EventfdRecord {
    pub initval: libc::c_uint,
    pub flags: libc::c_int,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for EventfdEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::EventfdRecord(EventfdRecord {
            initval: self.initval,
            flags: self.flags,
            ret: self.ret,
        }))
    }
}

record_type! {
pub struct SocketpairRecord {
    pub domain: libc::c_int,
    pub type_: libc::c_int,
    pub protocol: libc::c_int,
    pub sv: Option<[libc::c_int; 2]>,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SocketpairEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let sv = if self.sv.is_null() {
            None
        } else {
            Some([
                unsafe { *self.sv.offset(0) },
                unsafe { *self.sv.offset(1) },
            ])
        };

        Some(RecordEnum::SocketpairRecord(SocketpairRecord {
            domain: self.domain,
            type_: self.type_,
            protocol: self.protocol,
            sv,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct WriteRecord {
    pub fd: libc::c_int,
    pub buf: Vec<u8>,
    pub count: usize,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for WriteEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = if self.buf.is_null() || self.count == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(self.buf as *const u8, self.count) }
                .to_vec()
        };

        Some(RecordEnum::WriteRecord(WriteRecord {
            fd: self.fd,
            buf,
            count: self.count,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct ReadRecord {
    pub fd: libc::c_int,
    pub buf: Vec<u8>,
    pub count: usize,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for ReadEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = if self.buf.is_null() || self.ret <= 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(self.buf as *const u8, self.ret as usize) }
                .to_vec()
        };

        Some(RecordEnum::ReadRecord(ReadRecord {
            fd: self.fd,
            buf,
            count: self.count,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct ReadvRecord {
    pub fd: libc::c_int,
    pub iov: Vec<Vec<u8>>, // inlined iovec data
    pub iovcnt: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for ReadvEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let iov = if self.iov.is_null() || self.iovcnt <= 0 {
            Vec::new()
        } else {
            // Only copy up to the number of bytes actually read (ret)
            let mut remaining = self.ret;
            unsafe { slice::from_raw_parts(self.iov, self.iovcnt as usize) }
                .iter()
                .map(|iov| {
                    if iov.iov_base.is_null() || iov.iov_len == 0 || remaining <= 0 {
                        Vec::new()
                    } else {
                        let len = std::cmp::min(iov.iov_len as isize, remaining) as usize;
                        remaining -= len as isize;
                        unsafe { slice::from_raw_parts(iov.iov_base as *const u8, len) }.to_vec()
                    }
                })
                .collect()
        };

        Some(RecordEnum::ReadvRecord(ReadvRecord {
            fd: self.fd,
            iov,
            iovcnt: self.iovcnt,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct TimevalRecord {
    pub tv_sec: libc::c_long,
    pub tv_usec: libc::c_long,
}}

record_type! {
pub(crate) struct TimespecRecord {
    pub tv_sec: libc::c_long,
    pub tv_nsec: libc::c_long,
}}

record_type! {
pub struct GettimeofdayRecord {
    pub tv: Option<TimevalRecord>,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for GettimeofdayEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let tv = if self.tv.is_null() {
            None
        } else {
            let tv = unsafe { *self.tv };
            Some(TimevalRecord {
                tv_sec: tv.tv_sec,
                tv_usec: tv.tv_usec,
            })
        };

        Some(RecordEnum::GettimeofdayRecord(GettimeofdayRecord {
            tv,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct ClockGettimeRecord {
    pub clockid: clockid_t,
    pub tp: Option<TimespecRecord>,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for ClockGettimeEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let tp = if self.tp.is_null() {
            None
        } else {
            let tp = unsafe { *self.tp };
            Some(TimespecRecord {
                tv_sec: tp.tv_sec,
                tv_nsec: tp.tv_nsec,
            })
        };

        Some(RecordEnum::ClockGettimeRecord(ClockGettimeRecord {
            clockid: self.clockid,
            tp,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct OpenatRecord {
    pub dirfd: libc::c_int,
    pub path: Option<String>,
    pub flags: libc::c_int,
    pub mode: mode_t,
    pub ret: libc::c_int,
}}

impl ToRecordEnum for OpenatEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let path = if self.path.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(self.path) }
                        .to_string_lossy()
                        .into_owned()
                ,
            )
        };

        Some(RecordEnum::OpenatRecord(OpenatRecord {
            dirfd: self.dirfd,
            path,
            flags: self.flags,
            mode: self.mode,
            ret: self.ret,
        }))
    }
}

record_type! {
pub(crate) struct MmapRecord {
    pub addr_contents: Option<Vec<u8>>,
    pub length: usize,
    pub prot: libc::c_int,
    pub flags: libc::c_int,
    pub fd: libc::c_int,
    pub offset: off_t,
    pub ret: isize,
}}

impl ToRecordEnum for MmapEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let addr_contents = if self.addr.is_null() {
            None
        } else if self.length == 0 {
            Some(Vec::new())
        } else {
            Some(
                unsafe {
                slice::from_raw_parts(
                    self.addr.cast::<u8>(),
                    self.length,
                ) }
                    .to_vec(),
            )
        };

        Some(RecordEnum::MmapRecord(MmapRecord {
            addr_contents,
            length: self.length,
            prot: self.prot,
            flags: self.flags,
            fd: self.fd,
            offset: self.offset,
            ret: self.ret as isize,
        }))
    }
}

record_type! {
pub(crate) struct RecvRecord {
    pub sockfd: libc::c_int,
    pub buf: Vec<u8>,
    pub len: usize,
    pub flags: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for RecvEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = if self.buf.is_null() || self.ret <= 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.buf as *const u8,
                self.ret as usize,
            ) }
                .to_vec()
        };

        Some(RecordEnum::RecvRecord(RecvRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct SendRecord {
    pub sockfd: libc::c_int,
    pub buf: Vec<u8>,
    pub len: usize,
    pub flags: libc::c_int,
    pub ret: isize,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SendEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let buf = if self.buf.is_null() || self.len == 0 {
            Vec::new()
        } else {
            unsafe { slice::from_raw_parts(
                self.buf as *const u8,
                self.len,
            ) }
                .to_vec()
        };

        Some(RecordEnum::SendRecord(SendRecord {
            sockfd: self.sockfd,
            buf,
            len: self.len,
            flags: self.flags,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct PpollRecord {
    pub fds: Vec<Pollfd>,
    pub tmo_p: Option<TimespecRecord>,
    pub sigmask: Option<Vec<u8>>,
    pub ret: libc::c_int,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for PpollEvent {
    // safe as long as the ppoll arguments are valid
    fn to_record(&self) -> Option<RecordEnum> {
        let fds_vec: Vec<Pollfd> = if self.fds.is_null() || self.nfds == 0 {
            Vec::new()
        } else {
            (0..self.nfds)
                .map(|i| unsafe { *self.fds.add(i as usize) })
                .map(Pollfd::from_libc_pollfd)
                .collect()
        };

        let tmo_p = if self.tmo_p.is_null() {
            None
        } else {
            Some(TimespecRecord {
                tv_sec: unsafe { (*self.tmo_p).tv_sec },
                tv_nsec: unsafe { (*self.tmo_p).tv_nsec },
            })
        };

        let sigmask = if self.sigmask.is_null() {
            None
        } else {
            Some(
                unsafe {
                    slice::from_raw_parts(
                        self.sigmask as *const u8,
                        std::mem::size_of::<libc::sigset_t>(),
                    )
                }
                    .to_vec(),
            )
        };

        Some(RecordEnum::PpollRecord(PpollRecord {
            fds: fds_vec,
            tmo_p,
            sigmask,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
pub(crate) struct SyscallGetrandomRecord {
    pub buf: Vec<u8>,
    pub buflen: usize,
    pub flags: libc::c_uint,
    pub ret: libc::c_long,
    pub errno: libc::c_int,
}}

impl ToRecordEnum for SyscallEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        match self.number {
            libc::SYS_getrandom => {
                let buf_ptr = self.args[0] as usize as *const u8;
                let buflen = self.args[1] as usize;
                let flags = self.args[2] as libc::c_uint;

                let buf = if buf_ptr.is_null() || self.ret <= 0 {
                    Vec::new()
                } else {
                    unsafe { slice::from_raw_parts(buf_ptr, self.ret as usize).to_vec() }
                };

                Some(RecordEnum::SyscallGetrandomRecord(SyscallGetrandomRecord {
                    buf,
                    buflen,
                    flags,
                    ret: self.ret,
                    errno: self.errno_,
                }))
            }
            _ => None
        }
    }
}

record_type! {
pub(crate) struct TimeRecord {
    pub ret: time_t,
}}

impl ToRecordEnum for TimeEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        Some(RecordEnum::TimeRecord(TimeRecord {
            ret: self.ret,
        }))
    }
}

// stdio

macro_rules! stdio_scalar_record {
    ($record:ident, $event:ty, $ret_ty:ty) => {
        record_type! {
            pub(crate) struct $record {
                pub ret: $ret_ty,
                pub errno: libc::c_int,
            }
        }

        impl ToRecordEnum for $event {
            fn to_record(&self) -> Option<RecordEnum> {
                Some(RecordEnum::$record($record {
                    ret: self.ret,
                    errno: self.errno_,
                }))
            }
        }
    };
}

stdio_scalar_record!(FeofRecord, FeofEvent, libc::c_int);
stdio_scalar_record!(FerrorRecord, FerrorEvent, libc::c_int);
stdio_scalar_record!(FgetcRecord, FgetcEvent, libc::c_int);
stdio_scalar_record!(FputcRecord, FputcEvent, libc::c_int);
stdio_scalar_record!(FputsRecord, FputsEvent, libc::c_int);
stdio_scalar_record!(FseekRecord, FseekEvent, libc::c_int);
stdio_scalar_record!(FsetposRecord, FsetposEvent, libc::c_int);
stdio_scalar_record!(FtellRecord, FtellEvent, libc::c_long);
stdio_scalar_record!(FwriteRecord, FwriteEvent, libc::size_t);

record_type! {
    pub(crate) struct FgetposRecord {
        pub pos: Option<Vec<u8>>,
        pub ret: libc::c_int,
        pub errno: libc::c_int,
    }
}

impl ToRecordEnum for FgetposEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let pos = if self.ret == 0 {
            assert!(
                !self.pos.is_null(),
                "fgetpos succeeded with null pos"
            );

            Some(unsafe {
                slice::from_raw_parts(
                    self.pos.cast::<u8>(),
                    std::mem::size_of::<libc::fpos_t>(),
                )
                    .to_vec()
            })
        } else {
            None
        };

        Some(RecordEnum::FgetposRecord(FgetposRecord {
            pos,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}

record_type! {
    pub(crate) struct FgetsRecord {
        pub success: bool,
        pub buf: Vec<u8>,
        pub size: libc::c_int,
        pub errno: libc::c_int,
    }
}

impl ToRecordEnum for FgetsEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        let success = !self.ret.is_null();

        let buf = if success {
            assert!(
                self.size > 0,
                "successful fgets needs positive size"
            );
            assert!(
                !self.s.is_null(),
                "successful fgets needs a buffer"
            );
            assert_eq!(
                self.ret,
                self.s,
                "fgets returned a different buffer"
            );

            let buf = unsafe {
                CStr::from_ptr(self.s)
            }
                .to_bytes_with_nul()
                .to_vec();

            assert!(buf.len() <= self.size as usize);

            buf
        } else {
            Vec::new()
        };

        Some(RecordEnum::FgetsRecord(FgetsRecord {
            success,
            buf,
            size: self.size,
            errno: self.errno_,
        }))
    }
}

record_type! {
    pub(crate) struct FreadRecord {
        pub buf: Vec<u8>,
        pub size: libc::size_t,
        pub n: libc::size_t,
        pub ret: libc::size_t,
        pub errno: libc::c_int,
    }
}

impl ToRecordEnum for FreadEvent {
    fn to_record(&self) -> Option<RecordEnum> {
        assert!(
            self.ret <= self.n,
            "fread returned too many elements"
        );

        let len = self.ret
            .checked_mul(self.size)
            .expect("fread result byte count overflowed");

        let buf = if len == 0 {
            Vec::new()
        } else {
            assert!(
                !self.ptr.is_null(),
                "fread returned data with null ptr"
            );

            unsafe {
                slice::from_raw_parts(
                    self.ptr.cast::<u8>(),
                    len,
                )
                    .to_vec()
            }
        };

        Some(RecordEnum::FreadRecord(FreadRecord {
            buf,
            size: self.size,
            n: self.n,
            ret: self.ret,
            errno: self.errno_,
        }))
    }
}
