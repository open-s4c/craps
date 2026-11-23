/*
 * Copyright (C) 2026 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_UNISTD_H
#define DICE_UNISTD_H

#include <unistd.h>

#ifndef DICE_ROLLED
#define EVENT_ACCESS               137
#endif
#ifndef DICE_ROLLED
#define EVENT_ALARM                138
#endif
#ifndef DICE_ROLLED
#define EVENT_BRK                  139
#endif
#ifndef DICE_ROLLED
#define EVENT_CHDIR                140
#endif
#ifndef DICE_ROLLED
#define EVENT_CHROOT               141
#endif
#ifndef DICE_ROLLED
#define EVENT_CHOWN                142
#endif
#ifndef DICE_ROLLED
#define EVENT_CLOSE                143
#endif
#ifndef DICE_ROLLED
#define EVENT_CONFSTR              144
#endif
#ifndef DICE_ROLLED
#define EVENT_CRYPT                145
#endif
#ifndef DICE_ROLLED
#define EVENT_CTERMID              146
#endif
#ifndef DICE_ROLLED
#define EVENT_CUSERID              147
#endif
#ifndef DICE_ROLLED
#define EVENT_DUP                  148
#endif
#ifndef DICE_ROLLED
#define EVENT_DUP2                 149
#endif
#ifndef DICE_ROLLED
#define EVENT_DUP3                 150
#endif
#ifndef DICE_ROLLED
#define EVENT__EXIT                151
#endif
#ifndef DICE_ROLLED
#define EVENT_FACCESSAT            152
#endif
#ifndef DICE_ROLLED
#define EVENT_FCHDIR               153
#endif
#ifndef DICE_ROLLED
#define EVENT_FCHOWN               154
#endif
#ifndef DICE_ROLLED
#define EVENT_FCHOWNAT             155
#endif
#ifndef DICE_ROLLED
#define EVENT_GETUID               156
#endif
#ifndef DICE_ROLLED
#define EVENT_FDATASYNC            157
#endif
#ifndef DICE_ROLLED
#define EVENT_FORK                 158
#endif
#ifndef DICE_ROLLED
#define EVENT_FPATHCONF            159
#endif
#ifndef DICE_ROLLED
#define EVENT_FSYNC                160
#endif
#ifndef DICE_ROLLED
#define EVENT_FTRUNCATE            161
#endif
#ifndef DICE_ROLLED
#define EVENT_GETCWD               162
#endif
#ifndef DICE_ROLLED
#define EVENT_GETDTABLESIZE        163
#endif
#ifndef DICE_ROLLED
#define EVENT_GETEGID              164
#endif
#ifndef DICE_ROLLED
#define EVENT_GETEUID              165
#endif
#ifndef DICE_ROLLED
#define EVENT_GETGID               166
#endif
#ifndef DICE_ROLLED
#define EVENT_GETGROUPS            167
#endif
#ifndef DICE_ROLLED
#define EVENT_GETHOSTID            168
#endif
#ifndef DICE_ROLLED
#define EVENT_GETLOGIN             169
#endif
#ifndef DICE_ROLLED
#define EVENT_GETLOGIN_R           170
#endif
#ifndef DICE_ROLLED
#define EVENT_GETOPT               171
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPAGESIZE          172
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPASS              173
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPGID              174
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPGRP              175
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPID               176
#endif
#ifndef DICE_ROLLED
#define EVENT_GETPPID              177
#endif
#ifndef DICE_ROLLED
#define EVENT_GETSID               178
#endif
#ifndef DICE_ROLLED
#define EVENT_GETWD                179
#endif
#ifndef DICE_ROLLED
#define EVENT_GET_CURRENT_DIR_NAME 180
#endif
#ifndef DICE_ROLLED
#define EVENT_ISATTY               181
#endif
#ifndef DICE_ROLLED
#define EVENT_LCHOWN               182
#endif
#ifndef DICE_ROLLED
#define EVENT_LINK                 183
#endif
#ifndef DICE_ROLLED
#define EVENT_LINKAT               184
#endif
#ifndef DICE_ROLLED
#define EVENT_LOCKF                185
#endif
#ifndef DICE_ROLLED
#define EVENT_LSEEK                186
#endif
#ifndef DICE_ROLLED
#define EVENT_NICE                 187
#endif
#ifndef DICE_ROLLED
#define EVENT_PATHCONF             188
#endif
#ifndef DICE_ROLLED
#define EVENT_PAUSE                189
#endif
#ifndef DICE_ROLLED
#define EVENT_PIPE                 190
#endif
#ifndef DICE_ROLLED
#define EVENT_PIPE2                191
#endif
#ifndef DICE_ROLLED
#define EVENT_PREAD                192
#endif
#ifndef DICE_ROLLED
#define EVENT_PWRITE               193
#endif
#ifndef DICE_ROLLED
#define EVENT_READ                 194
#endif
#ifndef DICE_ROLLED
#define EVENT_READLINK             195
#endif
#ifndef DICE_ROLLED
#define EVENT_READLINKAT           196
#endif
#ifndef DICE_ROLLED
#define EVENT_RMDIR                197
#endif
#ifndef DICE_ROLLED
#define EVENT_SBRK                 198
#endif
#ifndef DICE_ROLLED
#define EVENT_SETGID               199
#endif
#ifndef DICE_ROLLED
#define EVENT_SETHOSTID            200
#endif
#ifndef DICE_ROLLED
#define EVENT_SETPGID              201
#endif
#ifndef DICE_ROLLED
#define EVENT_SETPGRP              202
#endif
#ifndef DICE_ROLLED
#define EVENT_SETREGID             203
#endif
#ifndef DICE_ROLLED
#define EVENT_SETREUID             204
#endif
#ifndef DICE_ROLLED
#define EVENT_SETSID               205
#endif
#ifndef DICE_ROLLED
#define EVENT_SETUID               206
#endif
#ifndef DICE_ROLLED
#define EVENT_SLEEP                207
#endif
#ifndef DICE_ROLLED
#define EVENT_SWAB                 208
#endif
#ifndef DICE_ROLLED
#define EVENT_SYMLINK              209
#endif
#ifndef DICE_ROLLED
#define EVENT_SYMLINKAT            210
#endif
#ifndef DICE_ROLLED
#define EVENT_SYNC                 211
#endif
#ifndef DICE_ROLLED
#define EVENT_SYNCFS               212
#endif
#ifndef DICE_ROLLED
#define EVENT_SYSCONF              213
#endif
#ifndef DICE_ROLLED
#define EVENT_TCGETPGRP            214
#endif
#ifndef DICE_ROLLED
#define EVENT_TCSETPGRP            215
#endif
#ifndef DICE_ROLLED
#define EVENT_TRUNCATE             216
#endif
#ifndef DICE_ROLLED
#define EVENT_TTYNAME              217
#endif
#ifndef DICE_ROLLED
#define EVENT_TTYNAME_R            218
#endif
#ifndef DICE_ROLLED
#define EVENT_UALARM               219
#endif
#ifndef DICE_ROLLED
#define EVENT_UNLINK               220
#endif
#ifndef DICE_ROLLED
#define EVENT_UNLINKAT             221
#endif
#ifndef DICE_ROLLED
#define EVENT_USLEEP               222
#endif
#ifndef DICE_ROLLED
#define EVENT_VFORK                223
#endif
#ifndef DICE_ROLLED
#define EVENT_WRITE                224
#endif
#ifndef DICE_ROLLED
#define EVENT_SYSCALL              261
#endif

struct access_event {
    const void *pc;
    const char *path;
    int mode;
    int ret;
    int errno_;
    int (*func)(const char *, int);
};

struct alarm_event {
    const void *pc;
    unsigned int seconds;
    unsigned int ret;
    unsigned int (*func)(unsigned int);
};

struct brk_event {
    const void *pc;
    void *addr;
    int ret;
    int errno_;
    int (*func)(void *);
};

struct chdir_event {
    const void *pc;
    const char *path;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct chroot_event {
    const void *pc;
    const char *path;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct chown_event {
    const void *pc;
    const char *path;
    uid_t owner;
    gid_t group;
    int ret;
    int errno_;
    int (*func)(const char *, uid_t, gid_t);
};

struct close_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct confstr_event {
    const void *pc;
    int name;
    char *buf;
    size_t size;
    size_t ret;
    int errno_;
    size_t (*func)(int, char *, size_t);
};

struct crypt_event {
    const void *pc;
    const char *key;
    const char *salt;
    char *ret;
    int errno_;
    char *(*func)(const char *, const char *);
};

struct ctermid_event {
    const void *pc;
    char *s;
    char *ret;
    char *(*func)(char *);
};

struct cuserid_event {
    const void *pc;
    char *string;
    char *ret;
    char *(*func)(char *);
};

struct dup_event {
    const void *pc;
    int oldfd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct dup2_event {
    const void *pc;
    int oldfd;
    int newfd;
    int ret;
    int errno_;
    int (*func)(int, int);
};

struct dup3_event {
    const void *pc;
    int oldfd;
    int newfd;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, int, int);
};

struct _exit_event {
    const void *pc;
    int status;
    void (*func)(int);
};

struct faccessat_event {
    const void *pc;
    int dirfd;
    const char *path;
    int mode;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, int, int);
};

struct fchdir_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct fchown_event {
    const void *pc;
    int fildes;
    uid_t owner;
    gid_t group;
    int ret;
    int errno_;
    int (*func)(int, uid_t, gid_t);
};

struct fchownat_event {
    const void *pc;
    int fd;
    const char *path;
    uid_t owner;
    gid_t group;
    int flag;
    int ret;
    int errno_;
    int (*func)(int, const char *, uid_t, gid_t, int);
};

struct getuid_event {
    const void *pc;
    uid_t ret;
    uid_t (*func)(void);
};

struct fdatasync_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct fork_event {
    const void *pc;
    pid_t ret;
    int errno_;
    pid_t (*func)(void);
};

struct fpathconf_event {
    const void *pc;
    int fd;
    int name;
    long ret;
    int errno_;
    long (*func)(int, int);
};

struct fsync_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct ftruncate_event {
    const void *pc;
    int fildes;
    off_t length;
    int ret;
    int errno_;
    int (*func)(int, off_t);
};

struct getcwd_event {
    const void *pc;
    size_t size;
    char *buf;
    char *ret;
    int errno_;
    char *(*func)(char *, size_t);
};

struct getdtablesize_event {
    const void *pc;
    int ret;
    int errno_;
    int (*func)(void);
};

struct getegid_event {
    const void *pc;
    gid_t ret;
    gid_t (*func)(void);
};

struct geteuid_event {
    const void *pc;
    uid_t ret;
    uid_t (*func)(void);
};

struct getgid_event {
    const void *pc;
    gid_t ret;
    gid_t (*func)(void);
};

struct getgroups_event {
    const void *pc;
    int size;
    gid_t *list;
    int ret;
    int errno_;
    int (*func)(int, gid_t *);
};

struct gethostid_event {
    const void *pc;
    long ret;
    long (*func)(void);
};

struct getlogin_event {
    const void *pc;
    char *ret;
    int errno_;
    char *(*func)(void);
};

struct getlogin_r_event {
    const void *pc;
    char *buf;
    size_t bufsize;
    int ret;
    int (*func)(char *, size_t);
};

struct getopt_event {
    const void *pc;
    int argc;
    char * const *argv;
    const char *optstring;
    int ret;
    int (*func)(int, char * const *, const char *);
};

struct getpagesize_event {
    const void *pc;
    int ret;
    int (*func)(void);
};

struct getpass_event {
    const void *pc;
    const char *prompt;
    char *ret;
    int errno_;
    char *(*func)(const char *);
};

struct getpgid_event {
    const void *pc;
    pid_t pid;
    pid_t ret;
    int errno_;
    pid_t (*func)(pid_t);
};

struct getpgrp_event {
    const void *pc;
    pid_t ret;
    pid_t (*func)(void);
};

struct getpid_event {
    const void *pc;
    pid_t ret;
    pid_t (*func)(void);
};

struct getppid_event {
    const void *pc;
    pid_t ret;
    pid_t (*func)(void);
};

struct getsid_event {
    const void *pc;
    pid_t pid;
    pid_t ret;
    int errno_;
    pid_t (*func)(pid_t);
};

struct getwd_event {
    const void *pc;
    char *buf;
    char *ret;
    int errno_;
    char *(*func)(char *);
};

struct get_current_dir_name_event {
    const void *pc;
    char *ret;
    int errno_;
    char *(*func)(void);
};

struct isatty_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct lchown_event {
    const void *pc;
    const char *path;
    uid_t owner;
    gid_t group;
    int ret;
    int errno_;
    int (*func)(const char *, uid_t, gid_t);
};

struct link_event {
    const void *pc;
    const char *oldpath;
    const char *newpath;
    int ret;
    int errno_;
    int (*func)(const char *, const char *);
};

struct linkat_event {
    const void *pc;
    int olddirfd;
    const char *oldpath;
    int newdirfd;
    const char *newpath;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, int, const char *, int);
};

struct lockf_event {
    const void *pc;
    int fd;
    int op;
    off_t size;
    int ret;
    int errno_;
    int (*func)(int, int, off_t);
};

struct lseek_event {
    const void *pc;
    int fd;
    off_t offset;
    int whence;
    off_t ret;
    int errno_;
    off_t (*func)(int, off_t, int);
};

struct nice_event {
    const void *pc;
    int inc;
    int ret;
    int errno_;
    int (*func)(int);
};

struct pathconf_event {
    const void *pc;
    const char *path;
    int name;
    long ret;
    int errno_;
    long (*func)(const char *, int);
};

struct pause_event {
    const void *pc;
    int ret;
    int errno_;
    int (*func)(void);
};

struct pipe_event {
    const void *pc;
    int *pipefd;
    int ret;
    int errno_;
    int (*func)(int[2]);
};

struct pipe2_event {
    const void *pc;
    int *pipefd;
    int flags;
    int ret;
    int errno_;
    int (*func)(int[2], int);
};

struct pread_event {
    const void *pc;
    int fd;
    void *buf;
    size_t count;
    off_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, void *, size_t, off_t);
};

struct pwrite_event {
    const void *pc;
    int fd;
    const void *buf;
    size_t count;
    off_t offset;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const void *, size_t, off_t);
};

struct read_event {
    const void *pc;
    int fd;
    void *buf;
    size_t count;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, void *, size_t);
};

struct readlink_event {
    const void *pc;
    const char *path;
    char *buf;
    size_t bufsiz;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(const char *, char *, size_t);
};

struct readlinkat_event {
    const void *pc;
    int dirfd;
    const char *path;
    char *buf;
    size_t bufsiz;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const char *, char *, size_t);
};

struct rmdir_event {
    const void *pc;
    const char *path;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct sbrk_event {
    const void *pc;
    intptr_t increment;
    void *ret;
    int errno_;
    void *(*func)(intptr_t);
};

struct setgid_event {
    const void *pc;
    gid_t gid;
    int ret;
    int errno_;
    int (*func)(gid_t);
};

struct sethostid_event {
    const void *pc;
    long hostid;
    int ret;
    int errno_;
    int (*func)(long);
};

struct setpgid_event {
    const void *pc;
    pid_t pid;
    pid_t pgid;
    int ret;
    int errno_;
    int (*func)(pid_t, pid_t);
};

struct setpgrp_event {
    const void *pc;
    pid_t ret;
    int errno_;
    pid_t (*func)(void);
};

struct setregid_event {
    const void *pc;
    gid_t rgid;
    gid_t egid;
    int ret;
    int errno_;
    int (*func)(gid_t, gid_t);
};

struct setreuid_event {
    const void *pc;
    uid_t ruid;
    uid_t euid;
    int ret;
    int errno_;
    int (*func)(uid_t, uid_t);
};

struct setsid_event {
    const void *pc;
    pid_t ret;
    int errno_;
    pid_t (*func)(void);
};

struct setuid_event {
    const void *pc;
    uid_t uid;
    int ret;
    int errno_;
    int (*func)(uid_t);
};

struct sleep_event {
    const void *pc;
    unsigned int seconds;
    unsigned int ret;
    unsigned int (*func)(unsigned int);
};

struct swab_event {
    const void *pc;
    ssize_t n;
    const void *from;
    void *to;
    void (*func)(const void *, void *, ssize_t);
};

struct symlink_event {
    const void *pc;
    const char *target;
    const char *linkpath;
    int ret;
    int errno_;
    int (*func)(const char *, const char *);
};

struct symlinkat_event {
    const void *pc;
    const char *target;
    int newdirfd;
    const char *linkpath;
    int ret;
    int errno_;
    int (*func)(const char *, int, const char *);
};

struct sync_event {
    const void *pc;
    void (*func)(void);
};

struct syncfs_event {
    const void *pc;
    int fd;
    int ret;
    int errno_;
    int (*func)(int);
};

struct sysconf_event {
    const void *pc;
    int name;
    long ret;
    int errno_;
    long (*func)(int);
};

struct tcgetpgrp_event {
    const void *pc;
    int fd;
    pid_t ret;
    int errno_;
    pid_t (*func)(int);
};

struct tcsetpgrp_event {
    const void *pc;
    int fd;
    pid_t pgrp;
    int ret;
    int errno_;
    int (*func)(int, pid_t);
};

struct truncate_event {
    const void *pc;
    const char *path;
    off_t length;
    int ret;
    int errno_;
    int (*func)(const char *, off_t);
};

struct ttyname_event {
    const void *pc;
    int fd;
    char *ret;
    int errno_;
    char *(*func)(int);
};

struct ttyname_r_event {
    const void *pc;
    int fd;
    char *buf;
    size_t size;
    int ret;
    int (*func)(int, char *, size_t);
};

struct ualarm_event {
    const void *pc;
    useconds_t usecs;
    useconds_t interval;
    useconds_t ret;
    int errno_;
    useconds_t (*func)(useconds_t, useconds_t);
};

struct unlink_event {
    const void *pc;
    const char *path;
    int ret;
    int errno_;
    int (*func)(const char *);
};

struct unlinkat_event {
    const void *pc;
    int dirfd;
    const char *path;
    int flags;
    int ret;
    int errno_;
    int (*func)(int, const char *, int);
};

struct usleep_event {
    const void *pc;
    useconds_t usec;
    int ret;
    int errno_;
    int (*func)(useconds_t);
};

struct vfork_event {
    const void *pc;
    pid_t ret;
    int errno_;
    pid_t (*func)(void);
};

struct write_event {
    const void *pc;
    int fd;
    const void *buf;
    size_t count;
    ssize_t ret;
    int errno_;
    ssize_t (*func)(int, const void *, size_t);
};

struct syscall_event {
    const void *pc;
    long number;
    long args[6];
    long ret;
    int errno_;
    int (*func)(long number, ...);
};

#endif /* DICE_UNISTD_H */
