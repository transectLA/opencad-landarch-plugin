//! Process-local plugin state.
//!
//! External OCS plugins run in their own runner process; host-side `dyn Any`
//! state does not cross IPC. This mirrors the working Land Survey plugin's
//! process-global `Mutex` approach.

use std::sync::{Mutex, MutexGuard, OnceLock};

static STATE: OnceLock<Mutex<LandArchState>> = OnceLock::new();

#[derive(Debug)]
pub struct LandArchState {
    pub last_plant: String,
    pub last_material: String,
    pub last_furnishing: String,
    pub last_amenity: String,
    pub created_this_session: usize,
}

impl Default for LandArchState {
    fn default() -> Self {
        Self {
            last_plant: "QAGR".to_string(),
            last_material: "PAV-CONC".to_string(),
            last_furnishing: "BENCH-01".to_string(),
            last_amenity: "DRINK-01".to_string(),
            created_this_session: 0,
        }
    }
}

pub fn state() -> MutexGuard<'static, LandArchState> {
    STATE
        .get_or_init(|| Mutex::new(LandArchState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub fn mark_created() { state().created_this_session += 1; }
