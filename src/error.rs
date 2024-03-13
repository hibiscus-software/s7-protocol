// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Connection(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::Connection(string) => write!(f, "[ERROR]: Connection Error: {}", string),
        }
    }
}
