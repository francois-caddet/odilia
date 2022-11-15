//! # DBus interface proxy for: `org.a11y.atspi.Component`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Component.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use serde::{Deserialize, Serialize};
use zbus::{dbus_proxy, zvariant::Type};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum ScrollType {
    TopLeft,
    BottomRight,
    TopEdge,
    BottomEdge,
    LeftEdge,
    RightEdge,
    Anywhere,
}

use crate::CoordType;

#[dbus_proxy(interface = "org.a11y.atspi.Component")]
trait Component {
    /// Contains method
    fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

    /// GetAccessibleAtPoint method
    fn get_accessible_at_point(
        &self,
        x: i32,
        y: i32,
        coord_type: CoordType,
    ) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// GetAlpha method
    fn get_alpha(&self) -> zbus::Result<f64>;

    /// GetExtents method
    fn get_extents(&self, coord_type: CoordType) -> zbus::Result<(i32, i32, i32, i32)>;

    /// GetLayer method
    fn get_layer(&self) -> zbus::Result<u32>;

    /// GetMDIZOrder method
    fn get_mdizorder(&self) -> zbus::Result<i16>;

    /// GetPosition method
    fn get_position(&self, coord_type: CoordType) -> zbus::Result<(i32, i32)>;

    /// GetSize method
    fn get_size(&self) -> zbus::Result<(i32, i32)>;

    /// GrabFocus method
    fn grab_focus(&self) -> zbus::Result<bool>;

    /// ScrollTo method
    fn scroll_to(&self, type_: ScrollType) -> zbus::Result<bool>;

    /// ScrollToPoint method
    fn scroll_to_point(&self, type_: ScrollType, x: i32, y: i32) -> zbus::Result<bool>;

    /// SetExtents method
    fn set_extents(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        coord_type: CoordType,
    ) -> zbus::Result<bool>;

    /// SetPosition method
    fn set_position(&self, x: i32, y: i32, coord_type: CoordType) -> zbus::Result<bool>;

    /// SetSize method
    fn set_size(&self, width: i32, height: i32) -> zbus::Result<bool>;
}
