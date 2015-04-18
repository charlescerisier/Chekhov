// This file is a part of Chekhov, an actor/model concurrency framework for Rust.
//
// Chekhov is free software; you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License or (at your option) any later version.
//
// Chekhov is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Chekhov. If not see
// <https://www.gnu.org/licenses/>.

#![macro_use]

//! ```
//! #[macro_use]
//! extern crate chekhov;
//! 
//! use std::fmt::Display;
//! use std::io;
//!
//! use chekhov::*;
//!
//! fn print_prefixed<T: Display>(msg: T, prefix: &str) -> Result<(), ActorError> {
//!     println!("{}{}", prefix, msg);
//!     Ok(())
//! }
//!
//! fn read_input<A: Actor<String>>(next: &A) -> Result<(), ActorError> {
//!     let mut buffer = String::new();
//!     if io::stdin().read_line(&mut buffer).is_ok() {
//!         next.cue(buffer)
//!     } else { Err(ActorError::Internal("Could not read from stdin.".to_string())) }
//! }
//! 
//! fn main() {
//!     let printer = actor!(print_prefixed, ">>> ");
//!     actor_loop!(read_input, printer);
//! }
//! ```

mod macros;

pub mod actors;
pub use actors::*;

