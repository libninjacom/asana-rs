impl ProjectRequest {
    pub fn new(name: impl Into<String>) -> Self {
        use crate::model::ProjectCompact;
        Self {
            project_base: ProjectBase {
                project_compact: ProjectCompact {
                    name: Some(name.into()),
                    ..ProjectCompact::default()
                },
                ..ProjectBase::default()
            },
            ..Self::default()
        }
    }
}