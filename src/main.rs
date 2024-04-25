mod common;
mod eventstore;
mod querystore;
mod db_sync;


use common::Event;
use eventstore::{ListEventStore, EventStoreTrait};
use querystore::{MapQueryStore};
use db_sync::sync_querystore;


fn main() {
    let mut event_store: ListEventStore = ListEventStore::new();

    let mut query_store: MapQueryStore = MapQueryStore::new();

    let defined_events = vec!(
        Event { event_id: 1, name: "NameAppendEvent".to_string() },
        Event { event_id: 2, name: "AgeAppendEvent".to_string() }
    );

    for event in defined_events {
        event_store.add_event(event);
    }


    // let first_event: Option<&Event> = store.get_event_by_id(1);
    // if first_event.is_some() {
    //     println!("{}", first_event.unwrap().name);
    // }
    
    sync_querystore(&mut event_store, &mut query_store);
    

   for item in query_store.data {
       println!("Event {}: name: {}", item.0, item.1);
   }
}