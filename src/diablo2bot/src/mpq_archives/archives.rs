use encoding_rs::WINDOWS_1250;
use mpq_reader::Archive;

use crate::{
    enums::{
        act::Act,
        character_class::CharacterClass,
        errors::{ArchiveError, MpqFileFromUtf8Error},
        palette::Palette,
    },
    pal_pl2::PalPl2Bytes,
};
use std::{fmt, io::Error};

use super::{
    dc6_inventory_raw_bytes::Dc6InventoryRawBytes, dc6_raw_bytes::Dc6RawBytes,
    dc6_stash_raw_bytes::Dc6StashRawBytes, dcc_bytes::DccBytes, excel_automap::ExcelAutomapRawText,
    excel_levels::ExcelLevelsRawText, excel_monstats::ExcelMonstatsRawText,
    excel_monstats2::ExcelMonstats2RawText, excel_skills::ExcelSkillsRawText,
    rand_transform_palettes::RandTransformRawBytes, strings_table::StringsTableRaw,
};

#[derive(Clone)]
pub struct Archives {
    data: Archive,
    expansion: Archive,
    patch: Archive,
}

enum Utf8ExcelFileType {
    Monstats,
    Monstats2,
    AutoMap,
    Levels,
}

enum ArchiveType {
    Data,
    Expansion,
    Patch,
}

pub enum StringTableType {
    Data,
    Expansion,
    Patch,
}

impl StringTableType {
    fn to_archive_type(&self) -> ArchiveType {
        match self {
            Self::Data => ArchiveType::Data,
            Self::Expansion => ArchiveType::Expansion,
            Self::Patch => ArchiveType::Patch,
        }
    }
}

impl fmt::Display for Utf8ExcelFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Monstats => "monstats",
            Self::Monstats2 => "monstats2",
            Self::AutoMap => "AutoMap",
            Self::Levels => "Levels",
        };

        write!(f, "{s}")
    }
}

impl fmt::Display for StringTableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Data => "string",
            Self::Expansion => "expansionstring",
            Self::Patch => "patchstring",
        };

        write!(f, "{s}")
    }
}

impl Archives {
    pub fn new(folder_path: &str) -> Self {
        let data = Archive::open(format!("{folder_path}/d2data.mpq")).unwrap(); // TODO Throw error
        let expansion = Archive::open(format!("{folder_path}/d2exp.mpq")).unwrap(); // TODO Throw error
        let patch = Archive::open(format!("{folder_path}/patch_d2.mpq")).unwrap(); // TODO Throw error

        Self {
            data,
            expansion,
            patch,
        }
    }

    pub fn extract_pal_pl2_bytes(&mut self, palette: Palette) -> Result<PalPl2Bytes, Error> {
        let archive_type = match palette {
            Palette::Act(Act::Act5) => ArchiveType::Expansion,
            _ => ArchiveType::Data,
        };

        let bytes = self.read_mpq_file(
            archive_type,
            &format!("data/global/palette/{palette}/Pal.PL2"),
        );

        match bytes {
            Ok(bytes) => Ok(PalPl2Bytes::new(bytes)),
            Err(e) => Err(e),
        }
    }

    pub fn extract_rand_transform_palettes(&mut self) -> Result<RandTransformRawBytes, Error> {
        let bytes =
            self.read_mpq_file(ArchiveType::Data, "data/global/monsters/RandTransforms.dat");

        match bytes {
            Ok(bytes) => Ok(RandTransformRawBytes::new(bytes)),
            Err(e) => Err(e),
        }
    }

    pub fn extract_font_16_bytes(&mut self) -> Result<Dc6RawBytes, Error> {
        self.extract_dc6_file_bytes(ArchiveType::Data, "data/local/font/latin/font16.DC6")
    }

    pub fn extract_inventory_dc6_bytes(&mut self) -> Result<Dc6InventoryRawBytes, Error> {
        let bytes = self.read_mpq_file(ArchiveType::Expansion, "data/global/ui/PANEL/invchar6.DC6");

        match bytes {
            Ok(bytes) => Ok(Dc6InventoryRawBytes::new(Dc6RawBytes::new(bytes))),
            Err(e) => Err(e),
        }
    }

    pub fn extract_stash_dc6_bytes(&mut self) -> Result<Dc6StashRawBytes, Error> {
        let bytes = self.read_mpq_file(
            ArchiveType::Expansion,
            "data/global/ui/PANEL/TradeStash.DC6",
        );

        match bytes {
            Ok(bytes) => Ok(Dc6StashRawBytes::new(Dc6RawBytes::new(bytes))),
            Err(e) => Err(e),
        }
    }

    pub fn extract_excel_skills_raw_text(&mut self) -> Result<ExcelSkillsRawText, Error> {
        let bytes = self.read_mpq_file(ArchiveType::Expansion, "data/global/excel/skills.txt")?;

        let (decoded, _, had_errors) = WINDOWS_1250.decode(&bytes);

        if had_errors {
            println!(
                "Decoding had errors, which were replaced with the Unicode replacement character."
            );
        }

        let text = decoded.to_string();

        Ok(ExcelSkillsRawText::new(text))
    }

    pub fn extract_string_table(
        &mut self,
        string_table_type: StringTableType,
    ) -> Result<StringsTableRaw, Error> {
        let file_path = format!("data/local/lng/eng/{}.tbl", string_table_type);
        let bytes = self.read_mpq_file(string_table_type.to_archive_type(), &file_path)?;

        Ok(StringsTableRaw::new(bytes))
    }

