cmake_minimum_required(VERSION 3.12)
project(nts1mkii C)

set(CMAKE_C_COMPILER ${CMAKE_CURRENT_SOURCE_DIR}/toolchain/gcc-arm-none-eabi/bin/arm-none-eabi-gcc)

set(SOURCES
    components/logue-sdk/platform/nts-1_mkii/common/_unit_base.c
)

include_directories(
    components/logue-sdk/platform/nts-1_mkii/common
    components/logue-sdk/platform/ext/CMSIS/CMSIS/Include
)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} \
    -mcpu=cortex-m7 \
    -DSTM32H725xE -DCORTEX_USE_FPU=TRUE -DARM_MATH_CM7 -D__FPU_PRESENT \
    -fPIC -std=c11 -fno-exceptions \
    -W -Wall -Wextra \
    -mfloat-abi=hard -mfpu=fpv4-sp-d16 -fsingle-precision-constant -fcheck-new \
    -g -Os -mlittle-endian \
    -mthumb -mno-thumb-interwork -DTHUMB_NO_INTERWORKING -DTHUMB_PRESENT"
)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ../dist)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ../dist)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ../dist)

add_library(${PROJECT_NAME} ${SOURCES})
