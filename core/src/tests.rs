pub mod todo_centre;

use std::fmt::Debug;

#[inline(always)]
fn assert_with_debug<L: Debug, R: Debug>(left: L, right: R) {
    assert_eq!(format!("{left:?}"), format!("{right:?}"))
}
