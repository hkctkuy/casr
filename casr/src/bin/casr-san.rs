use ::casr::{casr, mode::Mode};

use anyhow::Result;

fn main() -> Result<()> {
    casr::stub(Mode::San)
}
