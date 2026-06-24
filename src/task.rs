use std::time::Duration;

pub struct Task {
    id: usize,
    name: String,
    description: String,
    priority: usize,
    time_due: usize,
    duration: Duration,
    parent_id: Option<usize>,
    subtask_ids: Vec<usize>,
}

impl Task {
    pub fn new(id: usize, name: String, description: String, priority: usize, time_due: usize, duration: Duration, parent_id: Option<usize>, subtask_ids: Vec<usize>) -> Self {
        Task { id, name, description, priority, time_due, duration, parent_id, subtask_ids, }
    }

    pub fn id(&self) -> usize { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
    pub fn priority(&self) -> usize { self.priority }
    pub fn time_due(&self) -> usize { self.time_due }
    pub fn duration(&self) -> Duration { self.duration }
    pub fn parent_id(&self) -> Option<usize> { self.parent_id }
    pub fn subtask_ids(&self) -> &[usize] { &self.subtask_ids }
}
