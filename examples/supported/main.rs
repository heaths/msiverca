// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![windows_subsystem = "windows"]

use msiverca::{self, ProductType};
use std::{ffi::CString, process::ExitCode};
use windows::{
    core::PCSTR,
    s,
    Win32::UI::WindowsAndMessaging::{MessageBoxA, HWND_DESKTOP, MB_ICONERROR, MB_OK},
};

#[allow(clippy::collapsible_if)]
fn main() -> ExitCode {
    let cmd = clap::Command::new("supported")
        .arg(clap::arg!(-q - -quiet).action(clap::ArgAction::SetTrue));
    let m = cmd.get_matches();
    let quiet = m.get_flag("quiet");

    if let Ok(version) = msiverca::get_version() {
        if version.product_type() == ProductType::Workstation {
            // Client
            if version.major() < 11 {
                return error(quiet, "Supported on Windows 10 and newer");
            }
        } else if version.product_type() == ProductType::DomainController
            || version.product_type() == ProductType::Server
        {
            // Server
            if version.major() < 6 || (version.major() == 6 && version.minor() < 3) {
                return error(quiet, "Supported on Windows Server 2012 and newer");
            }
        }
    }

    ExitCode::SUCCESS
}

fn error(quiet: bool, message: &str) -> ExitCode {
    if !quiet {
        unsafe {
            // cspell:disable
            let message = CString::new(message).unwrap();
            MessageBoxA(
                HWND_DESKTOP,
                PCSTR::from_raw(message.as_ptr() as *const u8),
                s!("Example"),
                MB_ICONERROR | MB_OK,
            );
            // cspell:enable
        }
    }

    ExitCode::FAILURE
}
