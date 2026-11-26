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

#include <errno.h>
#include <dice/chains/intercept.h>
#include <dice/events/dirent.h>
#include <dice/interpose.h>
#include <dice/module.h>
#include <dice/pubsub.h>

INTERPOSE(int, closedir, DIR *dirp)
{
    struct closedir_event ev = {
        .pc   = INTERPOSE_PC,
        .dirp = dirp,
        .ret  = 0,
        .func = REAL_FUNC(closedir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_CLOSEDIR, &ev, &md);
    ev.ret = ev.func(ev.dirp);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_CLOSEDIR, &ev, &md);
    return ev.ret;
}

INTERPOSE(DIR *, fdopendir, int fd)
{
    struct fdopendir_event ev = {
        .pc   = INTERPOSE_PC,
        .fd   = fd,
        .ret  = NULL,
        .func = REAL_FUNC(fdopendir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_FDOPENDIR, &ev, &md);
    ev.ret = ev.func(ev.fd);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_FDOPENDIR, &ev, &md);
    return ev.ret;
}

INTERPOSE(DIR *, opendir, const char *name)
{
    struct opendir_event ev = {
        .pc   = INTERPOSE_PC,
        .name = name,
        .ret  = NULL,
        .func = REAL_FUNC(opendir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_OPENDIR, &ev, &md);
    ev.ret = ev.func(ev.name);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_OPENDIR, &ev, &md);
    return ev.ret;
}

INTERPOSE(struct dirent *, readdir, DIR *dirp)
{
    struct readdir_event ev = {
        .pc   = INTERPOSE_PC,
        .dirp = dirp,
        .ret  = NULL,
        .func = REAL_FUNC(readdir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_READDIR, &ev, &md);
    ev.ret = ev.func(ev.dirp);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_READDIR, &ev, &md);
    return ev.ret;
}

INTERPOSE(int, readdir_r, DIR *restrict dirp,
          struct dirent *restrict entry,
          struct dirent **restrict result)
{
    struct readdir_r_event ev = {
        .pc     = INTERPOSE_PC,
        .dirp   = dirp,
        .entry  = entry,
        .result = result,
        .ret    = 0,
        .func   = REAL_FUNC(readdir_r),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_READDIR_R, &ev, &md);
    ev.ret = ev.func(ev.dirp, ev.entry, ev.result);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_READDIR_R, &ev, &md);
    return ev.ret;
}

INTERPOSE(void, rewinddir, DIR *dirp)
{
    struct rewinddir_event ev = {
        .pc   = INTERPOSE_PC,
        .dirp = dirp,
        .func = REAL_FUNC(rewinddir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_REWINDDIR, &ev, &md);
    ev.func(ev.dirp);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_REWINDDIR, &ev, &md);
}

INTERPOSE(void, seekdir, DIR *dirp, long loc)
{
    struct seekdir_event ev = {
        .pc   = INTERPOSE_PC,
        .dirp = dirp,
        .loc  = loc,
        .func = REAL_FUNC(seekdir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_SEEKDIR, &ev, &md);
    ev.func(ev.dirp, ev.loc);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_SEEKDIR, &ev, &md);
}

INTERPOSE(long, telldir, DIR *dirp)
{
    struct telldir_event ev = {
        .pc   = INTERPOSE_PC,
        .dirp = dirp,
        .ret  = 0,
        .func = REAL_FUNC(telldir),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_TELLDIR, &ev, &md);
    ev.ret = ev.func(ev.dirp);
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_TELLDIR, &ev, &md);
    return ev.ret;
}

INTERPOSE(
    int,
    scandir,
    const char *dirp,
    struct dirent ***namelist,
    int (*filter)(const struct dirent *),
    int (*compar)(
        const struct dirent **,
        const struct dirent **
    )
)
{
    struct scandir_event ev = {
        .pc       = INTERPOSE_PC,
        .dirp     = dirp,
        .namelist = namelist,
        .filter   = filter,
        .compar   = compar,
        .ret      = 0,
        .errno_   = 0,
        .func     = REAL_FUNC(scandir),
    };

    metadata_t md = {0};

    PS_PUBLISH(
        INTERCEPT_BEFORE,
        EVENT_SCANDIR,
        &ev,
        &md
    );

    errno = 0;
    ev.ret = ev.func(
        ev.dirp,
        ev.namelist,
        ev.filter,
        ev.compar
    );
    ev.errno_ = errno;

    PS_PUBLISH(
        INTERCEPT_AFTER,
        EVENT_SCANDIR,
        &ev,
        &md
    );

    errno = ev.errno_;

    return ev.ret;
}


INTERPOSE(
    int,
    scandirat,
    int dirfd,
    const char *dirp,
    struct dirent ***namelist,
    int (*filter)(const struct dirent *),
    int (*compar)(
        const struct dirent **,
        const struct dirent **
    )
)
{
    struct scandirat_event ev = {
        .pc       = INTERPOSE_PC,
        .dirfd    = dirfd,
        .dirp     = dirp,
        .namelist = namelist,
        .filter   = filter,
        .compar   = compar,
        .ret      = 0,
        .errno_   = 0,
        .func     = REAL_FUNC(scandirat),
    };

    metadata_t md = {0};

    PS_PUBLISH(
        INTERCEPT_BEFORE,
        EVENT_SCANDIRAT,
        &ev,
        &md
    );

    errno = 0;
    ev.ret = ev.func(
        ev.dirfd,
        ev.dirp,
        ev.namelist,
        ev.filter,
        ev.compar
    );
    ev.errno_ = errno;

    PS_PUBLISH(
        INTERCEPT_AFTER,
        EVENT_SCANDIRAT,
        &ev,
        &md
    );

    errno = ev.errno_;

    return ev.ret;
}

