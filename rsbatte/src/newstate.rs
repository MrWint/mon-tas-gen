use std::mem::size_of;
use std::ptr::copy_nonoverlapping;

pub trait SyncObject {
  fn sync<S: SyncState>(&mut self, sync_state: &mut S);
}

pub trait SyncState {
  fn sync<T>(&mut self, value: &mut T);
  fn sync_object<S: SyncObject>(&mut self, value: &mut S);
  fn sync_offset(&mut self, value: &mut *mut u8, base: *mut u8);
}

pub struct LoadState<'a> {
  data: &'a [u8],
  position: usize,
}
impl<'a> LoadState<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    LoadState { data, position: 0, }
  }
}
impl<'a> SyncState for LoadState<'a> {
  fn sync<T>(&mut self, value: &mut T) {
    assert!(self.position + size_of::<T>() <= self.data.len());
    unsafe {
      copy_nonoverlapping(
          self.data.as_ptr().offset(self.position as isize),
          value as *mut T as *mut u8,
          size_of::<T>());
    }
    self.position += size_of::<T>();
  }
  fn sync_object<S: SyncObject>(&mut self, value: &mut S) { value.sync(self) }
  fn sync_offset(&mut self, value: &mut *mut u8, base: *mut u8) {
    let mut offset = 0;
    self.sync(&mut offset);
    *value = unsafe { base.offset(offset) };
  }
}

pub struct SaveState {
  data: Vec<u8>,
}
impl SaveState {
  pub fn new() -> Self {
    SaveState { data: Vec::new() }
  }
}
impl SyncState for SaveState {
  fn sync<T>(&mut self, value: &mut T) {
    self.data.reserve(size_of::<T>());
    unsafe {
      copy_nonoverlapping(
          value as *mut T as *mut u8,
          self.data.as_mut_ptr().offset(self.data.len() as isize),
          size_of::<T>());
      self.data.set_len(self.data.len() + size_of::<T>());
    }
  }
  fn sync_object<S: SyncObject>(&mut self, value: &mut S) { value.sync(self) }
  fn sync_offset(&mut self, value: &mut *mut u8, base: *mut u8) {
    let mut offset = unsafe { value.offset_from(base) };
    self.sync(&mut offset);
  }
}
pub struct StateLen {
  len: usize,
}
impl StateLen {
  pub fn new() -> Self {
    StateLen { len: 0 }
  }
}
impl SyncState for StateLen {
  fn sync<T>(&mut self, _value: &mut T) {
    self.len += size_of::<T>();
  }
  fn sync_object<S: SyncObject>(&mut self, value: &mut S) { value.sync(self) }
  fn sync_offset(&mut self, _value: &mut *mut u8, _base: *mut u8) {
    self.len += size_of::<isize>();
  }
}
