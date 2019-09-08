extern crate clap;

extern crate cartog_lib;
extern crate cartog_renderer;

use std::time::{SystemTime, UNIX_EPOCH};

use cartog_lib::terrain_map;
use clap::{App, Arg};
fn main() {
    let arguments = App::new("Cartog")
        .version("0.1.0")
        .author("Jamel El-Merhaby")
        .about("Procedurally generates maps using Perlin noise.")
        .arg(Arg::with_name("file")
            .short("o")
            .long("output_file_path")
            .takes_value(true)
            .help("Output file name and path for generated map. The file will be saved with '.png' extension."))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true)
            .help("Positive number representing the desired width in pixels for the generated map image."))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .takes_value(true)
            .help("Positive number representing the desired height in pixels for the generated map image."))
        .get_matches();

    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let default_output_path = format!("map_{:?}", time);
    let output_file_path = arguments.value_of("file").unwrap_or(&default_output_path);
    let width = get_number_from_str(get_arg_value(arguments.value_of("width")));
    let height = get_number_from_str(get_arg_value(arguments.value_of("height")));

    let map = terrain_map::TerrainMap::new(height, width);
    cartog_renderer::color_renderer::render(map, output_file_path);
}

fn get_arg_value(parsed_arg: Option<&str>) -> &str {
    return match parsed_arg {
        Some(value) => value,
        _ => panic!("Could not parse a value from the command line."),
    };
}

fn get_number_from_str(str_value: &str) -> u32 {
    let num_value = str_value.parse::<u32>();
    return match num_value {
        Ok(num) => num,
        _ => panic!("Could not parse a value from string: {}.", str_value),
    };
}