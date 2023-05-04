mod core;
mod fonts;
mod ux;

use anyhow::Result;

fn main() -> Result<()> {
    ux::init()?;

    Ok(())
}
