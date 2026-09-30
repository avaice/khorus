use crate::pack::PackFiles;

pub struct BuiltinPack {
    pub id: &'static str,
    files: &'static [(&'static str, &'static [u8])],
}

impl BuiltinPack {
    pub fn files(&self) -> PackFiles {
        self.files
            .iter()
            .map(|(name, bytes)| (name.to_string(), bytes.to_vec()))
            .collect()
    }
}

pub const BUILTIN_PACKS: &[BuiltinPack] = &[
    BuiltinPack {
        id: "default",
        files: &[(
            "pack.json",
            include_bytes!("../builtin-packs/default/pack.json"),
        )],
    },
    BuiltinPack {
        id: "click",
        files: &[(
            "pack.json",
            include_bytes!("../builtin-packs/click/pack.json"),
        )],
    },
];

pub fn default_pack() -> &'static BuiltinPack {
    &BUILTIN_PACKS[0]
}

pub fn find(id: &str) -> Option<&'static BuiltinPack> {
    BUILTIN_PACKS.iter().find(|pack| pack.id == id)
}
