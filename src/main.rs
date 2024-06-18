use anyhow::Result;

mod cli;
mod conversions;
mod splat_b;
mod splat_c;
mod splat_format;
mod support;

use cli::*;
use splat_b::*;
use splat_c::*;
use splat_format::*;
use support::*;

fn main() -> Result<()> {
    Cli::main()
}
