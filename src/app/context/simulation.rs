use super::LevelData;

mod planet;
mod player;
mod target;
mod vec2f;
mod wall;

pub use planet::Planet;
pub use player::Player;
pub use target::Target;
pub use vec2f::Vec2F;
pub use wall::Wall;

// const G: f64 = 6.67430e-11;
const G: f64 = 0.55;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Crashed,
    Won,
}

#[derive(Debug, Clone)]
pub struct Simulation {
    pub player: Player,
    pub target: Target,
    pub planets: Vec<Planet>,
    pub walls: Vec<Wall>,
    pub speed: u32,
    pub playing: bool,
}

impl Simulation {
    pub const fn empty() -> Self {
        Self {
            player: Player::new(Vec2F::new(50.0, 120.0)),
            target: Target::from_nums(&[20.0, 330.0, 120.0]),
            planets: Vec::new(),
            walls: Vec::new(),
            speed: 1,
            playing: true,
        }
    }

    /// Replace the contents of the simulation with the newly passed items
    pub fn push(&mut self, level_data: &LevelData) {
        self.player = level_data.player.clone();
        self.target = level_data.target.clone();
        self.planets.clone_from(&level_data.planets);
        self.walls.clone_from(&level_data.walls);
        self.playing = true;
    }

    pub fn tick(&mut self) -> Option<Event> {
        if !self.playing {
            return None;
        }

        for _ in 0..self.speed {
            if self.gravitate_player() || self.is_colliding_with_walls() {
                return Some(Event::Crashed);
            }

            if self.is_touching_target() {
                return Some(Event::Won);
            }
        }

        None
    }

    /// Returns `true` if the player crashes
    fn gravitate_player(&mut self) -> bool {
        for planet in &self.planets {
            let distance = planet.pos - self.player.pos;
            let angle = distance.angle();

            let magnitude = distance.x.mul_add(distance.x, distance.y.powi(2));
            let acceleration = G * planet.mass / magnitude;

            self.player.velocity += Vec2F::new(angle.cos(), angle.sin()) * acceleration;

            if magnitude < planet.mass.powi(2) / 144.0 {
                return true;
            }
        }

        self.player.pos += self.player.velocity;

        false
    }

    fn is_colliding_with_walls(&self) -> bool {
        let player_line = Wall::new(self.player.pos, self.player.pos - self.player.velocity);

        for wall in &self.walls {
            if wall.intersects(&player_line) {
                return true;
            }
        }

        false
    }

    fn is_touching_target(&self) -> bool {
        let vecdistance = self.target.pos - self.player.pos;

        let distance = vecdistance
            .x
            .mul_add(vecdistance.x, vecdistance.y * vecdistance.y);

        distance < self.target.size.powi(2)
    }
}
