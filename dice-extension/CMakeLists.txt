# Craps contributes Roll module catalogs only.  Dice Roll compiles each
# source into the configured dice-roll-runtime object target with the generated
# header and module slot.  No legacy preload libraries or dispatch tables are
# built from this extension.

if(NOT DICE_ROLL_CONFIG)
  message(FATAL_ERROR
          "Craps interceptors require DICE_ROLL_CONFIG (Dice Roll only)")
endif()

set(EVENTS_INCLUDE_DIR "${CMAKE_CURRENT_LIST_DIR}/include")
set(EVENTS_MODULE_DIR "${CMAKE_CURRENT_LIST_DIR}/src/mod")

function(craps_roll_module MODULE)
  set(sources "${EVENTS_MODULE_DIR}/${MODULE}.c")
  if(MODULE STREQUAL "syscall")
    list(APPEND sources "${EVENTS_MODULE_DIR}/syscall.S")
  endif()

  # The catalog is intentionally excluded from ALL. Its SOURCES, SOURCE_DIR
  # and INCLUDE_DIRECTORIES are consumed by add_dice_mods(), which compiles
  # those sources into a single generated runtime.
  add_library(dice-${MODULE}.o OBJECT EXCLUDE_FROM_ALL ${sources})
  target_include_directories(
    dice-${MODULE}.o PRIVATE "${EVENTS_INCLUDE_DIR}")
  set_target_properties(dice-${MODULE}.o PROPERTIES
                        DICE_MODULE_OPTIONS "")
endfunction()

foreach(module
    dirent fcntl ioctl mman_ext poll pthread_detach pthread_key pthread_once
    socket stat stdio time uio unistd)
  craps_roll_module(${module})
endforeach()

if(NOT APPLE)
  craps_roll_module(eventfd)
  craps_roll_module(random)
endif()

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  craps_roll_module(epoll)
  craps_roll_module(syscall)
endif()


# Generated interposition behavior tests stay enabled for Roll builds.
include("${CMAKE_CURRENT_LIST_DIR}/cmake/RollInterposeTests.cmake")

# ------------------------------------------------------------------------------
# Tests
# ------------------------------------------------------------------------------

