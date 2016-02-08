//! # The LLaMaPUn library in Rust
//! This is an attempt to reimplement the LLaMaPUn library in Rust.
//! The original library can be found at https://github.com/KWARC/LLaMaPUn

#![feature(slice_patterns)]
#![feature(type_ascription)]
#![warn(missing_docs)]

extern crate libxml;
extern crate libc;
extern crate regex;
extern crate unidecode;
extern crate gnuplot;
extern crate rustmorpha;
extern crate walkdir;
extern crate senna;

#[macro_use] pub mod util;
pub mod dnm;
pub mod data;
pub mod stopwords;
pub mod tokenizer;
pub mod ngrams;
pub mod util;
