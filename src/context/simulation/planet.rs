use super::Vec2F;

#[derive(Debug, Clone)]
pub struct Planet {
    pub mass: f64,
    pub pos: Vec2F,
}

impl Planet {
    pub const fn new(mass: f64, pos: Vec2F) -> Self {
        Self { mass, pos }
    }

    pub const fn from_nums(nums: &[f64]) -> Self {
        assert!(
            nums.len() == 3,
            "Wrong number of items passed to `Player::from_nums`"
        );

        Self::new(nums[0], Vec2F::new(nums[1], nums[2]))
    }
}
