/// An event in an events array can be optional.
pub type Event<T> = Option<T>;

/// An events array.
#[derive(Clone, PartialEq)]
pub struct Events<T, const LEN: usize = 16> {
    events: [Event<T>; LEN],
    length: u8,
}

impl<T, const LEN: usize> Events<T, LEN> {
    pub fn new() -> Self {
        Self {
            events: todo!(),
            length: 0,
        }
    }

    /// Change all the events to [None] and reset the length.
    pub fn reset(&mut self) {
        for event in &mut self.events {
            *event = None;
        }

        self.length = 0;
    }

    /// Append an event to the list.
    pub fn append(&mut self, event: T) -> bool {
        if self.length >= 16 {
            return false;
        }

        self.events[self.length as usize] = Some(event);
        self.length = self.length + 1;

        return true;
    }
}
