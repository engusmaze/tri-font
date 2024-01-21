use super::*;

impl Font {
    pub fn read(data: &[u8]) -> Result<Font> {
        let face = ttf_parser::Face::parse(data, 0)?;

        let scale = face.units_per_em() as f32;

        let cell_height = face.height() as f32 / scale;

        let mut font = Font {
            map: HashMap::default(),
            cell_height,
        };

        // for name in face.names() {
        //     if name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode() {
        //         if let Some(family_name) = name.to_string() {
        //             let language = name.language();
        //             family_names.push(format!(
        //                 "{} ({}, {})",
        //                 family_name,
        //                 language.primary_language(),
        //                 language.region()
        //             ));
        //         }
        //     }
        // }

        // for id in 0..face.number_of_glyphs() {
        //     let glyph_id = ttf_parser::GlyphId(id);

        //     face.glyph_name()

        //     match triangulate::glyph(&face, glyph_id) {
        //         Ok(character) => {
        //             font.map.insert(c, character);
        //         }
        //         Err(err) => error = Some(err),
        //     }
        // }
        for table in face
            .tables()
            .cmap
            .expect("Failed to get characters")
            .subtables
        {
            if !table.is_unicode() {
                continue;
            }
            let mut error = None;
            table.codepoints(|c| {
                if let Some(c) = char::from_u32(c) {
                    if let Some(glyph_id) = face.glyph_index(c) {
                        match triangulate::glyph(&face, glyph_id, scale) {
                            Ok(character) => {
                                font.map.insert(c, character);
                            }
                            Err(err) => error = Some(err),
                        }
                    }
                }
            });
            if let Some(error) = error {
                return Err(error);
            }
        }

        Ok(font)
    }
}
