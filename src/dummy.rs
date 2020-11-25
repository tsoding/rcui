use super::*;

#[derive(Default)]
pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Widget for Dummy {}
