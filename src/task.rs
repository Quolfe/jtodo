pub struct Task {
    id: usize,
    name: String,
    description: String,
}

impl Task {
    pub fn new(id: usize, name: String, description: String) -> Self {
        Task {
            id,
            name,
            description,
        }
    }

    pub fn id(&self) -> usize { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
}
