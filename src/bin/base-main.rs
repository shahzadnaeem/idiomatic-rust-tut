#![allow(unused)] // During initial development only

use std::env::{args, current_dir};
use std::fs::read_dir;
use std::path;

use idioms::base::prelude::*;
use idioms::base::utils::pop::pop;

fn main() -> Result<()> {
    println!("Hello from base-main");

    println!("\nDirectory entries for {}", (current_dir()?).display());

    for it in read_dir("./")?.filter_map(|e| e.ok()) {
        let it: String = NewT(&it).try_into()?;

        println!("  {it}");
    }

    println!("\npop(123) = {}", pop(123)?);

    if args().len() > 1 {
        for arg in args().skip(1) {
            println!("Parsing {}", arg);

            let num = arg.parse::<i32>()?;

            println!("pop({}) = {}", num, pop(num)?);
        }
    }

    Ok(())
}
