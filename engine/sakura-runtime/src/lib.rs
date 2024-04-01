//! # sakura-runtime
//!
//! A runtime for managing the Lua scripting environment.

use std::error::Error;

use mlua::{prelude::*, Lua, LuaOptions, StdLib};

/// A runtime for managing the Lua scripting environment.
pub struct Runtime {
    lua: Lua,
}

impl Runtime {
    /// Initialise the runtime.
    pub fn init(self) {}
}

/// A builder for creating a [Runtime] instance.
pub struct RuntimeBuilder {
    lua: Lua,
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self {
            lua: Lua::new_with(
                StdLib::MATH | StdLib::STRING | StdLib::TABLE,
                LuaOptions::new().thread_pool_size(4),
            )
            .expect("invalid default VM options"),
        }
    }
}

impl RuntimeBuilder {
    /// Add a module to the runtime.
    pub fn with_rust_module<M: FnOnce(&Lua) -> LuaResult<LuaTable>>(
        self,
        name: &str,
        module: M,
    ) -> Self {
        self.lua
            .globals()
            .set(name, module(&self.lua).expect("failed to load module"))
            .unwrap();
        self
    }

    /// Initialize the runtime.
    pub fn build(self) -> Result<Runtime, Box<dyn Error>> {
        Ok(Runtime { lua: self.lua })
    }
}

impl Runtime {
    /// Process events.
    pub fn process_events(&self) {
        self.lua
            .load("print('Hello, world!')")
            .exec()
            .expect("failed to execute script");
    }
}
