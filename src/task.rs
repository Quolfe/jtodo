use std::time::Duration;

#[derive(Debug, Clone)]
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

    pub fn format_data(&self) -> String {
        let mut subtask_ids_fmt = String::new();
        subtask_ids_fmt.push('[');
        for (i, id) in self.subtask_ids.iter().enumerate() {
            subtask_ids_fmt.push_str(&id.to_string());
            if i < self.subtask_ids.len() - 1 {
                subtask_ids_fmt.push(' ');
            }
        }
        subtask_ids_fmt.push(']');
        let data_line = format!("{} {} {} {} {} \"{}\" \"{}\"",
            self.priority,
            self.time_due,
            self.duration.as_secs() / 60,
            match self.parent_id { Some(n) => n.to_string(), None => "-".to_owned() },
            subtask_ids_fmt,
            self.name,
            self.description,
        );
        return data_line;
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
