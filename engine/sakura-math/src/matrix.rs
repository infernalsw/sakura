use mlua::prelude::*;

/// The matrix submodule.
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    Ok(exports)
}
