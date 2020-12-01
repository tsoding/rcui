use super::*;

pub enum Cell {
    One(Box<dyn Widget>),
    Many(usize, Box<dyn Widget>),
    Fixed(f32, Box<dyn Widget>),
}

impl Cell {
    #[allow(clippy::borrowed_box)]
    pub fn get_widget(&self) -> &Box<dyn Widget> {
        match self {
            Self::One(widget) => widget,
            Self::Many(_, widget) => widget,
            Self::Fixed(_, widget) => widget,
        }
    }

    pub fn get_widget_mut(&mut self) -> &mut Box<dyn Widget> {
        match self {
            Self::One(widget) => widget,
            Self::Many(_, widget) => widget,
            Self::Fixed(_, widget) => widget,
        }
    }

    pub fn size(&self, cell_size: f32) -> f32 {
        match self {
            Self::One(_) => cell_size,
            Self::Many(n, _) => cell_size * *n as f32,
            Self::Fixed(size, _) => *size,
        }
    }
}

pub struct Group {
    pub cells: Vec<Cell>,
    pub focus: usize,
}

impl Group {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells, focus: 0 }
    }

    pub fn wrap(cells: Vec<Cell>) -> Box<Self> {
        Box::new(Self::new(cells))
    }

    pub fn focus_next(&mut self) {
        if !self.cells.is_empty() {
            self.focus = (self.focus + 1) % self.cells.len()
        }
    }

    pub fn focus_prev(&mut self) {
        if !self.cells.is_empty() {
            if self.focus == 0 {
                self.focus = self.cells.len() - 1;
            } else {
                self.focus -= 1;
            }
        }
    }

    pub fn cell_size(&self, mut size: f32) -> f32 {
        let mut count = 0;

        for cell in self.cells.iter() {
            match cell {
                Cell::One(_) => count += 1,
                Cell::Many(n, _) => count += n,
                Cell::Fixed(s, _) => size -= s,
            }
        }

        size / count as f32
    }
}

impl Widget for Group {
    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        if let Some(cell) = self.cells.get_mut(self.focus) {
            cell.get_widget_mut().handle_event(context, event);
        }
    }
}
