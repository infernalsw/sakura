use std::error::Error;

use mlua::Lua;

/// A runtime for managing the Lua scripting environment.
pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    /// Initialise the runtime.
    pub fn init(self) {}
}

/// A builder for creating a [Runtime] instance.
#[derive(Default)]
pub struct RuntimeBuilder {
    lua: Lua,
}

impl RuntimeBuilder {
    /// Initialize the runtime.
    pub fn build(self) -> Result<Runtime, Box<dyn Error>> {
        Ok(Runtime { lua: self.lua })
    }
}
