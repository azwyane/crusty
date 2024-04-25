use crate::common::Event;
use crate::eventstore::{EventStoreTrait};
use crate::querystore::{QueryStoreTrait};


pub fn sync_querystore(eventstore: &mut dyn EventStoreTrait, querystore: &mut dyn QueryStoreTrait) {
    let events: &Vec<Event> = eventstore.get_events();
    for event in events {
        querystore.insert_item(event.event_id.to_string().clone(), event.name.clone());
    }
}