INTERPOSE(
    int,
    scandir64,
    const char *dirp,
    struct dirent64 ***namelist,
    int (*filter)(const struct dirent64 *),
    int (*compar)(
        const struct dirent64 **,
        const struct dirent64 **
    )
)
{
    struct scandir64_event ev = {
        .pc       = INTERPOSE_PC,
        .dirp     = dirp,
        .namelist = namelist,
        .filter   = filter,
        .compar   = compar,
        .ret      = 0,
        .errno_   = 0,
        .func     = REAL_FUNC(scandir64),
    };

    metadata_t md = {0};

    PS_PUBLISH(
        INTERCEPT_BEFORE,
        EVENT_SCANDIR64,
        &ev,
        &md
    );

    errno = 0;
    ev.ret = ev.func(
        ev.dirp,
        ev.namelist,
        ev.filter,
        ev.compar
    );
    ev.errno_ = errno;

    PS_PUBLISH(
        INTERCEPT_AFTER,
        EVENT_SCANDIR64,
        &ev,
        &md
    );

    errno = ev.errno_;

    return ev.ret;
}


INTERPOSE(
    int,
    scandirat64,
    int dirfd,
    const char *dirp,
    struct dirent64 ***namelist,
    int (*filter)(const struct dirent64 *),
    int (*compar)(
        const struct dirent64 **,
        const struct dirent64 **
    )
)
{
    struct scandirat64_event ev = {
        .pc       = INTERPOSE_PC,
        .dirfd    = dirfd,
        .dirp     = dirp,
        .namelist = namelist,
        .filter   = filter,
        .compar   = compar,
        .ret      = 0,
        .errno_   = 0,
        .func     = REAL_FUNC(scandirat64),
    };

    metadata_t md = {0};

    PS_PUBLISH(
        INTERCEPT_BEFORE,
        EVENT_SCANDIRAT64,
        &ev,
        &md
    );

    errno = 0;
    ev.ret = ev.func(
        ev.dirfd,
        ev.dirp,
        ev.namelist,
        ev.filter,
        ev.compar
    );
    ev.errno_ = errno;

    PS_PUBLISH(
        INTERCEPT_AFTER,
        EVENT_SCANDIRAT64,
        &ev,
        &md
    );

    errno = ev.errno_;

    return ev.ret;
}

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(struct dirent64 *, readdir64, DIR *dirp)
{
    int saved_errno = errno;
    struct readdir64_event ev = {
        .pc     = INTERPOSE_PC,
        .dirp   = dirp,
        .ret    = NULL,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(readdir64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_READDIR64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirp);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_READDIR64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
INTERPOSE(int, readdir64_r, DIR *dirp, struct dirent64 *entry, struct dirent64 **result)
{
    int saved_errno = errno;
    struct readdir64_r_event ev = {
        .pc     = INTERPOSE_PC,
        .dirp   = dirp,
        .entry  = entry,
        .result = result,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(readdir64_r),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_READDIR64_R, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.dirp, ev.entry, ev.result);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_READDIR64_R, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif

#if defined(__linux__) && defined(__GLIBC__)
#if __GLIBC_PREREQ(2, 30)
INTERPOSE(ssize_t, getdents64, int fd, void *buf, size_t nbytes)
{
    int saved_errno = errno;
    struct getdents64_event ev = {
        .pc     = INTERPOSE_PC,
        .fd     = fd,
        .buf    = buf,
        .nbytes = nbytes,
        .ret    = 0,
        .errno_ = saved_errno,
        .func   = REAL_FUNC(getdents64),
    };

    metadata_t md = {0};
    PS_PUBLISH(INTERCEPT_BEFORE, EVENT_GETDENTS64, &ev, &md);
    errno = ev.errno_;
    ev.ret = ev.func(ev.fd, ev.buf, ev.nbytes);
    ev.errno_ = errno;
    PS_PUBLISH(INTERCEPT_AFTER, EVENT_GETDENTS64, &ev, &md);
    errno = ev.errno_;
    return ev.ret;
}
#endif
#endif

/* Advertise event types */
PS_ADVERTISE_TYPE(EVENT_CLOSEDIR)
PS_ADVERTISE_TYPE(EVENT_FDOPENDIR)
PS_ADVERTISE_TYPE(EVENT_OPENDIR)
PS_ADVERTISE_TYPE(EVENT_READDIR)
PS_ADVERTISE_TYPE(EVENT_READDIR_R)
PS_ADVERTISE_TYPE(EVENT_REWINDDIR)
PS_ADVERTISE_TYPE(EVENT_SEEKDIR)
PS_ADVERTISE_TYPE(EVENT_TELLDIR)
PS_ADVERTISE_TYPE(EVENT_SCANDIR)
PS_ADVERTISE_TYPE(EVENT_SCANDIRAT)
PS_ADVERTISE_TYPE(EVENT_SCANDIR64)
PS_ADVERTISE_TYPE(EVENT_SCANDIRAT64)
PS_ADVERTISE_TYPE(EVENT_READDIR64)
PS_ADVERTISE_TYPE(EVENT_READDIR64_R)
PS_ADVERTISE_TYPE(EVENT_GETDENTS64)

/* Module init (optional) */
DICE_MODULE_INIT()
