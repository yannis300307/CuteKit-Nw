#[cfg(feature = "upsilon")]
#[unsafe(no_mangle)]
pub extern "C" fn extapp_main() { crate::main(); }
