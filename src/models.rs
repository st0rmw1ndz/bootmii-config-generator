#![allow(clippy::upper_case_acronyms)]

use std::env;
use std::path::PathBuf;

use ini::Ini;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, Display, EnumIter, EnumString)]
pub enum VideoMode {
    NTSC,
    PAL50,
    PAL60,
    PROGRESSIVE,
}

#[derive(Debug, PartialEq, Display, EnumIter, EnumString)]
pub enum AutoBoot {
    SYSMENU,
    HBC,
    NONE,
}

pub struct ConfigGenerator {
    video_mode: VideoMode,
    auto_boot: AutoBoot,
    auto_boot_delay: Option<u8>,
    sd_card_path: String,
}

impl ConfigGenerator {
    pub fn new(
        video_mode: VideoMode,
        auto_boot: AutoBoot,
        auto_boot_delay: Option<u8>,
        sd_card_path: String,
    ) -> Self {
        Self {
            video_mode,
            auto_boot,
            auto_boot_delay,
            sd_card_path,
        }
    }

    pub fn generate_ini(&self) -> Ini {
        let mut conf = Ini::new();
        conf.with_general_section()
            .set("VIDEOMODE", &self.video_mode.to_string());
        if self.auto_boot != AutoBoot::NONE {
            conf.with_general_section()
                .set("AUTOBOOT", &self.auto_boot.to_string());
            if let Some(auto_boot_delay) = self.auto_boot_delay {
                conf.with_general_section()
                    .set("BOOTDELAY", format!("{:?}", auto_boot_delay));
            }
        }
        conf
    }

    pub fn write_to_file(&self, conf: Ini) -> eyre::Result<()> {
        let mut file_path = if !self.sd_card_path.is_empty() {
            let mut path = PathBuf::from(&self.sd_card_path);
            path.push("bootmii");
            if !path.exists() {
                return Err(eyre::Report::msg(
                    "The directory 'bootmii' does not exist at the provided SD card path.",
                ));
            }
            path
        } else {
            env::current_dir()?
        };
        file_path.push("bootmii.ini");
        conf.write_to_file(file_path)?;
        Ok(())
    }
}
