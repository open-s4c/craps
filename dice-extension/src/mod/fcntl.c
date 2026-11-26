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

#include <assert.h>
#include <dice/chains/intercept.h>
#include <dice/events/fcntl.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>
#include <errno.h>
#include <stdarg.h>

/* The third argument depends on cmd; do not read it for getter commands. */
enum fcntl_arg_type { FC_NONE, FC_INT, FC_FLOCK, FC_FLOCK64, FC_OWNER, FC_HINT, FC_UNKNOWN };

static enum fcntl_arg_type fcntl_arg_type(int cmd, int large)
{
    if (cmd == F_GETFD || cmd == F_GETFL || cmd == F_GETOWN) return FC_NONE;
#if defined(__linux__)
    if (cmd == F_GETSIG || cmd == F_GETLEASE || cmd == F_GETPIPE_SZ || cmd == F_GET_SEALS)
        return FC_NONE;
#endif
    if (cmd == F_DUPFD || cmd == F_SETFD || cmd == F_SETFL || cmd == F_SETOWN)
        return FC_INT;
#ifdef F_DUPFD_CLOEXEC
    if (cmd == F_DUPFD_CLOEXEC) return FC_INT;
#endif
#if defined(__linux__)
    if (cmd == F_SETSIG || cmd == F_SETLEASE || cmd == F_NOTIFY ||
        cmd == F_ADD_SEALS || cmd == F_SETPIPE_SZ) return FC_INT;
#ifdef F_GETOWN_EX
    if (cmd == F_GETOWN_EX || cmd == F_SETOWN_EX) return FC_OWNER;
#endif
#ifdef F_GET_RW_HINT
    if (cmd == F_GET_RW_HINT || cmd == F_SET_RW_HINT) return FC_HINT;
#endif
#ifdef F_GET_FILE_RW_HINT
    if (cmd == F_GET_FILE_RW_HINT || cmd == F_SET_FILE_RW_HINT) return FC_HINT;
#endif
#endif
#if defined(__GLIBC__)
    if (cmd == F_GETLK64 || cmd == F_SETLK64 || cmd == F_SETLKW64) {
        /* The constants alias F_GETLK/... on LP64, so respect the symbol's ABI. */
        if (large || F_GETLK64 != F_GETLK) return FC_FLOCK64;
    }
#ifdef F_OFD_GETLK
    if (cmd == F_OFD_GETLK || cmd == F_OFD_SETLK || cmd == F_OFD_SETLKW)
        return large ? FC_FLOCK64 : FC_FLOCK;
#endif
#endif
    if (cmd == F_GETLK || cmd == F_SETLK || cmd == F_SETLKW) return FC_FLOCK;
    (void)large;
    return FC_UNKNOWN;
}

static uint64_t fcntl_arg(int cmd, va_list ap, int large)
{
    switch (fcntl_arg_type(cmd, large)) {
        case FC_NONE: return 0;
        case FC_INT: return (uint64_t)(int64_t)va_arg(ap, int);
        case FC_FLOCK: return (uint64_t)(uintptr_t)va_arg(ap, struct flock *);
#if defined(__GLIBC__)
        case FC_FLOCK64: return (uint64_t)(uintptr_t)va_arg(ap, struct flock64 *);
#endif
#if defined(__linux__)
        case FC_OWNER: return (uint64_t)(uintptr_t)va_arg(ap, struct f_owner_ex *);
        case FC_HINT: return (uint64_t)(uintptr_t)va_arg(ap, uint64_t *);
#endif
        default:
            /* Never consume an untyped or possibly missing variadic argument. */
            return 0;
    }
}

