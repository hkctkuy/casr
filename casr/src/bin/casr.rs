use casr::casr;

use anyhow::Result;

use std::env;

fn main() -> Result<()> {
    casr::casr(&env::args().collect::<Vec<String>>(), None)
}
