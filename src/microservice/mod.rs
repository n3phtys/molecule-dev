use database::*;
use common::*;
use std::time::Duration;

pub trait LifecycleMicroservice {
    fn get_lifecycle(&self) -> LifecycleEvent;

    fn on_lifecycle_trigger(&mut self, app_state : &mut AppState);
}

pub enum LifecycleEvent {
    OnStartup,
    OnEntityCreation(MoleculeEntity),
    OnEntityCreationComplete(MoleculeEntity),
    OnEntityDeletion(MoleculeEntity),
    OnEntityDeletionComplete(MoleculeEntity),
    OnEntityUpdate(MoleculeEntity),
    OnEntityUpdateComplete(MoleculeEntity),
}

pub trait ScheduledMicroservice {

    fn compute_milliseconds_til_next_trigger(&self) -> Duration;

    fn on_scheduler_trigger(&mut self, app_state : &mut AppState);
}



pub trait FixedScheduledMicroservice {

    fn get_milliseconds_between_triggers(&self) -> Duration;

    fn on_scheduler_trigger(&mut self, app_state : &mut AppState);
}

