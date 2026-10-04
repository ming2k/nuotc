//! Differential rendering engine, escape sequence emitters, and terminal profiles.

pub mod diff;
pub mod driver;
pub mod profile;
pub mod record;

pub use diff::{Draw, DrawCmd, diff};
pub use driver::{
    Ansi16Driver, DirectColorDriver, EscapeEmitter, Indexed256Driver, MonochromeDriver,
    TerminalDriver,
};
pub use profile::{
    CharsetStandard, ColorModel, ColorStandard, ElevationArchetype, SpatialCost, TerminalProfile,
    quantize_to_ansi16, quantize_to_indexed256,
};
pub use record::{Canvas, DisplayList, DrawOp, Rasterizer, RenderNode};
