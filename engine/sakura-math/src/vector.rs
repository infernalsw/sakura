use mlua::prelude::*;

/// The vector submodule.
pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("vec2", lua.create_proxy::<LuaVec2>()?)?;
    Ok(exports)
}

struct LuaVec2 {
    inner: glam::Vec2,
}

impl LuaUserData for LuaVec2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.inner.x));
        fields.add_field_method_get("y", |_, this| Ok(this.inner.y));
        fields.add_field_method_set("x", |_, this, value: f32| {
            this.inner.x = value;
            Ok(())
        });
        fields.add_field_method_set("y", |_, this, value: f32| {
            this.inner.y = value;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("add", |_, this, other: LuaUserDataRef<LuaVec2>| {
            Ok(LuaVec2 {
                inner: this.inner + other.inner,
            })
        });
        methods.add_method("sub", |_, this, other: LuaUserDataRef<LuaVec2>| {
            Ok(LuaVec2 {
                inner: this.inner - other.inner,
            })
        });
        methods.add_method("mul", |_, this, other: LuaUserDataRef<LuaVec2>| {
            Ok(LuaVec2 {
                inner: this.inner * other.inner,
            })
        });
        methods.add_method("div", |_, this, other: LuaUserDataRef<LuaVec2>| {
            Ok(LuaVec2 {
                inner: this.inner / other.inner,
            })
        });
    }
}
