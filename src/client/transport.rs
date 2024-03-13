// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use crate::error::Error;

/// An abstract communication used by the client to send requests
pub trait Transport {
    /// Sends a request to the PLC. Returns a reponse and an error, if there was any.
    fn send(&mut self, request: &[u8]) -> Result<Vec<u8>, Error>;
}
