use crate::common::Event;

pub struct ListEventStore {
    pub events: Vec<Event>,
}


impl ListEventStore {
    pub fn new() -> ListEventStore {
        ListEventStore { events: vec![] }
    }
}

pub trait EventStoreTrait {

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
