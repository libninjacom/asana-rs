impl TaskRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            task_base: TaskBase {
                name: Some(name.into()),
                ..TaskBase::default()
            },
            ..Self::default()
        }
    }
}