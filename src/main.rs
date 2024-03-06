use strum::IntoEnumIterator;

use crate::interaction::{input_with_range, select_from_enum, string_input};
use crate::models::{AutoBoot, ConfigGenerator, VideoMode};

mod interaction;
mod models;

fn main() {
    let video_mode_selection = select_from_enum::<VideoMode>("Please choose a video mode")?;
    let selected_video_mode = VideoMode::iter().nth(video_mode_selection).unwrap();

    let auto_boot_selection = select_from_enum::<AutoBoot>("Please choose an autoboot option")?;
    let selected_auto_boot = AutoBoot::iter().nth(auto_boot_selection).unwrap();

    let auto_boot_delay = if selected_auto_boot != AutoBoot::NONE {
        match input_with_range("Please enter an autoboot delay", 5, 0..=10) {
            Ok(input) => Some(input),
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        }
    } else {
        None
    };

    let sd_card_path = match string_input(
        "Please enter the path of your SD card (leave empty for current directory)",
        String::from(""),
    ) {
        Ok(path) => path,
        Err(e) => {
            log::error!("Error getting SD card path: {}", e);
            return;
        }
    };

    let config_generator = ConfigGenerator::new(
        selected_video_mode,
        selected_auto_boot,
        auto_boot_delay,
        sd_card_path,
    );
    let conf = config_generator.generate_ini();
    if let Err(e) = config_generator.write_to_file(conf) {
        log::error!("{}", e);
        return;
    };
}
