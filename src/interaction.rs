use std::ops::RangeInclusive;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use eyre::eyre;
use strum::IntoEnumIterator;

pub fn select_from_enum<T: IntoEnumIterator + ToString>(prompt: &str) -> eyre::Result<usize> {
    let items: Vec<_> = T::iter().map(|item| item.to_string()).collect();
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&items[..])
        .interact()
        .map_err(|e| eyre!(e))
}

pub fn input_with_range(prompt: &str, default: u8, range: RangeInclusive<u8>) -> eyre::Result<u8> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .validate_with(move |input: &u8| {
            if range.contains(input) {
                Ok(())
            } else {
                Err(format!(
                    "Input must be between {} and {}",
                    range.start(),
                    range.end()
                ))
            }
        })
        .interact()
        .map_err(|e| eyre!(e))
}

pub fn string_input(prompt: &str, default: String) -> eyre::Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(|e| eyre!(e))
}
