use crate::Event;
use std::ops::Add;

/// A structure representing the current state of touches on a touch device.
#[derive(Debug, Clone)]
pub struct Touchpad<Id, Coord>
where
    Id: PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    touches: Vec<Touch<Id, Coord>>,
}

impl<Id, Coord> Default for Touchpad<Id, Coord>
where
    Id: PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    fn default() -> Self {
        Touchpad {
            touches: Vec::with_capacity(4),
        }
    }
}

impl<Id, Coord> Touchpad<Id, Coord>
where
    Id: PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the first registered touch, if any.
    pub fn first_touch(&self) -> Option<&Touch<Id, Coord>> {
        self.touches.first()
    }

    /// Returns a touch by its `id`, if it exists.
    pub fn touch<I: AsRef<Id>>(&self, id: I) -> Option<&Touch<Id, Coord>> {
        self.touches.iter().find(|t| &t.id == id.as_ref())
    }

    /// Returns an iterator over all active touches.
    ///
    /// (This includes touches that have been released this frame.)
    pub fn touches(&self) -> impl Iterator<Item = &Touch<Id, Coord>> {
        self.touches.iter()
    }

    /// Register a touch event.
    pub fn touch_event<I, P>(&mut self, id: I, position: [Coord; 2], phase: P)
    where
        I: Into<Id>,
        P: Into<TouchPhase>,
    {
        let id = id.into();
        let existing_touch = self.touches.iter_mut().find(|t| t.id == id);
        let existing_touch = match existing_touch {
            Some(t) => t,
            None => {
                self.touches.push(Touch {
                    id,
                    position,
                    tapped: false,
                    released: false,
                });
                self.touches.last_mut().unwrap()
            }
        };

        match phase.into() {
            TouchPhase::Start => existing_touch.tapped = true,
            TouchPhase::Cancel | TouchPhase::End => existing_touch.released = true,
            _ => (),
        }

        existing_touch.position = position;
    }

    /// Clears the tapped/released state of active touches. Should be called at the end of each frame.
    pub fn clear_taps(&mut self) -> &mut Self {
        for touch in &mut self.touches {
            touch.tapped = false;
        }
        self.touches.retain(|t| !t.released);
        self
    }

    /// Convenience method for handling events. The type of event, `E`, will
    /// vary depending on the windowing library being used.
    pub fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Touch<Id, Coord>
where
    Id: PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    pub id: Id,
    pub position: [Coord; 2],
    pub tapped: bool,
    pub released: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Start,
    End,
    Cancel,
    Move,
}
