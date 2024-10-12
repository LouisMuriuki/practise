#[derive(Debug)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Products {
    pub shirts: Vec<ShirtColor>,
}

impl Products {
    pub fn give_away(&self, preference_color: Option<ShirtColor>) -> ShirtColor {
        preference_color.unwrap_or_else(|| self.most_available())
    }

    pub fn most_available(&self) -> ShirtColor {
        let mut red_len = 0;
        let mut blue_len = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_len += 1,
                ShirtColor::Blue => blue_len += 1,
            };
        }

        if blue_len >= red_len {
            return ShirtColor::Blue;
        } else {
            return ShirtColor::Red;
        }
    }
}
