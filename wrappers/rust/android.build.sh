#!/usr/bin/env bash
export BLACK=`tput setaf 0`
export RED=`tput setaf 1`
export GREEN=`tput setaf 2`
export YELLOW=`tput setaf 3`
export BLUE=`tput setaf 4`
export MAGENTA=`tput setaf 5`
export CYAN=`tput setaf 6`
export WHITE=`tput setaf 7`

export BOLD=`tput bold`
export RESET=`tput sgr0`

set -e
set -o pipefail
WORKDIR=${PWD}
LIBINDYRS_WORKDIR=${WORKDIR}
export ANDROID_BUILD_FOLDER="/tmp/android_build"
DOWNLOAD_PREBUILTS="0"

while getopts ":d" opt; do
    case ${opt} in
        d) export DOWNLOAD_PREBUILTS="1";;
        \?);;
    esac
done
shift $((OPTIND -1))

TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "${RED}Missing TARGET_ARCH argument${RESET}"
    echo STDERR "${BLUE}e.g. x86 or arm${RESET}"
    exit 1
fi

#### SET UP ANDROID ENV
generate_arch_flags(){
    if [ -z $1 ]; then
        echo STDERR "${RED}Please provide the arch e.g arm,armv7, x86 or arm64${RESET}"
        exit 1
    fi
    export ABSOLUTE_ARCH=$1
    export TARGET_ARCH=$1
    if [ $1 == "arm" ]; then
        export TARGET_API="21"
        export TRIPLET="arm-linux-androideabi"
        export ANDROID_TRIPLET=${TRIPLET}
        export ABI="armeabi-v7a"
        export TOOLCHAIN_SYSROOT_LIB="lib"
    fi
    
    if [ $1 == "armv7" ]; then
        export TARGET_ARCH="arm"
        export TARGET_API="21"
        export TRIPLET="armv7-linux-androideabi"
        export ANDROID_TRIPLET="arm-linux-androideabi"
        export ABI="armeabi-v7a"
        export TOOLCHAIN_SYSROOT_LIB="lib"
    fi
    
    if [ $1 == "arm64" ]; then
        export TARGET_API="21"
        export TRIPLET="aarch64-linux-android"
        export ANDROID_TRIPLET=${TRIPLET}
        export ABI="arm64-v8a"
        export TOOLCHAIN_SYSROOT_LIB="lib"
    fi
    
    if [ $1 == "x86" ]; then
        export TARGET_API="21"
        export TRIPLET="i686-linux-android"
        export ANDROID_TRIPLET=${TRIPLET}
        export ABI="x86"
        export TOOLCHAIN_SYSROOT_LIB="lib"
    fi
    
    if [ $1 == "x86_64" ]; then
        export TARGET_API="21"
        export TRIPLET="x86_64-linux-android"
        export ANDROID_TRIPLET=${TRIPLET}
        export ABI="x86_64"
        export TOOLCHAIN_SYSROOT_LIB="lib64"
    fi
    
}

