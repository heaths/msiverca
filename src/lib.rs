// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#[cfg(not(target_os = "windows"))]
compile_error!("supported on windows only");

use msica::*;
use std::mem::size_of;

const ERROR_SUCCCESS: u32 = 0;
const ERROR_INSTALL_FAILURE: u32 = 1603;

#[allow(unused_must_use)]
#[no_mangle]
pub extern "C" fn SetVersionInfo(session: Session) -> u32 {
    let mut info = OSVERSIONINFOEXW::default();
    unsafe {
        if RtlGetVersion(&mut info as *mut OSVERSIONINFOEXW) != 0 {
            return ERROR_INSTALL_FAILURE;
        }
    }

    session.set_property(
        "VER_WINDOWS_MAJOR",
        Some(format!("{}", info.dwMajorVersion).as_str()),
    );
    session.set_property(
        "VER_WINDOWS_MINOR",
        Some(format!("{}", info.dwMinorVersion).as_str()),
    );
    session.set_property(
        "VER_WINDOWS_BUILD",
        Some(format!("{}", info.dwBuildNumber).as_str()),
    );

    ERROR_SUCCCESS
}

pub fn get_version() -> std::result::Result<String, u32> {
    let mut info = OSVERSIONINFOEXW::default();
    unsafe {
        let err = RtlGetVersion(&mut info as *mut OSVERSIONINFOEXW);
        if err != 0 {
            return Err(err);
        }
    }

    Ok(format!(
        "{}.{}.{}",
        info.dwMajorVersion, info.dwMinorVersion, info.dwBuildNumber
    ))
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
struct OSVERSIONINFOEXW {
    dwOSVersionInfoSize: u32,
    dwMajorVersion: u32,
    dwMinorVersion: u32,
    dwBuildNumber: u32,
    dwPlatformId: u32,
    szCSDVersion: [u16; 128],
    wServicePackMajor: u16,
    wServicePackMinor: u16,
    wSuiteMask: u16,
    wProductType: u8,
    wReserved: u8,
}

impl Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        OSVERSIONINFOEXW {
            dwOSVersionInfoSize: size_of::<OSVERSIONINFOEXW>() as u32,
            dwMajorVersion: 0,
            dwMinorVersion: 0,
            dwBuildNumber: 0,
            dwPlatformId: 0,
            szCSDVersion: [0; 128],
            wServicePackMajor: 0,
            wServicePackMinor: 0,
            wSuiteMask: 0,
            wProductType: 0,
            wReserved: 0,
        }
    }
}

#[link(name = "ntdll")] // cspell:ignore ntdll
extern "C" {
    fn RtlGetVersion(version_information: *mut OSVERSIONINFOEXW) -> u32;
}
