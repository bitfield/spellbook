#![expect(unused_imports, reason = "example")]
// We start with crates from the “universal library”
use anyhow::Result;
use rand::prelude::*;

// Then imports “nearer to home”, from the stdlib
use std::collections::HashMap;
use std::process;

// Finally, the closest of all: items from this crate
use spellbook::example::Item;

fn main() {}
