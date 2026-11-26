/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
/* Define the actual libc symbols, not the fortified inline or LFS redirects. */
#ifdef _FORTIFY_SOURCE
#undef _FORTIFY_SOURCE
#endif
#ifdef _FILE_OFFSET_BITS
#undef _FILE_OFFSET_BITS
#endif
#ifdef _TIME_BITS
#undef _TIME_BITS
#endif
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#ifndef _LARGEFILE64_SOURCE
#define _LARGEFILE64_SOURCE
#endif

#include <dice/chains/intercept.h>
#include <dice/events/uio.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>
#include <errno.h>

INTERPOSE(ssize_t, readv, int fd, const struct iovec *iov, int iovcnt)
{
    struct readv_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .ret    = 0,
        .func   = REAL_FUNC(readv),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_READV, &ev, &md);

    errno = 0;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt);
    ev.errno_ = errno;

    PS_PUBLISH(INTERCEPT_AFTER, EVENT_READV, &ev, &md);
    errno = ev.errno_;

    return ev.ret;
}

INTERPOSE(ssize_t, writev, int fd, const struct iovec *iov, int iovcnt)
{
    struct writev_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .ret    = 0,
        .func   = REAL_FUNC(writev),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_WRITEV, &ev, &md);

    errno = 0;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt);
    ev.errno_ = errno;

    PS_PUBLISH(INTERCEPT_AFTER, EVENT_WRITEV, &ev, &md);
    errno = ev.errno_;

    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(ssize_t, preadv, int fd, const struct iovec *iov, int iovcnt, off_t offset)
{
    int saved_errno = errno;
    struct preadv_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(preadv),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PREADV, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PREADV, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(ssize_t, preadv64, int fd, const struct iovec *iov, int iovcnt, off64_t offset)
{
    int saved_errno = errno;
    struct preadv64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(preadv64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PREADV64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PREADV64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
INTERPOSE(ssize_t, preadv2, int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
{
    int saved_errno = errno;
    struct preadv2_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(preadv2),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PREADV2, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PREADV2, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
INTERPOSE(ssize_t, preadv64v2, int fd, const struct iovec *iov, int iovcnt, off64_t offset, int flags)
{
    int saved_errno = errno;
    struct preadv64v2_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(preadv64v2),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PREADV64V2, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PREADV64V2, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(ssize_t, pwritev, int fd, const struct iovec *iov, int iovcnt, off_t offset)
{
    int saved_errno = errno;
    struct pwritev_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(pwritev),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PWRITEV, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PWRITEV, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(ssize_t, pwritev64, int fd, const struct iovec *iov, int iovcnt, off64_t offset)
{
    int saved_errno = errno;
    struct pwritev64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(pwritev64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PWRITEV64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PWRITEV64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
INTERPOSE(ssize_t, pwritev2, int fd, const struct iovec *iov, int iovcnt, off_t offset, int flags)
{
    int saved_errno = errno;
    struct pwritev2_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(pwritev2),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PWRITEV2, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PWRITEV2, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 26)
INTERPOSE(ssize_t, pwritev64v2, int fd, const struct iovec *iov, int iovcnt, off64_t offset, int flags)
{
    int saved_errno = errno;
    struct pwritev64v2_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .iov    = iov,
        .iovcnt = iovcnt,
        .offset = offset,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(pwritev64v2),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_PWRITEV64V2, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.iov, ev.iovcnt, ev.offset, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_PWRITEV64V2, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

/* Advertise event type names for debugging messages */
PS_ADVERTISE_TYPE(EVENT_READV)
PS_ADVERTISE_TYPE(EVENT_WRITEV)
PS_ADVERTISE_TYPE(EVENT_PREADV)
PS_ADVERTISE_TYPE(EVENT_PREADV64)
PS_ADVERTISE_TYPE(EVENT_PREADV2)
PS_ADVERTISE_TYPE(EVENT_PREADV64V2)
PS_ADVERTISE_TYPE(EVENT_PWRITEV)
PS_ADVERTISE_TYPE(EVENT_PWRITEV64)
PS_ADVERTISE_TYPE(EVENT_PWRITEV2)
PS_ADVERTISE_TYPE(EVENT_PWRITEV64V2)

/* Mark module initialization (optional) */
DICE_MODULE_INIT()
