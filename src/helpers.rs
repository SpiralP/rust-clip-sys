use std::mem;

/// Hack to allow us to "take" from a mutable reference into a moved value
/// This works since null pointer objects are useless :JOY:
pub fn take_zeroed<T>(value: &mut T) -> T {
  let mut swap_me: T = unsafe { mem::zeroed() };
  mem::swap(&mut swap_me, value);
  swap_me
}
