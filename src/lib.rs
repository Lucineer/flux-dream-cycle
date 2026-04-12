#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum TaskState { Idle, Running, Done, Failed }
pub struct DreamTask { id: u32, name: String, priority: u8, state: TaskState, repeats: bool, interval: u64, last_run: u64, run_count: u32 }
pub struct DreamCycle { tasks: Vec<DreamTask>, next_id: u32, current_tick: u64 }
impl DreamCycle {
    pub fn new() -> Self { Self { tasks: Vec::new(), next_id: 1, current_tick: 0 } }
    pub fn schedule(&mut self, name: &str, priority: u8, interval: u64) -> u32 {
        let id = self.next_id; self.next_id += 1;
        self.tasks.push(DreamTask { id, name: name.to_string(), priority, state: TaskState::Idle, repeats: true, interval, last_run: 0, run_count: 0 }); id
    }
    pub fn schedule_once(&mut self, name: &str, priority: u8) -> u32 {
        let id = self.next_id; self.next_id += 1;
        self.tasks.push(DreamTask { id, name: name.to_string(), priority, state: TaskState::Idle, repeats: false, interval: 0, last_run: 0, run_count: 0 }); id
    }
    pub fn tick(&mut self) -> Vec<u32> { self.current_tick += 1; self.ready_tasks().iter().map(|t| t.id).collect() }
    pub fn start(&mut self, id: u32) -> bool { if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id && t.state == TaskState::Idle) { t.state = TaskState::Running; true } else { false } }
    pub fn complete(&mut self, id: u32) -> bool { if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id && t.state == TaskState::Running) {
        t.state = if t.repeats { TaskState::Idle } else { TaskState::Done }; t.run_count += 1; t.last_run = self.current_tick; true } else { false } }
    pub fn fail(&mut self, id: u32) -> bool { if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id && t.state == TaskState::Running) { t.state = TaskState::Idle; true } else { false } }
    pub fn cancel(&mut self, id: u32) -> bool { if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) { t.state = TaskState::Done; true } else { false } }
    pub fn ready_tasks(&self) -> Vec<&DreamTask> { self.tasks.iter().filter(|t| t.state == TaskState::Idle && (t.interval == 0 || self.current_tick - t.last_run >= t.interval)).collect() }
    pub fn next_priority(&self) -> Option<&DreamTask> { self.ready_tasks().iter().max_by_key(|t| t.priority).copied() }
    pub fn find(&self, id: u32) -> Option<&DreamTask> { self.tasks.iter().find(|t| t.id == id) }
    pub fn remove_done(&mut self) -> Vec<DreamTask> { let d: Vec<DreamTask> = self.tasks.iter().filter(|t| t.state == TaskState::Done).cloned().collect(); self.tasks.retain(|t| t.state != TaskState::Done); d }
    pub fn active_count(&self) -> usize { self.tasks.iter().filter(|t| t.state == TaskState::Idle || t.state == TaskState::Running).count() }
    pub fn stats(&self) -> (usize, usize, usize) {
        let (i, r, d) = (TaskState::Idle, TaskState::Running, TaskState::Done);
        (self.tasks.iter().filter(|t| t.state == i).count(), self.tasks.iter().filter(|t| t.state == r).count(), self.tasks.iter().filter(|t| t.state == d).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let d = DreamCycle::new(); assert_eq!(d.active_count(), 0); }
    #[test] fn test_schedule() { let mut d = DreamCycle::new(); let id = d.schedule("a", 5, 10); assert!(id > 0); assert_eq!(d.active_count(), 1); }
    #[test] fn test_schedule_once() { let mut d = DreamCycle::new(); d.schedule_once("a", 5); let t = d.ready_tasks(); assert_eq!(t.len(), 1); }
    #[test] fn test_start_complete() { let mut d = DreamCycle::new(); let id = d.schedule("a", 5, 10); assert!(d.start(id)); assert!(d.complete(id)); assert_eq!(d.find(id).unwrap().run_count, 1); }
    #[test] fn test_interval() { let mut d = DreamCycle::new(); let id = d.schedule("a", 5, 5); d.complete(id); for _ in 0..4 { d.tick(); } assert_eq!(d.tick().len(), 0); assert_eq!(d.tick().len(), 1); }
    #[test] fn test_next_priority() { let mut d = DreamCycle::new(); d.schedule("low", 1, 0); d.schedule("high", 10, 0); assert_eq!(d.next_priority().unwrap().name, "high"); }
    #[test] fn test_cancel() { let mut d = DreamCycle::new(); let id = d.schedule("a", 5, 10); assert!(d.cancel(id)); assert_eq!(d.active_count(), 0); }
    #[test] fn test_fail() { let mut d = DreamCycle::new(); let id = d.schedule("a", 5, 10); d.start(id); assert!(d.fail(id)); assert_eq!(d.find(id).unwrap().state, TaskState::Idle); }
    #[test] fn test_remove_done() { let mut d = DreamCycle::new(); let id = d.schedule_once("a", 5); d.start(id); d.complete(id); let r = d.remove_done(); assert_eq!(r.len(), 1); }
    #[test] fn test_stats() { let mut d = DreamCycle::new(); d.schedule("a", 5, 0); d.schedule("b", 5, 0); let id = d.schedule_once("c", 5); d.start(id); d.complete(id); let (i, r, done) = d.stats(); assert_eq!(done, 1); }
}