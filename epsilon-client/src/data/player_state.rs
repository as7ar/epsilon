#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    Standing,
    Running,
    Jumping,
    Dash,
    Attacking,
    Hit,
}