use std::sync::LazyLock;

use ronald_core::system::{CpcModel, memory::RomSlot};

use crate::system_config::{AutoConfigRule, OriginalRom, RomInfo, RomLanguage, RomVariant};

pub static ORIGINAL_ROMS: LazyLock<[OriginalRom; 17]> = LazyLock::new(|| {
    [
        OriginalRom {
            info: RomInfo::from((
                "CPC 464 OS",
                "b869b47a809ec463bfe67999ed57770bb4bf33827b8551148cabe4a0592f3f43",
                Some(RomVariant::Language(RomLanguage::Danish)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 464 OS",
                "4be45a49652c397d0771d516413ed955ec2f84d5e349d1b7c519c6088c8772fd",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 464 OS",
                "e1ad76d9e79afbb85e51e53ae303ec3f811ca0e428291e0b8a8c6b6e47c45f49",
                Some(RomVariant::Language(RomLanguage::Spanish)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 464 OS",
                "b57872c97d569a1968816fcaf359de4b6bf5c188e7dbd0954761cc552b25d327",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 664 OS",
                "31e57547f5405e6216345a7d6d059791208ce62e483ec7d023c7b6bcc80981f9",
                None,
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc664],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 6128 OS",
                "c9b740d79546a988e12b86b16beb3dbd164b740cac64fe61afa2fcc8e591009c",
                Some(RomVariant::Language(RomLanguage::Danish)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 6128 OS",
                "bf87e68ff847052fecc98f84f286b23edc49d607ea8b140fc93e90259cc98ffb",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 6128 OS",
                "95ee6b221aa48f2f04641948463372f6daa7ac3b56206598e5de01a49d37a071",
                Some(RomVariant::Language(RomLanguage::Spanish)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "CPC 6128 OS",
                "4b2aab13cb56e315be29efc6e0f041b58145e8a63522da69eb119b9bb4bf7520",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Lower),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.0",
                "fc05877430e0d5b500e470d73f833cc762c8654ea908f9da0d7d684c7669735d",
                Some(RomVariant::Language(RomLanguage::Danish)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.0",
                "6226d89d17d99c78171ac69a9144464fc824a097e2b1cc876425b5fececa9bdd",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.0",
                "ff6fbb6e12808e7d32c9217813d730df7321b4b5e499a45e3eac21b421dcf729",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.1 (CPC 664)",
                "1260317b7b631d36bccc8b08c93b0f99a530bddc2ff21e858d71224b4d7dab7c",
                None,
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc664],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.1",
                "fc9f747896664b6c89f6fd382691cf22d0840dbf8579b35af531b55b60f54aa5",
                Some(RomVariant::Language(RomLanguage::French)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.1",
                "8050045437f5127452ee51ac7cf762c748b180e27cea7884226f9d7594b7843e",
                Some(RomVariant::Language(RomLanguage::Spanish)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "Locomotive BASIC 1.1",
                "7630682a0fd8deaa4514954c1a256224d3ae4cb6e3dd4da8e5ad8e1c5c55adc7",
                Some(RomVariant::Language(RomLanguage::English)),
                Some(RomSlot::Upper(0)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc6128],
                requires_disk_drive: false,
            },
        },
        OriginalRom {
            info: RomInfo::from((
                "AMSDOS 0.5",
                "47085932df883b6d86101cfa12978846432e7aed3f7ccd738954e4c099220cd7",
                None,
                Some(RomSlot::Upper(7)),
            )),
            auto_config_rule: AutoConfigRule {
                models: &[CpcModel::Cpc464, CpcModel::Cpc664, CpcModel::Cpc6128],
                requires_disk_drive: true,
            },
        },
    ]
});
