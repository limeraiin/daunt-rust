#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TargetingState {
    None,
    SelectingDashTarget,
    SelectingFireballTarget,
}

impl TargetingState {
    pub fn is_targeting(&self) -> bool {
        *self != TargetingState::None
    }
}