//! Contains everything that has to do with ships, including the different types of ships, but also
//! the different modules that can be equipped.
//!
//! You usually want to use one of the top-level models like:
//! * [ShipType] for parsing which type of ship is being used.
//! * [ShipModule] for parsing any kind of ship module, including optional- and core internals,
//!   full-sized hardpoints and utility modules, hulls and cosmetics and some internal modules which
//!   are not modules which can be outfitted, but they may appear in some of the journal logs.
//! * [FighterType] and [SRVType] are small enums to specify their respective variants.

pub use models::blueprint::Blueprint;
pub use models::blueprint_modifier::BlueprintModifier;
pub use models::fighter_loadout::FighterLoadout;
pub use models::fighter_type::FighterType;
pub use models::ship_module::module_class::ModuleClass;
pub use models::ship_module::module_class::ModuleClassError;
pub use models::ship_module::ship_cockpit_module::ShipCockpitModule;
pub use models::ship_module::ship_cockpit_module::ShipCockpitModuleError;
pub use models::ship_module::ship_decal::ShipDecal;
pub use models::ship_module::ship_decal::ShipDecalError;
pub use models::ship_module::ship_hardpoint_module::hardpoint_module::HardpointModule;
pub use models::ship_module::ship_hardpoint_module::hardpoint_mounting::HardpointMounting;
pub use models::ship_module::ship_hardpoint_module::hardpoint_size::HardpointSize;
pub use models::ship_module::ship_hardpoint_module::hardpoint_size::HardpointSizeError;
pub use models::ship_module::ship_hardpoint_module::hardpoint_type::HardpointType;
pub use models::ship_module::ship_hardpoint_module::ShipHardpointModule;
pub use models::ship_module::ship_hardpoint_module::ShipHardpointModuleError;
pub use models::ship_module::ship_internal_module::armor_grade::ArmorGrade;
pub use models::ship_module::ship_internal_module::armor_grade::ArmorGradeError;
pub use models::ship_module::ship_internal_module::armor_module::ArmorModule;
pub use models::ship_module::ship_internal_module::armor_module::ArmorModuleError;
pub use models::ship_module::ship_internal_module::internal_module::InternalModule;
pub use models::ship_module::ship_internal_module::internal_type::InternalType;
pub use models::ship_module::ship_internal_module::ShipInternalModule;
pub use models::ship_module::ship_internal_module::ShipInternalModuleError;
pub use models::ship_module::ship_nameplate::ShipNameplate;
pub use models::ship_module::ship_nameplate::ShipNameplateError;
pub use models::ship_module::ship_paint_job::ShipPaintJob;
pub use models::ship_module::ship_paint_job::ShipPaintJobError;
pub use models::ship_module::ship_voicepack::ShipVoicepack;
pub use models::ship_module::ship_voicepack::ShipVoicepackError;
pub use models::ship_module::ShipModule;
pub use models::ship_slot::ShipSlot;
pub use models::ship_slot::ShipSlotError;
pub use models::ship_slot::ShipSlotKind;
pub use models::ship_type::ShipType;
pub use models::ship_type::ShipTypeError;
pub use models::srv_type::SRVType;

mod models;
