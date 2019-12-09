/*!
 * Contains modules relating to the app's
 * in-memory data model.
 */

mod workspace;
pub use workspace::*;

mod notebook;
pub use notebook::*;

mod note;
pub use note::*;

mod section;
pub use section::*;