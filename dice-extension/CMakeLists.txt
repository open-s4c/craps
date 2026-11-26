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
