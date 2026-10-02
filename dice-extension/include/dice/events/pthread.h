/*
 * Copyright (C) 2025 Huawei Technologies Co., Ltd.
 * SPDX-License-Identifier: 0BSD
 */
#ifndef DICE_PTHREAD_EXT_H
#define DICE_PTHREAD_EXT_H

#include <pthread.h>

#include <dice/events/thread.h>

#define EVENT_BEFORE_PTHREAD_ONCE 264
#define EVENT_AFTER_PTHREAD_ONCE  265
#define EVENT_PTHREAD_KEY_CREATE   275
#define EVENT_PTHREAD_KEY_DELETE   276
#define EVENT_PTHREAD_SETSPECIFIC  277
#define EVENT_PTHREAD_GETSPECIFIC  278
#define EVENT_PTHREAD_DETACH       279

struct before_pthread_once_event {
    const void *pc;
    pthread_once_t *once_control;
    void (*init_routine)(void);
    int ret;
    int(*func)(pthread_once_t *, void (*)(void));
};

struct after_pthread_once_event {
    const void *pc;
    pthread_once_t *once_control;
    void (*init_routine)(void);
    int ret;
    int(*func)(pthread_once_t *, void (*)(void));
};

struct pthread_key_create_event {
    const void *pc;
    pthread_key_t *key;
    void (*destr_function)(void *);
    int ret;
    int (*func)(pthread_key_t *, void (*)(void *));
};

struct pthread_key_delete_event {
    const void *pc;
    pthread_key_t key;
    int ret;
    int (*func)(pthread_key_t);
};

struct pthread_setspecific_event {
    const void *pc;
    pthread_key_t key;
    const void *pointer;
    int ret;
    int (*func)(pthread_key_t, const void *);
};

struct pthread_getspecific_event {
    const void *pc;
    pthread_key_t key;
    void *ret;
    void *(*func)(pthread_key_t);
};

struct pthread_detach_event {
    const void *pc;
    pthread_t thread;
    int ret;
    int (*func)(pthread_t);
};

#endif /* DICE_PTHREAD_H */