if(DICE_TESTS)
  include(CheckSymbolExists)

  # --------------------------------------------------------------------------
  # Platform-specific interfaces used by the extracted interpose tests
  # --------------------------------------------------------------------------

  check_symbol_exists(epoll_pwait2 "sys/epoll.h" HAVE_EPOLL_PWAIT2)
  check_symbol_exists(getdate "time.h" HAVE_GETDATE)
  check_symbol_exists(getdate_r "time.h" HAVE_GETDATE_R)
  check_symbol_exists(settimeofday "sys/time.h" HAVE_SETTIMEOFDAY)
  check_symbol_exists(gethostid "unistd.h" HAVE_GETHOSTID)
  check_symbol_exists(sethostid "unistd.h" HAVE_SETHOSTID)
  check_symbol_exists(getpass "unistd.h" HAVE_GETPASS)
  check_symbol_exists(getwd "unistd.h" HAVE_GETWD)
  check_symbol_exists(
    get_current_dir_name
    "unistd.h"
    HAVE_GET_CURRENT_DIR_NAME)
  check_symbol_exists(cuserid "stdio.h" HAVE_CUSERID)
  check_symbol_exists(ualarm "unistd.h" HAVE_UALARM)

  set(
    MORE_EVENTS_TEST_SIGNATURES
    "${CMAKE_CURRENT_LIST_DIR}/test/interpose/interposed_.h.in")

  set(
    MORE_EVENTS_TEST_INCLUDE_DIR
    "${CMAKE_CURRENT_LIST_DIR}/include")

  # Small wrapper around Dice's generic test generator.
  #
  # The old more-events CMake linked libcrypt to every interpose test on
  # non-Apple systems and suppressed deprecation warnings for every generated
  # test. Preserve that behavior here.
  function(more_events_add_interpose_test MODULE HEADER)
    set(EXTRA_ARGS
        MAPPINGS
          "${MORE_EVENTS_TEST_SIGNATURES}"
        INCLUDE_DIRECTORIES
          "${MORE_EVENTS_TEST_INCLUDE_DIR}"
        COMPILE_OPTIONS
          -Wno-deprecated-declarations)

    if(NOT APPLE)
      list(
        APPEND
        EXTRA_ARGS
        LINK_LIBRARIES
          crypt)
    endif()

    craps_roll_add_interpose_test(
      ${MODULE}
      HEADER "${HEADER}"
      FUNCTIONS ${ARGN}
      ${EXTRA_ARGS})
  endfunction()


  # --------------------------------------------------------------------------
  # random
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    more_events_add_interpose_test(
      random
      "<dice/events/random.h>"
      getrandom)
  endif()


  # --------------------------------------------------------------------------
  # poll
  # --------------------------------------------------------------------------

  if(NOT (
      CMAKE_C_COMPILER_ID STREQUAL "GNU"
      AND CMAKE_BUILD_TYPE STREQUAL "Release"))
    more_events_add_interpose_test(
      poll
      "<dice/events/poll.h>"
      poll)
  endif()


  # --------------------------------------------------------------------------
  # epoll
  # --------------------------------------------------------------------------

  if(NOT APPLE AND NOT BSD)
    set(
      EPOLL_FUNCTIONS
      epoll_create
      epoll_create1
      epoll_ctl
      epoll_wait
      epoll_pwait)

    if(HAVE_EPOLL_PWAIT2 AND NOT MUSL)
      list(APPEND EPOLL_FUNCTIONS epoll_pwait2)
    endif()

    more_events_add_interpose_test(
      epoll
      "<dice/events/epoll.h>"
      ${EPOLL_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # eventfd
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    more_events_add_interpose_test(
      eventfd
      "<dice/events/eventfd.h>"
      eventfd)
  endif()


  # --------------------------------------------------------------------------
  # fcntl.h
  # --------------------------------------------------------------------------

  if(NOT APPLE)
    set(
      FCNTL_FUNCTIONS
      creat
      # fcntl
      # openat
      posix_fadvise
      posix_fallocate)

    if(NOT (
        CMAKE_C_COMPILER_ID STREQUAL "GNU"
        AND CMAKE_BUILD_TYPE STREQUAL "Release"))
      list(APPEND FCNTL_FUNCTIONS open)
    endif()

    more_events_add_interpose_test(
      fcntl
      "<dice/events/fcntl.h>"
      ${FCNTL_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # socket
  # --------------------------------------------------------------------------

  set(
    SOCKET_FUNCTIONS
    accept
    bind
    connect
    getpeername
    getsockname
    getsockopt
    setsockopt
    listen
    recv
    recvfrom
    recvmsg
    send
    sendto
    sendmsg
    shutdown
    socket
    socketpair)

  if(NOT APPLE)
    list(APPEND SOCKET_FUNCTIONS accept4)
  endif()

  more_events_add_interpose_test(
    socket
    "<dice/events/socket.h>"
    ${SOCKET_FUNCTIONS})


  # --------------------------------------------------------------------------
  # time.h
  # --------------------------------------------------------------------------

  if(NOT (
      CMAKE_C_COMPILER_ID STREQUAL "Clang"
      AND CMAKE_BUILD_TYPE STREQUAL "Release")
     AND NOT BSD)

    set(
      TIME_FUNCTIONS
      asctime
      asctime_r
      clock
      time
      clock_getres
      clock_gettime
      clock_settime
      ctime
      ctime_r
      difftime
      gmtime
      gmtime_r
      localtime
      localtime_r
      mktime
      nanosleep
      strftime
      strptime
      getitimer
      setitimer
      gettimeofday
      select
      tzset
      utimes)

    if(HAVE_GETDATE AND NOT BSD)
      list(APPEND TIME_FUNCTIONS getdate)
    endif()

    if(HAVE_SETTIMEOFDAY AND NOT BSD)
      list(APPEND TIME_FUNCTIONS settimeofday)
    endif()

    if(NOT APPLE)
      list(
        APPEND
        TIME_FUNCTIONS
        timer_create
        timer_delete
        timer_gettime
        timer_getoverrun
        timer_settime
        strftime_l)
    endif()

    if(HAVE_GETDATE_R)
      list(APPEND TIME_FUNCTIONS getdate_r)
    endif()

    more_events_add_interpose_test(
      time
      "<dice/events/time.h>"
      ${TIME_FUNCTIONS})
  endif()


  # --------------------------------------------------------------------------
  # uio.h
  # --------------------------------------------------------------------------

  if(NOT APPLE AND NOT BSD)
    more_events_add_interpose_test(
      uio
      "<dice/events/uio.h>"
      readv
      writev)
  endif()


  # --------------------------------------------------------------------------
  # unistd.h
  # --------------------------------------------------------------------------

  if(NOT CMAKE_BUILD_TYPE STREQUAL "Release")
    set(
      UNISTD_FUNCTIONS
      getdtablesize
      getegid
      geteuid
      write
      getgid
      getgroups
      getlogin
      getpagesize
      getpgrp
      getpid
      getppid
      getuid
      fork
      pause
      # pipe
      setsid
      sync
      access
      alarm
      chdir
      chown
      chroot
      close
      confstr
      ctermid
      dup
      dup2
      faccessat
      fchdir
      fchown
      fchownat
      fpathconf
      fsync
      ftruncate
      getcwd
      getlogin_r
      getopt
      getpgid
      getsid
      isatty
      lchown
      link
      linkat
      lockf
      lseek
      nice
      pathconf
      pread
      pwrite
      read
      readlink
      readlinkat
      rmdir
      setgid
      setpgid
      setregid
      setreuid
      setuid
      sleep
      swab
      symlink
      symlinkat
      sysconf
      tcgetpgrp
      tcsetpgrp
      truncate
      ttyname
      ttyname_r
      unlink
      unlinkat
      usleep)

    if(NOT APPLE)
      list(
        APPEND
        UNISTD_FUNCTIONS
        brk
        crypt
        fdatasync
        # pipe2
        sbrk
        vfork)
    endif()

    if(NOT APPLE AND NOT BSD)
      list(
        APPEND
        UNISTD_FUNCTIONS
        setpgrp
        syncfs
        dup3)
    endif()

    if(HAVE_GETHOSTID)
      list(APPEND UNISTD_FUNCTIONS gethostid)
    endif()

    if(HAVE_SETHOSTID AND NOT APPLE AND NOT BSD)
      list(APPEND UNISTD_FUNCTIONS sethostid)
    endif()

    if(HAVE_GETPASS)
      list(APPEND UNISTD_FUNCTIONS getpass)
    endif()

    if(HAVE_GETWD)
      list(APPEND UNISTD_FUNCTIONS getwd)
    endif()

    if(HAVE_GET_CURRENT_DIR_NAME)
      list(APPEND UNISTD_FUNCTIONS get_current_dir_name)
    endif()

    if(HAVE_CUSERID)
      list(APPEND UNISTD_FUNCTIONS cuserid)
    endif()

    if(HAVE_UALARM)
      list(APPEND UNISTD_FUNCTIONS ualarm)
    endif()

    more_events_add_interpose_test(
      unistd
      "<dice/events/unistd.h>"
      ${UNISTD_FUNCTIONS})
  endif()
endif()
