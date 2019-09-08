
use crate::terrain;
use noise::*;

use rand::random;
use rand::Rng;

pub fn generate(rows: usize, cols: usize) -> Vec<terrain::Terrain> {
    let mut terrains = Vec::with_capacity(rows * cols);
    let elevation_generator = Perlin::new().set_seed(random());
    let moisture_generator = Perlin::new().set_seed(random());
    let mut rng = rand::thread_rng();
    let seed = rng.gen::<f64>();
    for row in 0..rows {
        for col in 0..cols {
            let nx = (row as f64 / rows as f64) - 0.5;
            let ny = (col as f64 / cols as f64) - 0.5;
            let elevation = elevation_generator.get([5.0 * nx, 5.0 * ny, seed]);
            let moisture = moisture_generator.get([nx, ny, seed]);

            terrains.push(terrain::Terrain::new(elevation, moisture));
        }
    }

    terrains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_terrain() {
        let terrains = generate(5, 10);
        assert_eq!(terrains.len(), 50);
    }
}