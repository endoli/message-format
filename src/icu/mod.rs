// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! ICU Message Format Support
//!
//! This module provides support for [ICU-style message formatting].
//! The ICU Message Format is widely supported in many languages and
//! environments.
//!
//! This library aims to support all of the ICU Message Format with the
//! exception of the deprecated `ChoiceFormat`.
//!
//! The important functionality provided here is the [`icu::parse`]
//! function which generates [`Message`] from a string.
//!
//! [`icu::parse`]: fn.parse.html
//! [`Message`]: ../struct.Message.html
//! [ICU-style message formatting]: http://userguide.icu-project.org/formatparse/messages

pub mod ast;
mod parse;

pub use self::parse::parse;
