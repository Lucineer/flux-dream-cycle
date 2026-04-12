pub type TaskState = i32;
pub const PENDING: TaskState = 0;
pub const RUNNING: TaskState = 1;
pub const COMPLETED: TaskState = 2;
pub const FAILED: TaskState = 3;
pub const CANCELLED: TaskState = 4;
pub const DEFERRED: TaskState = 5;

pub type TaskPriority = i32;
pub const CRITICAL: TaskPriority = 0;
pub const HIGH: TaskPriority = 1;
pub const NORMAL: TaskPriority = 2;
pub const LOW: TaskPriority = 3;
pub const BACKGROUND: TaskPriority = 4;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub created: u64,
    pub started: u64,
    pub deadline: u64,
    pub retries_left: u32,
    pub recurring: bool,
    pub recurr_interval: u64,
    pub last_completed: u64,
    pub run_count: u32,
}
