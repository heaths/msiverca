// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() {
    if let Ok(version) = msiverca::get_version() {
        println!("{}", version);
    }
}
