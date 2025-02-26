use crate::Event;
use std::ops::Add;

/// Represents an active touch on the touch device.
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

/// The phase of a touch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Start,
    End,
    Cancel,
    Move,
}

/// A trait for objects that can represent the state of a touch device.
pub trait TouchInterface {
    /// A type that can uniquely represent a touch.
    type TouchId: PartialEq;

    /// The numeric type used for touch coordinates.
    type Coord: Copy + Default + Add<Output = Self::Coord>;

    /// Returns a reference to the first registered touch, if any.
    fn first_touch(&self) -> Option<&Touch<Self::TouchId, Self::Coord>>;

    /// Returns a touch by its `id`, if it exists.
    fn touch<I: AsRef<Self::TouchId>>(&self, id: I) -> Option<&Touch<Self::TouchId, Self::Coord>>;

    /// Returns an iterator over all active touches.
    ///
    /// (This includes touches that have been released this frame.)
    fn touches(&self) -> impl Iterator<Item = &Touch<Self::TouchId, Self::Coord>>;

    /// Register a touch event.
    fn touch_event<I, P>(&mut self, id: I, position: [Self::Coord; 2], phase: P) -> &mut Self
    where
        I: Into<Self::TouchId>,
        P: Into<TouchPhase>;

    /// Clears the tapped/released state of active touches. Should be called at the end of each frame.
    fn clear_taps(&mut self) -> &mut Self;

    /// Convenience method for handling events. The type of event, `E`, will
    /// vary depending on the windowing library being used.
    fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
        self
    }
}

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
        Self::new()
    }
}

impl<Id, Coord> Touchpad<Id, Coord>
where
    Id: PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    pub fn new() -> Self {
        Touchpad {
            touches: Vec::with_capacity(4),
        }
    }
}

impl<Id, C> TouchInterface for Touchpad<Id, C>
where
    Id: PartialEq,
    C: Copy + Default + Add<Output = C>,
{
    type TouchId = Id;
    type Coord = C;

    fn first_touch(&self) -> Option<&Touch<Self::TouchId, Self::Coord>> {
        self.touches.first()
    }

    fn touch<I: AsRef<Self::TouchId>>(&self, id: I) -> Option<&Touch<Self::TouchId, Self::Coord>> {
        self.touches.iter().find(|t| &t.id == id.as_ref())
    }

    fn touches(&self) -> impl Iterator<Item = &Touch<Self::TouchId, Self::Coord>> {
        self.touches.iter()
    }

    fn touch_event<I, P>(&mut self, id: I, position: [Self::Coord; 2], phase: P) -> &mut Self
    where
        I: Into<Self::TouchId>,
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
        self
    }

    fn clear_taps(&mut self) -> &mut Self {
        for touch in &mut self.touches {
            touch.tapped = false;
        }
        self.touches.retain(|t| !t.released);
        self
    }
}
