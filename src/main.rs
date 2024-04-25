

struct Event {
    name: String,
}


struct ListEventStore {
    events: Vec<Event>,
}


trait EventStoreTrait {

    fn add_event(&mut self, event: Event);

    fn get_events(&self) -> &Vec<Event>;

    fn get_event_by_id(&self, _id: i32) -> Option<&Event> { None }

}

impl EventStoreTrait for ListEventStore {
    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    fn get_event_by_id(&self, _id: i32) -> Option<&Event> {
        self.events.get(_id as usize)
    }
}

fn main() {
    let mut store: ListEventStore = ListEventStore { events: vec![] };
    store.add_event(Event { name: "AgeAppendEvent".to_string() });

    store.add_event(Event { name: "NameAppendEvent".to_string() });

    let events = store.get_events();
    for event   in events {
        println!("{}", event.name); 
    }

    let first_event: Option<&Event> = store.get_event_by_id(2);

    if first_event.is_some() {
        println!("{}", first_event.unwrap().name);
    }

}