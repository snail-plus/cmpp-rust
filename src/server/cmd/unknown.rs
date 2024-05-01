

#[derive(Debug, Clone)]
pub struct Unknown {
    command_id: u32,
}

impl Unknown {
    /// Create a new `Unknown` command which responds to unknown commands
    /// issued by clients
    pub(crate) fn new(command_id: u32) -> Unknown {
        Unknown {
            command_id,
        }
    }
}