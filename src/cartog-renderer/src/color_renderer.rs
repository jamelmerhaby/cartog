extern crate cartog_lib;
extern crate image;

use cartog_lib::terrain;
use cartog_lib::terrain_map;
use cartog_lib::terrain_type;

pub fn render(map: terrain_map::TerrainMap, output_path: &str) {
    let mut image = image::RgbImage::new(map.get_cols(), map.get_rows());
    for row in 0..map.get_rows() {
        for col in 0..map.get_cols() {
            let curr_terrain = map.get_terrain_at(row as usize, col as usize);
            if let Some(curr_terrain) = curr_terrain {
                image.put_pixel(col as u32, row as u32, terrain_to_color(curr_terrain));
            } else {
                panic!("Could not get terrain for row: {}, column: {}!", row, col);
            }
        }
    }

    let file_name = format!("{}.png", output_path);
    image.save(&std::path::Path::new(&file_name)).unwrap();
}

fn terrain_to_color(terrain: &terrain::Terrain) -> image::Rgb<u8> {
    return match terrain.get_type() {
        terrain_type::TerrainType::Bare => image::Rgb([209, 205, 192]),
        terrain_type::TerrainType::Beach => image::Rgb([227, 212, 170]),
        terrain_type::TerrainType::Desert => image::Rgb([207, 170, 110]),
        terrain_type::TerrainType::Forest => image::Rgb([100, 140, 80]),
        terrain_type::TerrainType::Grass => image::Rgb([140, 195, 120]),
        terrain_type::TerrainType::Mountain => image::Rgb([103, 112, 117]),
        terrain_type::TerrainType::RainForest => image::Rgb([60, 166, 80]),
        terrain_type::TerrainType::Snow => image::Rgb([230, 230, 230]),
        terrain_type::TerrainType::SubTropicalDesert => image::Rgb([180, 144, 116]),
        terrain_type::TerrainType::Swamp => image::Rgb([160, 190, 115]),
        terrain_type::TerrainType::Taiga => image::Rgb([160, 200, 100]),
        terrain_type::TerrainType::TropicalForest => image::Rgb([120, 180, 90]),
        terrain_type::TerrainType::TropicalRainForest => image::Rgb([120, 180, 90]),
        terrain_type::TerrainType::Tundra => image::Rgb([200, 215, 215]),
        terrain_type::TerrainType::Volcanic => image::Rgb([68, 75, 75]),
        terrain_type::TerrainType::Water => image::Rgb([15, 65, 100]),
    };
}