use polars_arrow::compute::tile;

use crate::datatypes::PolarsNumericType;
use crate::prelude::ChunkedArray;

impl<T: PolarsNumericType> ChunkedArray<T> {
    pub fn tile(&self, n: usize) -> Self {
        let ca = self.rechunk();
        let arr = ca.downcast_iter().next().unwrap();
        let arr = tile::tile_primitive(arr, n);
        ChunkedArray::from_chunk_iter(self.name(), [arr])
    }
}
