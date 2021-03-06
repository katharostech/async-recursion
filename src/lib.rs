//! <h5>Procedural macro for recursive async functions</h5>
//!
//! Consider the following recursive implementation of the fibonacci numbers:
//!
//! ```compile_fail
//! async fn fib(n : u32) -> u64 {
//!    match n {
//!        0     => panic!("zero is not a valid argument to fib()!"),
//!        1 | 2 => 1,
//!        3     => 2,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! The compiler helpfully tells us that:
//!
//! ```console
//! error[E0733]: recursion in an `async fn` requires boxing
//! --> src/main.rs:1:26
//! |
//! 1 | async fn fib(n : u32) -> u64 {
//! |                          ^^^ recursive `async fn`
//! |
//! = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
//! ```
//!
//! This crate provides an attribute macro to automatically convert an async function
//! to one returning a boxed Future.
//!
//! # Example
//!
//! ```
//! use async_recursion::async_recursion;
//!
//! #[async_recursion]
//! async fn fib(n : u32) -> u64 {
//!    match n {
//!        0     => panic!("zero is not a valid argument to fib()!"),
//!        1 | 2 => 1,
//!        3     => 2,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! # Limitations
//! Currently the macro doesn't consider lifetimes at all; this is something I plan to work
//! on in the future.
//!
//! ### License
//! Licensed under either of
//! * Apache License, Version 2.0
//!   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.

extern crate proc_macro;

mod expand;
mod parse;

use crate::expand::expand;
use crate::parse::AsyncItem;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_recursion(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as AsyncItem);

    expand(&mut item);

    TokenStream::from(quote!(#item))
}
