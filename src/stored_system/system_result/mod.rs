use aion_event::prelude::Event;

pub struct SystemResult {
    spawn_system_associated_event: bool,
    spawn_events: Vec<Event>
}

impl Default for SystemResult {
    fn default() -> Self {
        Self {
            spawn_system_associated_event: true,
            spawn_events: Vec::new()
        }
    }
}

impl SystemResult {
    pub fn with(mut self, event: Event) -> Self {
        self.spawn_events.push(event);

        self
    }

    pub fn disable_associated_event(mut self) -> Self {
        self.spawn_system_associated_event = false;

        self
    }

    pub fn enable_associated_event(mut self) -> Self {
        self.spawn_system_associated_event = true;

        self
    }
}