//! # english-numbers
//!
//! A library for converting integers to their written-english formats.  
//! Supports both American "short" and European "long" number formats.

mod groups;
mod hundreds;
mod tens;
mod decimal;

mod words;
mod formatting;

#[cfg(test)]
mod test;

pub use formatting::Formatting;

/// Converts a number to it's written format, using "short" format.
///
/// Arguments:
/// * `val`: the `i64` to convert  
/// * `fmt`: the formatting options to use
pub fn convert_def(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false, Formatting::def())
        .build(Formatting::def())
}

/// Converts a number to it's written format, using "short" format with all formatting options enabled.
///
/// # Arguments:
/// * `val` - the `i64` to convert
pub fn convert_norm_dol(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false, Formatting::dollar())
        .build(Formatting::dollar())
}

/// Converts a number to it's written format, using "short" format with no formatting options enabled.
///
/// # Arguments:
/// * `val` - the `i64` to convert
pub fn convert_norm_cent(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false, Formatting::cents())
        .build(Formatting::cents())
}

/// Converts a number to it's written format, using "long" format.
///
/// # Arguments:
/// * `val` - the `i64` to convert  
/// * `fmt` - the formatting options to use
pub fn convert_norm_euro(val: i64) -> String
{
    groups::Groups::new(val)
        .build(false, Formatting::euro())
        .build(Formatting::euro())
}

pub fn convert_def_dec(val: i64) -> String
{

	groups::Groups::new(val)
		.build(false, Formatting::dec())
		.build(val, false, Formatting::dec())
}