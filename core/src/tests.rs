pub mod todo_centre;
pub mod todo_permission_centre;

use std::fmt::Debug;

#[inline(always)]
fn assert_with_debug<A: Debug>(left: A, right: A) {
    assert_eq!(format!("{left:?}"), format!("{right:?}"))
}