normalize_dir(){
    case "$1" in
        /*) echo "$1";;
        ~/*) echo "$1";;
        *) echo "$(pwd)/$1";;
    esac
}

setup_dependencies_env_vars(){
    export OPENSSL_DIR=${ANDROID_BUILD_FOLDER}/openssl_$1
    export SODIUM_DIR=${ANDROID_BUILD_FOLDER}/libsodium_$1
    export LIBZMQ_DIR=${ANDROID_BUILD_FOLDER}/libzmq_$1
}

setup_dependencies(){
    if [ "${DOWNLOAD_PREBUILTS}" == "1" ]; then
        setup_dependencies_env_vars ${ABSOLUTE_ARCH}
    else
        echo "${BLUE}Not downloading prebuilt dependencies. Dependencies locations have to be passed${RESET}"
        if [ -z "${OPENSSL_DIR}" ]; then
            
            OPENSSL_DIR=$(normalize_dir "openssl_${ABSOLUTE_ARCH}")
            if [ -d "${OPENSSL_DIR}" ]; then
                echo "${GREEN}Found ${OPENSSL_DIR}${RESET}"
                elif [ -z "$2" ]; then
                echo STDERR "${RED}Missing OPENSSL_DIR argument and environment variable${RESET}"
                echo STDERR "${BLUE}e.g. set OPENSSL_DIR=<path> for environment or openssl_${ABSOLUTE_ARCH}${RESET}"
                exit 1
            else
                OPENSSL_DIR=$2
            fi
        fi
        
        if [ -z "${SODIUM_DIR}" ]; then
            SODIUM_DIR=$(normalize_dir "libsodium_${ABSOLUTE_ARCH}")
            if [ -d "${SODIUM_DIR}" ] ; then
                echo "${GREEN}Found ${SODIUM_DIR}${RESET}"
                elif [ -z "$3" ]; then
                echo STDERR "${RED}Missing SODIUM_DIR argument and environment variable${RESET}"
                echo STDERR "${BLUE}e.g. set SODIUM_DIR=<path> for environment or libsodium_${ABSOLUTE_ARCH}${RESET}"
                exit 1
            else
                SODIUM_DIR=$3
            fi
        fi
        
        if [ -z "${LIBZMQ_DIR}" ] ; then
            LIBZMQ_DIR=$(normalize_dir  "libzmq_${ABSOLUTE_ARCH}")
            if [ -d "${LIBZMQ_DIR}" ] ; then
                echo "${GREEN}Found ${LIBZMQ_DIR}${RESET}"
                elif [ -z "$4" ] ; then
                echo STDERR "${RED}Missing LIBZMQ_DIR argument and environment variable${RESET}"
                echo STDERR "${BLUE}e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${ABSOLUTE_ARCH}${RESET}"
                exit 1
            else
                LIBZMQ_DIR=$4
            fi
        fi
    fi
}

set_env_vars(){
    export PKG_CONFIG_ALLOW_CROSS=1
    export CARGO_INCREMENTAL=1
    export RUST_LOG=indy=trace
    export RUST_TEST_THREADS=1
    export RUST_BACKTRACE=1
    export OPENSSL_DIR=${OPENSSL_DIR}
    export SODIUM_LIB_DIR=${SODIUM_DIR}/lib
    export SODIUM_INCLUDE_DIR=${SODIUM_DIR}/include
    export LIBZMQ_LIB_DIR=${LIBZMQ_DIR}/lib
    export LIBZMQ_INCLUDE_DIR=${LIBZMQ_DIR}/include
    export TOOLCHAIN_DIR=${TOOLCHAIN_PREFIX}/${TARGET_ARCH}
    export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
    export PKG_CONFIG_ALLOW_CROSS=1
    export CC=${TOOLCHAIN_DIR}/bin/${ANDROID_TRIPLET}-clang
    export AR=${TOOLCHAIN_DIR}/bin/${ANDROID_TRIPLET}-ar
    export CXX=${TOOLCHAIN_DIR}/bin/${ANDROID_TRIPLET}-clang++
    export CXXLD=${TOOLCHAIN_DIR}/bin/${ANDROID_TRIPLET}-ld
    export RANLIB=${TOOLCHAIN_DIR}/bin/${ANDROID_TRIPLET}-ranlib
    export TARGET=android
    export OPENSSL_STATIC=1
}

create_standalone_toolchain_and_rust_target(){
    #will only create toolchain if not already created
    python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py \
    --arch ${TARGET_ARCH} \
    --api ${TARGET_API} \
    --stl=libc++ \
    --force \
    --install-dir ${TOOLCHAIN_DIR}
    
    # add rust target
    rustup target add ${TRIPLET}
}

create_cargo_config(){
    mkdir -p ${LIBINDYRS_WORKDIR}/.cargo
cat << EOF > ${LIBINDYRS_WORKDIR}/.cargo/config
[target.${TRIPLET}]
ar = "$(realpath ${AR})"
linker = "$(realpath ${CC})"
EOF
}

build(){
    echo "**************************************************"
    echo "Building for architecture ${BOLD}${YELLOW}${ABSOLUTE_ARCH}${RESET}"
    echo "Toolchain path ${BOLD}${YELLOW}${TOOLCHAIN_DIR}${RESET}"
    echo "ZMQ path ${BOLD}${YELLOW}${LIBZMQ_DIR}${RESET}"
    echo "Sodium path ${BOLD}${YELLOW}${SODIUM_DIR}${RESET}"
    echo "Openssl path ${BOLD}${YELLOW}${OPENSSL_DIR}${RESET}"
    echo "Artifacts will be in ${YELLOW}${GREEN}${ANDROID_BUILD_FOLDER}/libindyrs_${ABSOLUTE_ARCH}${RESET}"
    echo "**************************************************"
    pushd ${WORKDIR}
    rm -rf target/${TRIPLET}
    cargo clean
    RUSTFLAGS="-L ${ANDROID_BUILD_FOLDER}/libindy_${ABSOLUTE_ARCH}/lib" \
    cargo build --release --target=${TRIPLET}
    
    popd
}

generate_arch_flags ${TARGET_ARCH}
setup_dependencies
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build
