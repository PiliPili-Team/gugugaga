mod register;
mod server;

use register::*;
use server::*;

// We have Register which handles token refresh and channel registration
// and Server which handles incoming HTTP requests
// These 2 components should running in 2 threads
pub struct Listener {
    register: Register,
    server: Server,
}

impl Listener {
    pub async fn new() -> Self {
        todo!()
    }

    pub async fn serve(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}
