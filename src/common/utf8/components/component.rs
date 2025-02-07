use std::fmt;

use crate::private;

/// Interface representing a component in a [`Utf8Path`]
///
/// [`Utf8Path`]: crate::Utf8Path
pub trait Utf8Component<'a>:
    AsRef<str> + Clone + fmt::Debug + PartialEq + Eq + PartialOrd + Ord + private::Sealed
{
    /// Extracts the underlying [`str`] slice
    fn as_str(&self) -> &'a str;

    /// Returns true if this component is the root component, meaning
    /// there are no more components before this one
    ///
    /// Use cases are for the root dir separator on Windows and Unix as
    /// well as Windows [`std::path::PrefixComponent`]
    ///
    /// # Examples
    ///
    /// `/my/../path/./here.txt` has the components on Unix of
    ///
    /// * `UnixComponent::RootDir` - `is_root() == true`
    /// * `UnixComponent::ParentDir` - `is_root() == false`
    /// * `UnixComponent::CurDir` - `is_root() == false`
    /// * `UnixComponent::Normal("here.txt")` - `is_root() == false`
    fn is_root(&self) -> bool;

    /// Returns true if this component represents a normal part of the path
    ///
    /// # Examples
    ///
    /// `/my/../path/./here.txt` has the components on Unix of
    ///
    /// * `UnixComponent::RootDir` - `is_normal() == false`
    /// * `UnixComponent::ParentDir` - `is_normal() == false`
    /// * `UnixComponent::CurDir` - `is_normal() == false`
    /// * `UnixComponent::Normal("here.txt")` - `is_normal() == true`
    fn is_normal(&self) -> bool;

    /// Returns true if this component represents a relative representation of a parent directory
    ///
    /// # Examples
    ///
    /// `/my/../path/./here.txt` has the components on Unix of
    ///
    /// * `UnixComponent::RootDir` - `is_parent() == false`
    /// * `UnixComponent::ParentDir` - `is_parent() == true`
    /// * `UnixComponent::CurDir` - `is_parent() == false`
    /// * `UnixComponent::Normal("here.txt")` - `is_parent() == false`
    fn is_parent(&self) -> bool;

    /// Returns true if this component represents a relative representation of the current
    /// directory
    ///
    /// # Examples
    ///
    /// `/my/../path/./here.txt` has the components on Unix of
    ///
    /// * `UnixComponent::RootDir` - `is_current() == false`
    /// * `UnixComponent::ParentDir` - `is_current() == false`
    /// * `UnixComponent::CurDir` - `is_current() == true`
    /// * `UnixComponent::Normal("here.txt")` - `is_current() == false`
    fn is_current(&self) -> bool;

    /// Returns size of component in bytes
    fn len(&self) -> usize;

    /// Returns true if component represents an empty str
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a root [`Utf8Component`].
    fn root() -> Self;

    /// Returns a parent directory [`Utf8Component`].
    fn parent() -> Self;

    /// Returns a current directory [`Utf8Component`].
    fn current() -> Self;
}
