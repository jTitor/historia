/*!
 * TODO
 */

/**
 * TODO
 */
pub trait DescribeFsPath {
    fn element_name<'a>(&self) -> &'a str;
    fn is_container(&self) -> bool;
}
