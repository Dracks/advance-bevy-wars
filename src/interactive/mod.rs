use bevy::prelude::*;

pub type PlayerId = u8;
pub type CapturePoints = u8;

#[derive(Component, Debug)]
pub struct Owner(pub PlayerId);

#[derive(Component, Debug)]
pub struct Life(pub u8);

impl Life {
    pub fn new() -> Self {
        Self(100)
    }
}

#[derive(Component, Debug)]
pub struct Capturable {
    points: CapturePoints,
    current: Option<Capture>,
}

impl Capturable {
    pub fn new(points: CapturePoints) -> Self {
        Self {
            points,
            current: None,
        }
    }

    pub fn capture(&mut self, player: PlayerId, points: CapturePoints) -> bool {
        let capture = match &mut self.current {
            None => {
                self.current = Some(Capture::new(self.points, player));
                self.current.as_mut().unwrap()
            }
            Some(current) => {
                if current.player != player {
                    *current = Capture::new(self.points, player);
                }
                current
            }
        };

        if capture.pending < points {
            self.current = None;
            true
        } else {
            capture.pending -= points;

            false
        }
    }
}

#[derive(Debug)]
pub struct Capture {
    pending: CapturePoints,
    player: PlayerId,
}

impl Capture {
    fn new(pending: CapturePoints, player: PlayerId) -> Self {
        Self { pending, player }
    }
}

#[derive(Component, Debug)]
// Will be nice to be able to force to have owner
pub struct Income(pub u32);

enum MovementType {
    foot,
    weels,
}

#[derive(Component)]
pub struct Movement {
    mov_type: MovementType,
    movements: u8,
}
