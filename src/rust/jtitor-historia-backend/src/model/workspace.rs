/*!
 * Defines the Workspace type.
 */
use crate::model::Notebook;

/**
 * Contains one or more Notebook instances.
 */
pub struct Workspace {
    notebooks: Vec<Notebook>,
    metadata: WorkspaceMetadata,
}

pub struct WorkspaceMetadata {
    name: String,
}