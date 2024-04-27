use std::num::ParseIntError;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde::Deserialize;
use thiserror::Error;
use crate::from_str_deserialize_impl;
use crate::models::journal_event_kind::shared::ship::ship_module::module_class::{ModuleClass, ModuleClassError};
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;
use crate::models::journal_event_kind::shared::ship::ship_slot::hardpoint_size::{HardpointSize, HardpointSizeParseError};

pub mod hardpoint_module;
pub mod hardpoint_mounting;
mod hardpoint_type;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShipHardpointModule {
    pub module: HardpointModule,
    pub mounting: HardpointMounting,
    pub size: HardpointSize,
    pub class: Option<ModuleClass>,
}

#[derive(Debug, Error)]
pub enum ShipHardpointModuleError {
    #[error("Failed to parse hardpoint module: {0}")]
    FailedToParseHardpointModule(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint mounting: {0}")]
    FailedToParseHardpointMounting(#[source] serde_json::Error),

    #[error("Failed to parse hardpoint size: {0}")]
    FailedToParseHardpointSize(#[from] HardpointSizeParseError),

    #[error("Failed to parse class number: {0}")]
    FailedToParseClassNumber(#[source] ParseIntError),

    #[error("Mounting type cannot be missing when the module size is not 'tiny'")]
    MissingMounting,

    #[error(transparent)]
    ModuleClassError(#[from] ModuleClassError),

    #[error("Failed to parse ship hardpoint module: '{0}'")]
    FailedToParse(String),
}

const GRADED_HARDPOINT_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?[hH]pt_(\w+?)_size(\d+)_class(\d+)(_name;)?$"#).unwrap());
const HARDPOINT_MODULE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\$?[hH]pt_(\w+?)(_([gG]imbal|[fG]ixed|[tT]urret))?_([tT]iny|[sS]mall|[mS]edium|[lL]arge|[hH]uge)(_name;)?$"#).unwrap());

impl FromStr for ShipHardpointModule {
    type Err = ShipHardpointModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = GRADED_HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_graded_hardpoint_captures(captures);
        }

        if let Some(captures) = HARDPOINT_MODULE_REGEX.captures(s) {
            return Self::from_hardpoint_captures(captures);
        };

        return Err(ShipHardpointModuleError::FailedToParse(s.to_string()));
    }
}

impl ShipHardpointModule {
    fn from_graded_hardpoint_captures(captures: Captures) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipHardpointModuleError::FailedToParseHardpointModule(e))?;

        let class = captures.get(3)
            .expect("Should have been captured already")
            .as_str()
            .parse::<u8>()
            .map_err(|e| ShipHardpointModuleError::FailedToParseClassNumber(e))?
            .try_into()?;

        Ok(ShipHardpointModule {
            module,
            mounting: HardpointMounting::Turreted,
            size: HardpointSize::Tiny,
            class: Some(class),
        })
    }

    fn from_hardpoint_captures(captures: Captures) -> Result<ShipHardpointModule, ShipHardpointModuleError> {
        let module = captures.get(1)
            .expect("Should have been captured already")
            .as_str()
            .parse()
            .map_err(|e| ShipHardpointModuleError::FailedToParseHardpointModule(e))?;

        let size = captures.get(4)
            .expect("Should have been captured already")
            .as_str()
            .parse()?;

        // Sometimes the mounting is missing when the size is 'tiny'
        let mounting = match captures.get(3) {
            Some(capture) => capture
                .as_str()
                .parse()
                .map_err(|e| ShipHardpointModuleError::FailedToParseHardpointMounting(e)),
            None => {
                if matches!(size, HardpointSize::Tiny) {
                    Ok(HardpointMounting::Turreted)
                } else {
                    Err(ShipHardpointModuleError::MissingMounting)
                }
            }
        }?;

        Ok(ShipHardpointModule {
            module,
            mounting,
            class: Some(match &size {
                HardpointSize::Tiny => ModuleClass::I,
                _ => ModuleClass::E,
            }),
            size,
        })
    }
}

from_str_deserialize_impl!(ShipHardpointModule);

impl ShipHardpointModule {
    pub fn hardpoint_type(&self) -> HardpointType {
        self.module.hardpoint_type()
    }

    pub fn is_full_sized(&self) -> bool {
        self.module.is_full_sized()
    }

    pub fn is_utility(&self) -> bool {
        self.module.is_utility()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::journal_event_kind::shared::ship::ship_module::module_class::ModuleClass;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
    use crate::models::journal_event_kind::shared::ship::ship_module::ship_hardpoint_module::{ShipHardpointModule, ShipHardpointModuleError};
    use crate::models::journal_event_kind::shared::ship::ship_slot::hardpoint_size::HardpointSize;

    #[test]
    fn ship_hardpoint_module_test_cases_all_pass() {
        let test_cases = [
            ("$hpt_beamlaser_gimbal_medium_name;", ShipHardpointModule {
                module: HardpointModule::BeamLaser,
                mounting: HardpointMounting::Gimballed,
                size: HardpointSize::Medium,
                class: Some(ModuleClass::E),
            }),
            ("$hpt_heatsinklauncher_turret_tiny_name;", ShipHardpointModule {
                module: HardpointModule::HeatsinkLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
                class: Some(ModuleClass::I),
            }),
            ("Hpt_CausticSinkLauncher_Turret_Tiny", ShipHardpointModule {
                module: HardpointModule::CausticSinkLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
                class: Some(ModuleClass::I),
            }),
            ("$hpt_chafflauncher_tiny_name;", ShipHardpointModule {
                module: HardpointModule::ChaffLauncher,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
                class: Some(ModuleClass::I),
            }),
            ("$hpt_shieldbooster_size0_class5_name;", ShipHardpointModule {
                module: HardpointModule::ShieldBooster,
                mounting: HardpointMounting::Turreted,
                size: HardpointSize::Tiny,
                class: Some(ModuleClass::A),
            })
        ];

        for (case, expected) in test_cases {
            let result = ShipHardpointModule::from_str(case);

            dbg!(&result);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn incorrect_combination_of_missing_mounting_type_and_size_is_rejected() {
        let result = ShipHardpointModule::from_str("$hpt_chafflauncher_mediun_name;");
        assert!(result.is_err());
    }
}
