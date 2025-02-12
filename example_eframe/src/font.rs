use std::sync::Arc;

use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};
use font_kit::{font::Font, source::SystemSource};

use crate::error::BoxError;

/// chinese hei font as fallback 
const FONT_HEI: &'static [u8; 1894471] = include_bytes!("../fonts/chinese_hei.ttf");

// chinese char '中'
const CN_C1: char = '\u{4E2D}';

/// search system fonts support chinese,and call the reg closure
/// Ok(true): found and loaded,Ok(false): not found
pub fn load_chinese_font<F,REG>(ctx: &Context, filter: F, reg: REG) -> Result<bool, BoxError>
where
    F: Fn(&Font)->bool,
    REG: Fn(&Context, FontData) -> Result<bool, BoxError>,
{
    let mut handles = Vec::with_capacity(16);
    let src = SystemSource::new();
    for h in src.all_fonts().unwrap() {
        let font = h.load().unwrap();
        let idx = font.glyph_for_char(CN_C1);
        if idx.is_some() {
            println!("Family Name: {}, Full Name: {}",font.family_name(),font.full_name());
            if filter(&font) {
                handles.push(h);
            }
        }
    }
    if !handles.is_empty() {
        let font = handles[0].load()?;
        let fdata = font.copy_font_data().unwrap();
        let fdata = fdata.as_ref().clone();
        let fdata = FontData::from_owned(fdata);
        reg(ctx, fdata)?;
        return Ok(true);
    }
    return Ok(false);
}

/// overwrite egui default font
fn overwrite_font(ctx: &Context,font: FontData,name: &str)->Result<(), BoxError> {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(name.to_owned(),
    Arc::new(font));

    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .push(name.to_owned());

    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push(name.to_owned());

    // ctx.add_font(new_font);
    // ctx.set_fonts(fonts);
    unimplemented!()
}
