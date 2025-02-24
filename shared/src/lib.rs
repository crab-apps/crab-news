pub mod app;
pub use app::*;
pub use crux_core::bridge::{Bridge, Request};
pub use crux_core::Core;
pub use crux_http as http;
use lazy_static::lazy_static;

uniffi::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Bridge<CrabNews> = Bridge::new(Core::new());
}
pub fn process_event(data: &[u8]) -> Vec<u8> {
    match CORE.process_event(data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

pub fn handle_response(id: u32, data: &[u8]) -> Vec<u8> {
    match CORE.handle_response(id, data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

pub fn view() -> Vec<u8> {
    match CORE.view() {
        Ok(view) => view,
        Err(e) => panic!("{e}"),
    }
}
