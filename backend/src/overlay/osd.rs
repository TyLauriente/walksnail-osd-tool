use image::{imageops::overlay, RgbaImage};

use crate::{
    font,
    osd::{self, OsdOptions},
};

#[inline]
pub fn overlay_osd(image: &mut RgbaImage, osd_frame: &osd::Frame, font: &font::FontFile, osd_options: &OsdOptions) {
    // TODO: check if this can be run in parallel
    for character in &osd_frame.glyphs {
        if character.index == 0 || osd_options.get_mask(&character.grid_position) {
            continue;
        }
        if let Some(character_image) = font.characters.get(character.index as usize) {
            let grid_position = &character.grid_position;
            let (char_width, char_height) = character_image.dimensions();
            overlay(
                image,
                character_image,
                (grid_position.x as i32 * char_width as i32 + osd_options.position.x).into(),
                (grid_position.y as i32 * char_height as i32 + osd_options.position.y).into(),
            )
        }
    }
}
