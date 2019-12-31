/*!
 * TODO
 */
use std::rc::{Rc, Weak};

pub trait WorkspaceNodeDescribe {
    fn node_name<'a>(&self) -> &'a str;
    fn full_file_name<'a>(&self) -> &'a str;
    fn full_container_name<'a>(&self) -> &'a str;
    fn is_container(&self) -> bool;
}

pub trait WorkspaceNodeTraverse<P: WorkspaceNodeDescribe> {
    fn parent(&self) -> Weak<P>;
}

/**
 * Wrapper for model elements that allows referring to an
 * element's parent without making the element's data non-serde
 * compatible.
 */
#[derive(Debug)]
pub struct WorkspaceNode<T: WorkspaceNodeDescribe, P: WorkspaceNodeDescribe> {
    pub data: T,
    pub parent: Weak<P>,
}

impl<T, P> WorkspaceNode<T, P>
where T: WorkspaceNodeDescribe,
P: WorkspaceNodeDescribe {
    pub fn new(data: T, parent: Option<&P>) -> Self {
        let mut parent_weak: Weak<P> = Weak::<P>::new();

        if let Some(parent_ref) = parent {
            let parent_rc = Rc::new(*parent_ref);
            parent_weak = Rc::downgrade(&parent_rc);
        }

        Self {
            data: data,
            parent: parent_weak,
        }
    }

    pub fn node_name<'a>(&self) -> &'a str {
        self.data.node_name()
    }

    pub fn full_file_name<'a>(&self) -> &'a str {
        self.prefix_name_with_parent_full_name(self.node_file_name())
    }

    pub fn full_container_name<'a>(&self) -> &'a str {
        self.prefix_name_with_parent_full_name(self.node_container_name())
    }

    pub fn is_container(&self) -> bool {
        self.data.is_container()
    }

    fn prefix_name_with_parent_full_name<'a>(&self, name: &str) -> &'a str {
        let separator = get_os_separator();
        let mut parent_root = "";
        if let Some(parent_rc) = self.parent.upgrade() {
            parent_root = parent_rc.full_container_name()
        }

        let result = format!("{}{}{}", parent_root, separator, name);

        result.as_str()
    }

    fn node_file_name<'a>(&self) -> &'a str {
        //TODO: This consists of the node name, a dot,
        //then the node's file extension.
        unimplemented!()
    }

    fn node_container_name<'a>(&self) -> &'a str {
        self.node_name()
    }
}

impl<T, P> WorkspaceNodeTraverse<P> for WorkspaceNode<T, P>
where T: WorkspaceNodeDescribe,
P: WorkspaceNodeDescribe {
    fn parent(&self) -> Weak<P> {
        self.parent()
    }
}

fn get_os_separator<'a>() -> &'a str {
    unimplemented!();
}