static int call_fcntl(int (*func)(int, int, ...), int fd, int cmd, uint64_t arg, int large)
{
    switch (fcntl_arg_type(cmd, large)) {
        case FC_NONE: return func(fd, cmd);
        case FC_INT: return func(fd, cmd, (int)arg);
        case FC_FLOCK: return func(fd, cmd, (struct flock *)(uintptr_t)arg);
#if defined(__GLIBC__)
        case FC_FLOCK64: return func(fd, cmd, (struct flock64 *)(uintptr_t)arg);
#endif
#if defined(__linux__)
        case FC_OWNER: return func(fd, cmd, (struct f_owner_ex *)(uintptr_t)arg);
        case FC_HINT: return func(fd, cmd, (uint64_t *)(uintptr_t)arg);
#endif
        default: errno = EINVAL; return -1;
    }
}

INTERPOSE(int, creat, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct creat_event ev = {
        .pc   = INTERPOSE_PC,
        .path = path,
        .mode = mode,
        .ret  = 0,
        .func = REAL_FUNC(creat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_CREAT, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.path, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_CREAT, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}

INTERPOSE(int, fcntl, int fd, int cmd, ...)
{
    int saved_errno = errno;
    va_list ap;
    va_start(ap, cmd);
    uint64_t arg = fcntl_arg(cmd, ap, 0);
    va_end(ap);

    struct fcntl_event ev = {
        .pc   = INTERPOSE_PC,
        .fd   = fd,
        .cmd  = cmd,
        .arg  = arg,
        .ret  = 0,
        .func = REAL_FUNC(fcntl),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FCNTL, &ev, &md);
    errno = saved_errno;
    ev.ret = call_fcntl(ev.func, ev.fd, ev.cmd, ev.arg, 0);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FCNTL, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}

INTERPOSE(int, open, const char *path, int flags, ...)
{
    int saved_errno = errno;
    mode_t mode = 0;
    int needs_mode = (flags & O_CREAT) != 0;
#ifdef O_TMPFILE
    needs_mode |= ((flags & O_TMPFILE) == O_TMPFILE);
#endif
    if (needs_mode) {
        va_list ap;
        va_start(ap, flags);
#ifdef __APPLE__
        mode = (mode_t)va_arg(ap, int);
#else
        mode = va_arg(ap, mode_t);
#endif
        va_end(ap);
    }
    struct open_event ev = {
        .pc    = INTERPOSE_PC,
        .path  = path,
        .flags = flags,
        .mode  = mode,
        .ret   = 0,
        .func  = REAL_FUNC(open),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPEN, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPEN, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
REAL_DECL(int, __open_2, const char *path, int flags);

static int real_open_2(const char *path, int flags, ...)
{
    int saved_errno = errno;
    __typeof__(REAL_NAME(__open_2)) func = REAL_FUNC(__open_2);
    errno = saved_errno;
    return func(path, flags);
}

INTERPOSE(int, __open_2, const char *path, int flags)
{
    int saved_errno = errno;
    struct open_event ev = {
        .pc    = INTERPOSE_PC,
        .path  = path,
        .flags = flags,
        .mode  = 0,
        .ret   = 0,
        .func  = real_open_2,
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPEN, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPEN, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

INTERPOSE(int, openat, int dirfd, const char *path, int flags, ...)
{
    int saved_errno = errno;
    mode_t mode = 0;
    int needs_mode = (flags & O_CREAT) != 0;
#ifdef O_TMPFILE
    needs_mode |= ((flags & O_TMPFILE) == O_TMPFILE);
#endif
    if (needs_mode) {
        va_list ap;
        va_start(ap, flags);
#ifdef __APPLE__
        mode = (mode_t)va_arg(ap, int);
#else
        mode = va_arg(ap, mode_t);
#endif
        va_end(ap);
    }
    struct openat_event ev = {
        .pc    = INTERPOSE_PC,
        .dirfd = dirfd,
        .path  = path,
        .flags = flags,
        .mode  = mode,
        .ret   = 0,
        .func  = REAL_FUNC(openat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPENAT, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPENAT, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
REAL_DECL(int, __openat_2, int dirfd, const char *path, int flags);

static int real_openat_2(int dirfd, const char *path, int flags, ...)
{
    int saved_errno = errno;
    __typeof__(REAL_NAME(__openat_2)) func = REAL_FUNC(__openat_2);
    errno = saved_errno;
    return func(dirfd, path, flags);
}

INTERPOSE(int, __openat_2, int dirfd, const char *path, int flags)
{
    int saved_errno = errno;
    struct openat_event ev = {
        .pc    = INTERPOSE_PC,
        .dirfd = dirfd,
        .path  = path,
        .flags = flags,
        .mode  = 0,
        .ret   = 0,
        .func  = real_openat_2,
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPENAT, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPENAT, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, openat64, int dirfd, const char *path, int flags, ...)
{
    int saved_errno = errno;
    mode_t mode = 0;
    int needs_mode = (flags & O_CREAT) != 0;
#ifdef O_TMPFILE
    needs_mode |= ((flags & O_TMPFILE) == O_TMPFILE);
#endif
    if (needs_mode) {
        va_list ap;
        va_start(ap, flags);
#ifdef __APPLE__
        mode = (mode_t)va_arg(ap, int);
#else
        mode = va_arg(ap, mode_t);
#endif
        va_end(ap);
    }
    struct openat64_event ev = {
        .pc    = INTERPOSE_PC,
        .dirfd = dirfd,
        .path  = path,
        .flags = flags,
        .mode  = mode,
        .ret   = 0,
        .func  = REAL_FUNC(openat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPENAT64, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPENAT64, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
REAL_DECL(int, __openat64_2, int dirfd, const char *path, int flags);

static int real_openat64_2(int dirfd, const char *path, int flags, ...)
{
    int saved_errno = errno;
    __typeof__(REAL_NAME(__openat64_2)) func = REAL_FUNC(__openat64_2);
    errno = saved_errno;
    return func(dirfd, path, flags);
}

INTERPOSE(int, __openat64_2, int dirfd, const char *path, int flags)
{
    int saved_errno = errno;
    struct openat64_event ev = {
        .pc    = INTERPOSE_PC,
        .dirfd = dirfd,
        .path  = path,
        .flags = flags,
        .mode  = 0,
        .ret   = 0,
        .func  = real_openat64_2,
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPENAT64, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPENAT64, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, open64, const char *path, int flags, ...)
{
    int saved_errno = errno;
    mode_t mode = 0;
    int needs_mode = (flags & O_CREAT) != 0;
#ifdef O_TMPFILE
    needs_mode |= ((flags & O_TMPFILE) == O_TMPFILE);
#endif
    if (needs_mode) {
        va_list ap;
        va_start(ap, flags);
#ifdef __APPLE__
        mode = (mode_t)va_arg(ap, int);
#else
        mode = va_arg(ap, mode_t);
#endif
        va_end(ap);
    }
    struct open64_event ev = {
        .pc    = INTERPOSE_PC,
        .path  = path,
        .flags = flags,
        .mode  = mode,
        .ret   = 0,
        .func  = REAL_FUNC(open64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPEN64, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPEN64, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
REAL_DECL(int, __open64_2, const char *path, int flags);

static int real_open64_2(const char *path, int flags, ...)
{
    int saved_errno = errno;
    __typeof__(REAL_NAME(__open64_2)) func = REAL_FUNC(__open64_2);
    errno = saved_errno;
    return func(path, flags);
}

INTERPOSE(int, __open64_2, const char *path, int flags)
{
    int saved_errno = errno;
    struct open64_event ev = {
        .pc    = INTERPOSE_PC,
        .path  = path,
        .flags = flags,
        .mode  = 0,
        .ret   = 0,
        .func  = real_open64_2,
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPEN64, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.path, ev.flags, ev.mode);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPEN64, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if !defined(__APPLE__)
INTERPOSE(int, posix_fadvise, int fd, off_t offset, off_t size, int advice)
{
    int saved_errno = errno;
    struct posix_fadvise_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .offset = offset,
        .size   = size,
        .advice = advice,
        .ret    = 0,
        .func   = REAL_FUNC(posix_fadvise),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_POSIX_FADVISE, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.fd, ev.offset, ev.size, ev.advice);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_POSIX_FADVISE, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if !defined(__APPLE__)
INTERPOSE(int, posix_fallocate, int fd, off_t offset, off_t size)
{
    int saved_errno = errno;
    struct posix_fallocate_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .offset = offset,
        .size   = size,
        .ret    = 0,
        .func   = REAL_FUNC(posix_fallocate),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_POSIX_FALLOCATE, &ev, &md);
    errno = saved_errno;
    ev.ret = ev.func(ev.fd, ev.offset, ev.size);
    saved_errno = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_POSIX_FALLOCATE, &ev, &md);
    errno = saved_errno;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, creat64, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct creat64_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(creat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_CREAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_CREAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 28)
INTERPOSE(int, fcntl64, int fd, int cmd, ...)
{
    int saved_errno = errno;
    va_list ap;
    va_start(ap, cmd);
    uint64_t arg = fcntl_arg(cmd, ap, 1);
    va_end(ap);

    struct fcntl64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .cmd    = cmd,
        .arg    = arg,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fcntl64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FCNTL64, &ev, &md);
    errno = ev.errno_;
    ev.ret = call_fcntl(ev.func, ev.fd, ev.cmd, ev.arg, 1);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FCNTL64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, fallocate, int fd, int mode, off_t offset, off_t len)
{
    int saved_errno = errno;
    struct fallocate_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .mode   = mode,
        .offset = offset,
        .len    = len,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fallocate),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FALLOCATE, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.mode, ev.offset, ev.len);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FALLOCATE, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, fallocate64, int fd, int mode, off64_t offset, off64_t len)
{
    int saved_errno = errno;
    struct fallocate64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .mode   = mode,
        .offset = offset,
        .len    = len,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fallocate64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FALLOCATE64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.mode, ev.offset, ev.len);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FALLOCATE64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, posix_fallocate64, int fd, off64_t offset, off64_t len)
{
    int saved_errno = errno;
    struct posix_fallocate64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .offset = offset,
        .len    = len,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(posix_fallocate64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_POSIX_FALLOCATE64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.offset, ev.len);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_POSIX_FALLOCATE64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, posix_fadvise64, int fd, off64_t offset, off64_t len, int advice)
{
    int saved_errno = errno;
    struct posix_fadvise64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .offset = offset,
        .len    = len,
        .advice = advice,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(posix_fadvise64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_POSIX_FADVISE64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.offset, ev.len, ev.advice);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_POSIX_FADVISE64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

/* Advertise event types */
PS_ADVERTISE_TYPE(EVENT_CREAT)
PS_ADVERTISE_TYPE(EVENT_FCNTL)
PS_ADVERTISE_TYPE(EVENT_OPEN)
PS_ADVERTISE_TYPE(EVENT_OPENAT)
PS_ADVERTISE_TYPE(EVENT_OPENAT64)
PS_ADVERTISE_TYPE(EVENT_POSIX_FADVISE)
PS_ADVERTISE_TYPE(EVENT_POSIX_FALLOCATE)
PS_ADVERTISE_TYPE(EVENT_OPEN64)
PS_ADVERTISE_TYPE(EVENT_CREAT64)
PS_ADVERTISE_TYPE(EVENT_FCNTL64)
PS_ADVERTISE_TYPE(EVENT_FALLOCATE)
PS_ADVERTISE_TYPE(EVENT_FALLOCATE64)
PS_ADVERTISE_TYPE(EVENT_POSIX_FALLOCATE64)
PS_ADVERTISE_TYPE(EVENT_POSIX_FADVISE64)

/* Module init (optional) */
DICE_MODULE_INIT()
