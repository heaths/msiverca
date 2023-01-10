// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#[cfg(not(target_os = "windows"))]
compile_error!("supported on windows only");

use msica::*;
use std::{fmt::Display, mem::size_of};

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

#[derive(Copy, Clone, Debug, Default)]
pub struct Version {
    major: u32,
    minor: u32,
    build: u32,
    product_type: ProductType,
}

impl Version {
    pub fn new(major: u32, minor: u32, build: u32, product_type: ProductType) -> Self {
        Version {
            major,
            minor,
            build,
            product_type,
        }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn build(&self) -> u32 {
        self.build
    }

    pub fn product_type(&self) -> ProductType {
        self.product_type
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

pub fn get_version() -> std::result::Result<Version, u32> {
    let mut info = OSVERSIONINFOEXW::default();
    unsafe {
        let err = RtlGetVersion(&mut info as *mut OSVERSIONINFOEXW);
        if err != 0 {
            return Err(err);
        }
    }

    Ok(Version {
        major: info.dwMajorVersion,
        minor: info.dwMinorVersion,
        build: info.dwBuildNumber,
        product_type: info.wProductType,
    })
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
    wProductType: ProductType,
    wReserved: u8,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum ProductType {
    #[default]
    Unknown,
    Workstation,
    DomainController,
    Server,
}

impl Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "(unknown)"),
            Self::Workstation => write!(f, "workstation"),
            Self::DomainController => write!(f, "domain controller"),
            Self::Server => write!(f, "server"),
        }
    }
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
            wProductType: ProductType::Unknown,
            wReserved: 0,
        }
    }
}

#[link(name = "ntdll")] // cspell:ignore ntdll
extern "C" {
    fn RtlGetVersion(version_information: *mut OSVERSIONINFOEXW) -> u32;
}
