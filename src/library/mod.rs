//! The standard library.
//!
//! Call [`new`] to obtain a [`Scope`] containing all standard library
//! definitions.

mod align;
mod document;
mod flow;
mod grid;
mod image;
mod link;
mod pad;
mod page;
mod par;
mod placed;
mod shape;
mod sized;
mod spacing;
mod stack;
mod text;
mod transform;
mod utility;

/// Helpful imports for creating library functionality.
mod prelude {
    pub use std::fmt::{self, Debug, Formatter};
    pub use std::rc::Rc;

    pub use typst_macros::properties;

    pub use crate::diag::{At, TypResult};
    pub use crate::eval::{
        Args, Construct, EvalContext, Node, Property, Set, Smart, Styles, Value,
    };
    pub use crate::frame::*;
    pub use crate::geom::*;
    pub use crate::layout::*;
    pub use crate::syntax::{Span, Spanned};
    pub use crate::util::{EcoString, OptionExt};
}

pub use self::image::*;
pub use align::*;
pub use document::*;
pub use flow::*;
pub use grid::*;
pub use link::*;
pub use pad::*;
pub use page::*;
pub use par::*;
pub use placed::*;
pub use shape::*;
pub use sized::*;
pub use spacing::*;
pub use stack::*;
pub use text::*;
pub use transform::*;
pub use utility::*;

use crate::eval::{Scope, Value};
use crate::geom::*;

/// Construct a scope containing all standard library definitions.
pub fn new() -> Scope {
    let mut std = Scope::new();

    // Classes.
    std.def_class::<PageNode>("page");
    std.def_class::<ParNode>("par");
    std.def_class::<TextNode>("text");

    // Text functions.
    std.def_func("strike", strike);
    std.def_func("underline", underline);
    std.def_func("overline", overline);
    std.def_func("link", link);

    // Layout functions.
    std.def_func("h", h);
    std.def_func("v", v);
    std.def_func("box", box_);
    std.def_func("block", block);
    std.def_func("pagebreak", pagebreak);
    std.def_func("parbreak", parbreak);
    std.def_func("linebreak", linebreak);
    std.def_func("stack", stack);
    std.def_func("grid", grid);
    std.def_func("pad", pad);
    std.def_func("align", align);
    std.def_func("place", place);
    std.def_func("move", move_);
    std.def_func("scale", scale);
    std.def_func("rotate", rotate);

    // Element functions.
    std.def_func("image", image);
    std.def_func("rect", rect);
    std.def_func("square", square);
    std.def_func("ellipse", ellipse);
    std.def_func("circle", circle);

    // Utility functions.
    std.def_func("assert", assert);
    std.def_func("type", type_);
    std.def_func("repr", repr);
    std.def_func("join", join);
    std.def_func("int", int);
    std.def_func("float", float);
    std.def_func("str", str);
    std.def_func("abs", abs);
    std.def_func("min", min);
    std.def_func("max", max);
    std.def_func("range", range);
    std.def_func("rgb", rgb);
    std.def_func("lower", lower);
    std.def_func("upper", upper);
    std.def_func("len", len);
    std.def_func("sorted", sorted);

    // Predefined colors.
    std.def_const("white", RgbaColor::WHITE);
    std.def_const("black", RgbaColor::BLACK);
    std.def_const("eastern", RgbaColor::new(0x23, 0x9D, 0xAD, 0xFF));
    std.def_const("conifer", RgbaColor::new(0x9f, 0xEB, 0x52, 0xFF));
    std.def_const("forest", RgbaColor::new(0x43, 0xA1, 0x27, 0xFF));

    // Other constants.
    std.def_const("ltr", Dir::LTR);
    std.def_const("rtl", Dir::RTL);
    std.def_const("ttb", Dir::TTB);
    std.def_const("btt", Dir::BTT);
    std.def_const("left", Align::Left);
    std.def_const("center", Align::Center);
    std.def_const("right", Align::Right);
    std.def_const("top", Align::Top);
    std.def_const("horizon", Align::Horizon);
    std.def_const("bottom", Align::Bottom);
    std.def_const("serif", FontFamily::Serif);
    std.def_const("sans-serif", FontFamily::SansSerif);
    std.def_const("monospace", FontFamily::Monospace);

    std
}

dynamic! {
    Dir: "direction",
}

castable! {
    Paint,
    Expected: "color",
    Value::Color(color) => Paint::Solid(color),
}
