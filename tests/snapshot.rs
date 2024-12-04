use std::error::Error;

use snapshot::make_snapshot;

#[test]
fn try_make_snapshot() -> Result<(), Box<dyn Error>> {
  make_snapshot("http://localhost:3000", true)?;
  Ok(())
}