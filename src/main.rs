use strum::IntoEnumIterator;

use crate::interaction::{input_with_range, select_from_enum, string_input};
use crate::models::{AutoBoot, ConfigGenerator, VideoMode};

mod interaction;
mod models;

fn main() -> eyre::Result<()> {
    let video_mode_selection = select_from_enum::<VideoMode>("Select a video mode")?;
    let selected_video_mode = VideoMode::iter().nth(video_mode_selection).unwrap();

    let auto_boot_selection = select_from_enum::<AutoBoot>("Select an autoboot option")?;
    let selected_auto_boot = AutoBoot::iter().nth(auto_boot_selection).unwrap();

    let auto_boot_delay = if selected_auto_boot != AutoBoot::NONE {
        Some(input_with_range("Enter an autoboot delay", 5, 0..=10)?)
    } else {
        None
    };

    let sd_card_path = string_input(
        "Enter the path of your SD card (leave empty for current directory)",
        String::from(""),
    )?;

    let config_generator = ConfigGenerator::new(
        selected_video_mode,
        selected_auto_boot,
        auto_boot_delay,
        sd_card_path,
    );
    let conf = config_generator.generate_ini();
    config_generator.write_to_file(conf)?;

    Ok(())
}
