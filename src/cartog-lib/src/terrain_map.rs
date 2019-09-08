use crate::terrain;
use crate::terrain_generator;

pub struct TerrainMap {
    terrain_items: Vec<terrain::Terrain>,
    rows: u32,
    cols: u32,
}

impl TerrainMap {
    pub fn new(rows: u32, cols: u32) -> Self {
        Self {
            terrain_items: terrain_generator::generate(rows as usize, cols as usize),
            rows: rows,
            cols: cols,
        }
    }

    pub fn get_rows(&self) -> u32 {
        self.rows
    }

    pub fn get_cols(&self) -> u32 {
        self.cols
    }

    pub fn get_terrain_at(&self, row: usize, col: usize) -> Option<&terrain::Terrain> {
        if row >= self.rows as usize || col >= self.cols as usize {
            return None;
        }

        let index = (row * self.cols as usize) + col;
        Some(&self.terrain_items[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_map() {
        let rows = 5;
        let cols = 10;
        let map = TerrainMap::new(rows, cols);
        assert_eq!(rows, map.get_rows());
        assert_eq!(cols, map.get_cols());
    }

    #[test]
    fn successfully_get_terrain_at() {
        let rows = 5;
        let cols = 10;
        let map = TerrainMap::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                assert_eq!(
                    true,
                    map.get_terrain_at(row as usize, col as usize).is_some()
                );
            }
        }
    }

    #[test]
    fn out_of_bounds_get_terrain_at() {
        let rows = 5;
        let cols = 10;
        let map = TerrainMap::new(rows, cols);

        assert_eq!(
            true,
            map.get_terrain_at((rows + 1) as usize, cols as usize)
                .is_none()
        );
        assert_eq!(
            true,
            map.get_terrain_at(rows as usize, (cols + 1) as usize)
                .is_none()
        );
        assert_eq!(
            true,
            map.get_terrain_at((rows + 1) as usize, (cols + 1) as usize)
                .is_none()
        );
    }
}