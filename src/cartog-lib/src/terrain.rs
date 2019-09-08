use crate::terrain_type;

const WATER_ELEVATION: f64 = -0.4;
const BEACH_ELEVATION: f64 = 0.0;
const HIGH_ELEVATION: f64 = 0.8;
const LOW_ELEVATION: f64 = 0.3;

pub struct Terrain {
    elevation: f64,
    terrain_type: terrain_type::TerrainType,
}

impl Terrain {

    pub fn new(elevation: f64, moisture: f64) -> Self {
        if elevation <= WATER_ELEVATION {
            return Self::with_type(elevation, terrain_type::TerrainType::Water);
        }

        if elevation <= BEACH_ELEVATION {
            return Self::with_type(elevation, terrain_type::TerrainType::Beach);
        }

        let high_terrain = Self::high_elevation_terrain(elevation, moisture);
        if let Some(land) = high_terrain {
            return land;
        }

        let mid_terrain = Self::mid_elevation_terrain(elevation, moisture);
        if let Some(land) = mid_terrain {
            return land;
        }

        let low_terrain = Self::low_elevation_terrain(elevation, moisture);
        if let Some(land) = low_terrain {
            return land;
        }

        if moisture < 0.16 {
            return Self::with_type(elevation, terrain_type::TerrainType::SubTropicalDesert);
        }

        if moisture < 0.33 {
            return Self::with_type(elevation, terrain_type::TerrainType::Grass);
        }

        if moisture < 0.66 {
            return Self::with_type(elevation, terrain_type::TerrainType::TropicalForest);
        }

        Self::with_type(elevation, terrain_type::TerrainType::TropicalRainForest)

    }

    pub fn with_type(elevation: f64, terrain_type: terrain_type::TerrainType) -> Self {
        Self {
            elevation: elevation,
            terrain_type: terrain_type,
        }
    }

    pub fn with_polar(elevation: f64, moisture: f64, is_polar: bool) -> Self {
        if elevation < WATER_ELEVATION {
            return Self::with_type(elevation, terrain_type::TerrainType::Water);
        }

        if is_polar {
            return Self::with_type(elevation, terrain_type::TerrainType::Snow);
        }

        Self::new(elevation, moisture)
    }

    pub fn get_elevation(&self) -> f64 {
        self.elevation
    }

    pub fn get_type(&self) -> &terrain_type::TerrainType {
        &self.terrain_type
    }

    fn high_elevation_terrain(elevation: f64, moisture: f64) -> Option<Self> {
        if elevation < HIGH_ELEVATION {
            return None;
        }

        if moisture < -0.8 {
            return Some(Self::with_type(
                elevation,
                terrain_type::TerrainType::Volcanic,
            ));
        }

        if moisture < -0.6 {
            return Some(Self::with_type(elevation, terrain_type::TerrainType::Bare));
        }

        if moisture < 0.0 {
            return Some(Self::with_type(
                elevation,
                terrain_type::TerrainType::Tundra,
            ));
        }

        return Some(Self::with_type(elevation, terrain_type::TerrainType::Snow));
    }

    fn mid_elevation_terrain(elevation: f64, moisture: f64) -> Option<Self> {
        if elevation <= LOW_ELEVATION || elevation >= HIGH_ELEVATION {
            return None;
        }

        if moisture < -0.33 {
            return Some(Self::with_type(
                elevation,
                terrain_type::TerrainType::Desert,
            ));
        }

        if moisture < 0.22 {
            return Some(Self::with_type(elevation, terrain_type::TerrainType::Grass));
        }

        return Some(Self::with_type(elevation, terrain_type::TerrainType::Taiga));
    }

    fn low_elevation_terrain(elevation: f64, moisture: f64) -> Option<Self> {
        if elevation > LOW_ELEVATION {
            return None;
        }

        if moisture < -0.68 {
            return Some(Self::with_type(
                elevation,
                terrain_type::TerrainType::Desert,
            ));
        }

        if moisture < 0.0 {
            return Some(Self::with_type(elevation, terrain_type::TerrainType::Grass));
        }

        if moisture < 0.66 {
            return Some(Self::with_type(
                elevation,
                terrain_type::TerrainType::Forest,
            ));
        }

        return Some(Self::with_type(
            elevation,
            terrain_type::TerrainType::RainForest,
        ));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn water_level_test() {
        let water = Terrain::new(WATER_ELEVATION, 0.0);
        assert_eq!(water.get_type(), &terrain_type::TerrainType::Water);

        let not_water = Terrain::new(WATER_ELEVATION + 0.1, 0.0);
        assert_ne!(not_water.get_type(), &terrain_type::TerrainType::Water);
    }

    #[test]
    fn beach_level_test() {
        let beach = Terrain::new(BEACH_ELEVATION, 0.0);
        assert_eq!(beach.get_type(), &terrain_type::TerrainType::Beach);

        let not_beach = Terrain::new(BEACH_ELEVATION + 0.1, 0.0);
        assert_ne!(not_beach.get_type(), &terrain_type::TerrainType::Beach);
    }

    #[test]
    fn high_elevation_terrain_test() {
        let volcanic = Terrain::new(HIGH_ELEVATION, -0.81);
        assert_eq!(volcanic.get_type(), &terrain_type::TerrainType::Volcanic);
        let bare = Terrain::new(HIGH_ELEVATION, -0.61);
        assert_eq!(bare.get_type(), &terrain_type::TerrainType::Bare);
        let tundra = Terrain::new(HIGH_ELEVATION, -0.1);
        assert_eq!(tundra.get_type(), &terrain_type::TerrainType::Tundra);
        let snow = Terrain::new(HIGH_ELEVATION, 0.1);
        assert_eq!(snow.get_type(), &terrain_type::TerrainType::Snow);
    }

    #[test]
    fn mid_elevation_terrain_test() {
        let desert = Terrain::new(HIGH_ELEVATION - 0.1, -0.34);
        assert_eq!(desert.get_type(), &terrain_type::TerrainType::Desert);
        let grass = Terrain::new(HIGH_ELEVATION - 0.1, 0.2);
        assert_eq!(grass.get_type(), &terrain_type::TerrainType::Grass);
        let taiga = Terrain::new(HIGH_ELEVATION - 0.1, 1.0);
        assert_eq!(taiga.get_type(), &terrain_type::TerrainType::Taiga);
    }

    #[test]
    fn low_elevation_terrain_test() {
        let desert = Terrain::new(LOW_ELEVATION - 0.1, -0.81);
        assert_eq!(desert.get_type(), &terrain_type::TerrainType::Desert);
        let grass = Terrain::new(LOW_ELEVATION - 0.1, -0.2);
        assert_eq!(grass.get_type(), &terrain_type::TerrainType::Grass);
        let forest = Terrain::new(LOW_ELEVATION - 0.1, 0.65);
        assert_eq!(forest.get_type(), &terrain_type::TerrainType::Forest);
        let rain_forest = Terrain::new(LOW_ELEVATION - 0.1, 1.0);
        assert_eq!(
            rain_forest.get_type(),
            &terrain_type::TerrainType::RainForest
        );
    }
}