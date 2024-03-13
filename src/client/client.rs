// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

/// Client connection type
#[derive(Debug, Copy, Clone)]
pub enum Connection {
    /// Connect to the PLC
    PLC = 1,
    /// Connect to the HMI panel
    HMI = 2,
    /// Basic connection for generic data transfer
    BASIC = 3,
}

#[derive(Debug, Clone)]
pub struct Client<T: Transport> {
    transport: T,
}

impl<T: Transport> Client<T> {
    pub fn new(mut transport: T) -> Result<Client<T>, Error> {
        Ok(Client { transport })
    }
}
