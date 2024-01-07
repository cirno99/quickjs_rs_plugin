use anyhow::Result;
use quickjs_wasm_rs::{JSContextRef, JSValue};
use std::io::Write;

/// set quickjs globals
pub fn set_quickjs_globals(context: &JSContextRef) -> anyhow::Result<()> {
    let console_object = context.object_value()?;

    let global = context.global_object()?;
    global.set_property("console", console_object)?;

    Ok(())
}
