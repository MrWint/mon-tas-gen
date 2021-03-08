use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, Error};

pub trait BigArray<'de>: Sized {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

impl<'de, T: Sized + Serialize + Deserialize<'de>, const N: usize> BigArray<'de> for [T; N] {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      let mut seq = serializer.serialize_tuple(self.len())?;
      for elem in &self[..] {
          seq.serialize_element(elem)?;
      }
      seq.end()
  }

  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<[T; N], D::Error> {

    let visitor = ArrayVisitor { element: PhantomData };
    deserializer.deserialize_tuple(N, visitor)
  }
}

struct ArrayVisitor<T, const N: usize> {
  element: PhantomData<T>,
}

impl<'de, T: Sized + Deserialize<'de>, const N: usize> Visitor<'de> for ArrayVisitor<T, N> {
  type Value = [T; N];

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(&format!("an array of length {}", N))
  }

  fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<[T; N], A::Error> {
    let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
    for i in 0..N {
      data[i] = MaybeUninit::new(seq.next_element()?.ok_or_else(|| Error::invalid_length(i, &self))?);
    }
    Ok(unsafe { std::mem::transmute_copy::<_, [T; N]>(&data) })
  }
}
