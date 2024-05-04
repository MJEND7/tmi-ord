use super::*;

pub struct Rtx(pub redb::ReadTransaction);

impl Rtx {
  pub fn block_height(&self) -> Result<Option<Height>> {
    Ok(
      self
        .0
        .open_table(HEIGHT_TO_BLOCK_HEADER)?
        .range(0..)?
        .next_back()
        .transpose()?
        .map(|(height, _header)| Height(height.value())),
    )
  }

  pub fn block_count(&self) -> Result<u32> {
    Ok(
      self
        .0
        .open_table(HEIGHT_TO_BLOCK_HEADER)?
        .range(0..)?
        .next_back()
        .transpose()?
        .map(|(height, _header)| height.value() + 1)
        .unwrap_or(0),
    )
  }

  pub fn block_hash(&self, height: Option<u32>) -> Result<Option<BlockHash>> {
    let height_to_block_header = self.0.open_table(HEIGHT_TO_BLOCK_HEADER)?;

    Ok(
      match height {
        Some(height) => height_to_block_header.get(height)?,
        None => height_to_block_header
          .range(0..)?
          .next_back()
          .transpose()?
          .map(|(_height, header)| header),
      }
      .map(|header| Header::load(*header.value()).block_hash()),
    )
  }
}
