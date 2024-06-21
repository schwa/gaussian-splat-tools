use anyhow::Result;

mod cli;
mod guess_format;
mod splat_a;
mod splat_b;
mod splat_c;
mod splat_format;
mod support;
mod uber_splat;
mod actions;

use cli::*;
use guess_format::*;
use splat_a::*;
use splat_b::*;
use splat_c::*;
use splat_format::*;
use support::*;
use uber_splat::*;
use actions::*;

fn main() -> Result<()> {
    Cli::main()
}
