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
#include <dice/events/stat.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>
#include <errno.h>

INTERPOSE(int, chmod, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct chmod_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(chmod),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_CHMOD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_CHMOD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fchmod, int fd, mode_t mode)
{
    int saved_errno = errno;
    struct fchmod_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fchmod),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FCHMOD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FCHMOD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fchmodat, int dirfd, const char *path, mode_t mode, int flags)
{
    int saved_errno = errno;
    struct fchmodat_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .mode   = mode,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fchmodat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FCHMODAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.mode, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FCHMODAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, lchmod, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct lchmod_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(lchmod),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_LCHMOD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_LCHMOD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fstat, int fd, struct stat *buf)
{
    int saved_errno = errno;
    struct fstat_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fstat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSTAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSTAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, fstatat, int dirfd, const char *path, struct stat *buf, int flags)
{
    int saved_errno = errno;
    struct fstatat_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .buf    = buf,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fstatat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSTATAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.buf, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSTATAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, lstat, const char *path, struct stat *buf)
{
    int saved_errno = errno;
    struct lstat_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(lstat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_LSTAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_LSTAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, stat, const char *path, struct stat *buf)
{
    int saved_errno = errno;
    struct stat_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(stat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_STAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_STAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, fstat64, int fd, struct stat64 *buf)
{
    int saved_errno = errno;
    struct fstat64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fstat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSTAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSTAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, fstatat64, int dirfd, const char *path, struct stat64 *buf, int flags)
{
    int saved_errno = errno;
    struct fstatat64_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .buf    = buf,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(fstatat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FSTATAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.buf, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FSTATAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, lstat64, const char *path, struct stat64 *buf)
{
    int saved_errno = errno;
    struct lstat64_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(lstat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_LSTAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_LSTAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, stat64, const char *path, struct stat64 *buf)
{
    int saved_errno = errno;
    struct stat64_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(stat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_STAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_STAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 28)
INTERPOSE(int, statx, int dirfd, const char *pathname, int flags, unsigned int mask, struct statx *statxbuf)
{
    int saved_errno = errno;
    struct statx_event ev = {
        .pc       = INTERPOSE_PC,
        .dirfd    = dirfd,
        .pathname = pathname,
        .flags    = flags,
        .mask     = mask,
        .statxbuf = statxbuf,
        .ret      = 0,
        .errno_   = saved_errno,
        .func     = REAL_FUNC(statx),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_STATX, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.pathname, ev.flags, ev.mask, ev.statxbuf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_STATX, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

INTERPOSE(int, mkdir, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct mkdir_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mkdir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKDIR, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKDIR, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, mkdirat, int dirfd, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct mkdirat_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mkdirat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKDIRAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKDIRAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, mkfifo, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct mkfifo_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mkfifo),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKFIFO, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKFIFO, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, mkfifoat, int dirfd, const char *path, mode_t mode)
{
    int saved_errno = errno;
    struct mkfifoat_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .mode   = mode,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mkfifoat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKFIFOAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.mode);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKFIFOAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, mknod, const char *path, mode_t mode, dev_t dev)
{
    int saved_errno = errno;
    struct mknod_event ev = {
        .pc     = INTERPOSE_PC,
        .path   = path,
        .mode   = mode,
        .dev    = dev,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mknod),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKNOD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.path, ev.mode, ev.dev);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKNOD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(int, mknodat, int dirfd, const char *path, mode_t mode, dev_t dev)
{
    int saved_errno = errno;
    struct mknodat_event ev = {
        .pc     = INTERPOSE_PC,
        .dirfd  = dirfd,
        .path   = path,
        .mode   = mode,
        .dev    = dev,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(mknodat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_MKNODAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirfd, ev.path, ev.mode, ev.dev);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_MKNODAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

INTERPOSE(mode_t, umask, mode_t mask)
{
    int saved_errno = errno;
    struct umask_event ev = {
        .pc     = INTERPOSE_PC,
        .mask   = mask,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(umask),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_UMASK, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.mask);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_UMASK, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __xstat, int ver, const char *path, struct stat *buf)
{
    int saved_errno = errno;
    struct xstat_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__xstat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_XSTAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_XSTAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __fxstat, int ver, int fd, struct stat *buf)
{
    int saved_errno = errno;
    struct fxstat_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .fd     = fd,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__fxstat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FXSTAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.fd, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FXSTAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __lxstat, int ver, const char *path, struct stat *buf)
{
    int saved_errno = errno;
    struct lxstat_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__lxstat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_LXSTAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_LXSTAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __fxstatat, int ver, int dirfd, const char *path, struct stat *buf, int flags)
{
    int saved_errno = errno;
    struct fxstatat_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .dirfd  = dirfd,
        .path   = path,
        .buf    = buf,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__fxstatat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FXSTATAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.dirfd, ev.path, ev.buf, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FXSTATAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __xstat64, int ver, const char *path, struct stat64 *buf)
{
    int saved_errno = errno;
    struct xstat64_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__xstat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_XSTAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_XSTAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __fxstat64, int ver, int fd, struct stat64 *buf)
{
    int saved_errno = errno;
    struct fxstat64_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .fd     = fd,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__fxstat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FXSTAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.fd, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FXSTAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __lxstat64, int ver, const char *path, struct stat64 *buf)
{
    int saved_errno = errno;
    struct lxstat64_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .path   = path,
        .buf    = buf,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__lxstat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_LXSTAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.path, ev.buf);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_LXSTAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __fxstatat64, int ver, int dirfd, const char *path, struct stat64 *buf, int flags)
{
    int saved_errno = errno;
    struct fxstatat64_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .dirfd  = dirfd,
        .path   = path,
        .buf    = buf,
        .flags  = flags,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__fxstatat64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FXSTATAT64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.dirfd, ev.path, ev.buf, ev.flags);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FXSTATAT64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __xmknod, int ver, const char *path, mode_t mode, dev_t *dev)
{
    int saved_errno = errno;
    struct xmknod_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .path   = path,
        .mode   = mode,
        .dev    = dev,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__xmknod),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_XMKNOD, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.path, ev.mode, ev.dev);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_XMKNOD, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, __xmknodat, int ver, int dirfd, const char *path, mode_t mode, dev_t *dev)
{
    int saved_errno = errno;
    struct xmknodat_event ev = {
        .pc     = INTERPOSE_PC,
        .ver    = ver,
        .dirfd  = dirfd,
        .path   = path,
        .mode   = mode,
        .dev    = dev,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(__xmknodat),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_XMKNODAT, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.ver, ev.dirfd, ev.path, ev.mode, ev.dev);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_XMKNODAT, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

/* Advertise event types */
PS_ADVERTISE_TYPE(EVENT_CHMOD)
PS_ADVERTISE_TYPE(EVENT_FCHMOD)
PS_ADVERTISE_TYPE(EVENT_FCHMODAT)
PS_ADVERTISE_TYPE(EVENT_LCHMOD)
PS_ADVERTISE_TYPE(EVENT_FSTAT)
PS_ADVERTISE_TYPE(EVENT_FSTATAT)
PS_ADVERTISE_TYPE(EVENT_LSTAT)
PS_ADVERTISE_TYPE(EVENT_STAT)
PS_ADVERTISE_TYPE(EVENT_FSTAT64)
PS_ADVERTISE_TYPE(EVENT_FSTATAT64)
PS_ADVERTISE_TYPE(EVENT_LSTAT64)
PS_ADVERTISE_TYPE(EVENT_STAT64)
PS_ADVERTISE_TYPE(EVENT_STATX)
PS_ADVERTISE_TYPE(EVENT_MKDIR)
PS_ADVERTISE_TYPE(EVENT_MKDIRAT)
PS_ADVERTISE_TYPE(EVENT_MKFIFO)
PS_ADVERTISE_TYPE(EVENT_MKFIFOAT)
PS_ADVERTISE_TYPE(EVENT_MKNOD)
PS_ADVERTISE_TYPE(EVENT_MKNODAT)
PS_ADVERTISE_TYPE(EVENT_UMASK)
PS_ADVERTISE_TYPE(EVENT_XSTAT)
PS_ADVERTISE_TYPE(EVENT_FXSTAT)
PS_ADVERTISE_TYPE(EVENT_LXSTAT)
PS_ADVERTISE_TYPE(EVENT_FXSTATAT)
PS_ADVERTISE_TYPE(EVENT_XSTAT64)
PS_ADVERTISE_TYPE(EVENT_FXSTAT64)
PS_ADVERTISE_TYPE(EVENT_LXSTAT64)
PS_ADVERTISE_TYPE(EVENT_FXSTATAT64)
PS_ADVERTISE_TYPE(EVENT_XMKNOD)
PS_ADVERTISE_TYPE(EVENT_XMKNODAT)

DICE_MODULE_INIT()
