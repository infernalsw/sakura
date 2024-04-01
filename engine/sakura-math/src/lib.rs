//! # sakura-math
//!
//! Lua bindings for the [glam] math library.

use mlua::prelude::*;

mod matrix;
mod vector;

/// Create and export the math module.
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    // declare submodules
    exports.set("vector", vector::module(lua)?)?;
    exports.set("matrix", matrix::module(lua)?)?;

    Ok(exports)
}