    pub fn extract_excel_monstats_raw_text(
        &mut self,
    ) -> Result<ExcelMonstatsRawText, ArchiveError> {
        Ok(ExcelMonstatsRawText::new(
            self.extract_excel_file_raw_text(Utf8ExcelFileType::Monstats)?,
        ))
    }

    pub fn extract_excel_monstats2_raw_text(
        &mut self,
    ) -> Result<ExcelMonstats2RawText, ArchiveError> {
        Ok(ExcelMonstats2RawText::new(
            self.extract_excel_file_raw_text(Utf8ExcelFileType::Monstats2)?,
        ))
    }

    pub fn extract_excel_automap_raw_text(&mut self) -> Result<ExcelAutomapRawText, ArchiveError> {
        Ok(ExcelAutomapRawText::new(
            self.extract_excel_file_raw_text(Utf8ExcelFileType::AutoMap)?,
        ))
    }

    pub fn extract_excel_levels_raw_text(&mut self) -> Result<ExcelLevelsRawText, ArchiveError> {
        Ok(ExcelLevelsRawText::new(
            self.extract_excel_file_raw_text(Utf8ExcelFileType::Levels)?,
        ))
    }

    fn get_excel_filetype_archive(&mut self, excel_file_type: Utf8ExcelFileType) -> ArchiveType {
        // TODO I think we should try d2_patch first, then d2exp and then d2data
        match excel_file_type {
            Utf8ExcelFileType::AutoMap => ArchiveType::Expansion,
            Utf8ExcelFileType::Monstats
            | Utf8ExcelFileType::Monstats2
            | Utf8ExcelFileType::Levels => ArchiveType::Patch,
        }
    }

    fn extract_excel_file_raw_text(
        &mut self,
        excel_file_type: Utf8ExcelFileType,
    ) -> Result<String, ArchiveError> {
        let file_path = format!("data/global/excel/{}.txt", excel_file_type);

        let archive = self.get_excel_filetype_archive(excel_file_type);

        let bytes = self.read_mpq_file(archive, &file_path)?;

        String::from_utf8(bytes).map_err(|e| {
            ArchiveError::MpqFileFromUtf8Error(MpqFileFromUtf8Error::new(&file_path, e))
        })
    }

    pub fn extract_palshift_palettes_bytes(
        &mut self,
        monster_code: &str,
    ) -> Result<Vec<u8>, Error> {
        self.read_mpq_file(
            ArchiveType::Data,
            &format!("data/global/monsters/{monster_code}/COF/palshift.dat",),
        )
    }

    pub fn extract_item_inventory_sprite(
        &mut self,
        item_file_name: &str,
    ) -> Result<Dc6RawBytes, Error> {
        self.extract_dc6_file_bytes(
            ArchiveType::Data,
            &format!("data/global/items/{item_file_name}.DC6"),
        )
    }

    pub fn extract_map_sprites(&mut self) -> Result<Dc6RawBytes, Error> {
        self.extract_dc6_file_bytes(ArchiveType::Expansion, "data/global/ui/AUTOMAP/MaxiMap.dc6")
    }

    pub fn extract_general_skill_icon_dc6_bytes(&mut self) -> Result<Dc6RawBytes, Error> {
        self.extract_dc6_file_bytes(ArchiveType::Data, "data/global/ui/SPELLS/Skillicon.DC6")
    }

    pub fn extract_class_skill_icon_dc6_bytes(
        &mut self,
        class: CharacterClass,
    ) -> Result<Dc6RawBytes, Error> {
        let archive_type = match class {
            CharacterClass::Assassin | CharacterClass::Druid => ArchiveType::Expansion,
            _ => ArchiveType::Data,
        };

        let class_code = class.get_class_code();

        let file_path = format!("data/global/ui/SPELLS/{class_code}Skillicon.dc6");

        self.extract_dc6_file_bytes(archive_type, &file_path)
    }

    fn extract_dc6_file_bytes(
        &mut self,
        archive_type: ArchiveType,
        file_path: &str,
    ) -> Result<Dc6RawBytes, Error> {
        let bytes = self.read_mpq_file(archive_type, file_path);

        match bytes {
            Ok(bytes) => Ok(Dc6RawBytes::new(bytes)),
            Err(e) => Err(e),
        }
    }

    pub fn extract_dcc_file_bytes(&mut self, file_path: &str) -> Result<DccBytes, Error> {
        match self._extract_dcc_file_bytes(ArchiveType::Data, file_path) {
            Ok(bytes) => Ok(bytes),
            Err(_) => self._extract_dcc_file_bytes(ArchiveType::Expansion, file_path),
        }
    }

    fn _extract_dcc_file_bytes(
        &mut self,
        archive_type: ArchiveType,
        file_path: &str,
    ) -> Result<DccBytes, Error> {
        let bytes = self.read_mpq_file(archive_type, file_path);

        match bytes {
            Ok(bytes) => Ok(DccBytes::new(bytes)),
            Err(e) => Err(e),
        }
    }

    fn read_mpq_file(
        &mut self,
        archive_type: ArchiveType,
        file_name: &str,
    ) -> Result<Vec<u8>, Error> {
        let archive = self.get_archive(archive_type);

        let file = archive.open_file(file_name)?;
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        file.read(archive, &mut buf)?;

        Ok(buf)
    }

    fn get_archive(&mut self, archive_type: ArchiveType) -> &mut Archive {
        match archive_type {
            ArchiveType::Data => &mut self.data,
            ArchiveType::Expansion => &mut self.expansion,
            ArchiveType::Patch => &mut self.patch,
        }
    }
}
