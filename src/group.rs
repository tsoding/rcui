use super::*;

pub enum Cell {
    One(Box<dyn Widget>),
    Many(usize, Box<dyn Widget>),
}

impl Cell {
    #[allow(clippy::borrowed_box)]
    pub fn get_widget(&self) -> &Box<dyn Widget> {
        match self {
            Self::One(widget) => widget,
            Self::Many(_, widget) => widget,
        }
    }

    pub fn get_widget_mut(&mut self) -> &mut Box<dyn Widget> {
        match self {
            Self::One(widget) => widget,
            Self::Many(_, widget) => widget,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::Many(n, _) => *n,
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

    pub fn size(&self, w: f32) -> f32 {
        w / self.cells.iter().map(|x| x.size()).sum::<usize>() as f32
    }
}

impl Widget for Group {
    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        if let Some(cell) = self.cells.get_mut(self.focus) {
            cell.get_widget_mut().handle_event(context, event);
        }
    }
}
