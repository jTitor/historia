/*!
 * TODO
 */
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

pub type WorkspaceNode<T> = Rc<RefCell<T>>;
pub type WeakWorkspaceNode<T> = Weak<RefCell<T>>;

pub fn new_workspace_node<T>(data: T) -> WorkspaceNode<T> {
    Rc::new(RefCell::new(data))
}