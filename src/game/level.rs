use crate::game::global::{Position};
use std::io::{self, Stdout};
use crossterm::{cursor::{self, MoveTo}, queue, style::{Color, Print, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum FoodKind {
    Cherry,
    Mouse,
    Bomb,
}

#[derive(Debug)]
pub struct Food {
    pub kind: FoodKind,
    pub meals: i16,
    pub symbol: char,
    pub color: Color,
    pub pos: Position,
}

impl Food {
    pub fn new(kind: FoodKind, pos: Position) -> Self {
        let (meals, symbol, color) = match kind {
            FoodKind::Cherry => (1, '🍒', Color::Rgb { r: 255, g: 0, b: 0 }),
            FoodKind::Mouse => (2, '🐁', Color::Rgb { r: 50, g: 60, b: 70 }),
            FoodKind::Bomb => (-10, '💣', Color::Rgb { r: 0, g: 0, b: 0 }),
        };

        Self {
            kind,
            pos,
            meals,
            symbol,
            color,
        }
    }
}

#[derive(Debug)]
pub struct Level {
    pub width: u16,
    pub height: u16,
    pub background: char,
    pub border: char,
    pub border_width: u16,
    pub border_height: u16,
    pub fg_color: Color,
    pub bg_color: Color,
    pub bg_color_range: Vec<Color>,
    pub food_vec: Vec<Food>,
    pub max_food: usize
}

impl Level {
    pub fn new (mut width: u16, mut height: u16) -> Self {
        // Force even dimensions
        if width % 2 == 0 { width += 1; }
        if height % 2 == 0 { height += 1; }
        
        return  Self {
            width,
            height,
            background: ' ', // \u{2591}
            border: '\u{2588}',
            border_width: 2,
            border_height: 1,
            fg_color: Color::Rgb{ r: 10, g: 100, b: 120 },
            bg_color: Color::Rgb{ r: 230, g: 40, b: 130 },
            bg_color_range: vec![],
            food_vec: vec![],
            max_food: 4
        };
    }

    pub fn total_height(&self) -> u16 {
        self.height + self.border_height * 2
    }

    pub fn total_width(&self) -> u16 {
        self.width + self.border_width * 2
    }
    
    pub fn generate (&mut self, stdout: &mut Stdout) -> io::Result<()> {
        queue!(stdout, Clear(ClearType::All))?;
    
        // Generate bg_color_range
        {
            if let Color::Rgb { r, g, b} = self.bg_color {
                for i in 0..self.total_height() {
                    let new_g = g.saturating_add((10 * i) as u8);
                    self.bg_color_range.push(Color::Rgb { r,  g: new_g, b});
                }
            }
        }
        
        // Generate level
        for y in 0..self.total_height() {
            for x in 0..self.total_width() {
                queue!(stdout, cursor::MoveTo(x, y))?;
                
                queue!(stdout, SetForegroundColor(self.fg_color))?;
                if let Some(color_ref) = self.bg_color_range.get(y as usize){
                    queue!(stdout, SetBackgroundColor(*color_ref))?;
                }
    
                if x < self.border_width || x > self.width + self.border_width - 1 || y < self.border_height || y > self.height + self.border_height - 1 {
                    queue!(stdout, Print(&self.border))?;
                } else {
                    queue!(stdout, Print(&self.background))?;
                }
            }
        }

        Ok(())
    }

    pub fn rng_food(&mut self) -> Food {
        let rng_pos = self.rng_pos(None);

        let food = match rand::rng().random_range(0..=2) {
            0 => Food::new(FoodKind::Cherry, rng_pos),
            1 => Food::new(FoodKind::Mouse, rng_pos),
            _ => Food::new(FoodKind::Bomb, rng_pos),
        };

        food
    }

    pub fn rng_pos(&self, offset: Option<u16>) -> Position {
        let off = offset.unwrap_or(0);
        let x = rand::rng().random_range(self.border_width + off .. self.width + self.border_width - off);
        let y = rand::rng().random_range(self.border_height + off .. self.height + self.border_height - off);
    
        Position { x: x, y: y }
    }
}


