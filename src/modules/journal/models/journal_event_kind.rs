use crate::backpack::Backpack;
use crate::logs::LogEvent;
use crate::market::Market;
use crate::modules::cargo::Cargo;
use crate::modules::outfitting::Outfitting;
use crate::modules_info::ModulesInfo;
use crate::nav_route::NavRoute;
use crate::ship_locker::ShipLocker;
use crate::shipyard::Shipyard;
use crate::status::Status;

/// This event is fired from the [LiveJournalDirReader] when any change happens in the journal
/// directory and includes all the possible models that could have been updated.
#[derive(Debug, Clone, PartialEq)]
// The large enum variant is allowed here as this is usually allocated by the reader anyway and
// adding another box here wouldn't be that useful. Also even though it's large, it's not huge.
#[allow(clippy::large_enum_variant)]
pub enum JournalEventKind {
    LogEvent(LogEvent),
    StatusEvent(Status),
    OutfittingEvent(Outfitting),
    ShipyardEvent(Shipyard),
    MarketEvent(Market),
    NavRoute(NavRoute),
    ModulesInfo(ModulesInfo),
    Backpack(Backpack),
    Cargo(Cargo),
    ShipLocker(ShipLocker),
}
