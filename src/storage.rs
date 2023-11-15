use std::borrow::{Cow, ToOwned};

/// A trait for types that can be converted to owned types.
///
/// This trait is similar to [`ToOwned`], but adds the [`into_owned`] method.
pub trait IntoOwned {
    /// The owned type.
    type Owned: Owned;
    /// Converts `self` into an owned value.
    fn into_owned(self) -> Self::Owned;
}

/// A trait for types that are already owned.
///
/// This trait requires the owned type to be the same as the type itself.
pub trait Owned: IntoOwned
where
    Self: IntoOwned<Owned = Self>,
{
}

impl<T> Owned for T where T: IntoOwned<Owned = T> {}

/// Storage for differentials
///
/// This trait implies that elements are stored in a contiguous array.
///
/// This trait only provides methods for immutable access to the elements,
/// [`MutStorage`] provides methods for mutable access.
pub trait ConstStorage: IntoOwned
{
    type Item: Clone;
    fn slice(&self) -> &[Self::Item];
    fn map_into_owned(self, f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned;
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned;
    fn from_slice(slice: &[Self::Item]) -> Self::Owned;
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned;
}

pub trait MutStorage: ConstStorage {
    fn slice_mut(&mut self) -> &mut [Self::Item];
}

pub trait OwnedStorage: MutStorage
where
    Self: IntoOwned<Owned = Self>,
{
}

impl<S: MutStorage> OwnedStorage for S
where
    S: IntoOwned<Owned = S>,
{
}

impl<T, const N: usize> IntoOwned for [T; N] {
    type Owned = [T; N];
    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T, const N: usize> ConstStorage for [T; N]
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[T] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self;
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        let mut it = iter.into_iter();
        std::array::from_fn(|_| it.next().expect("not enough elements in iterator"))
    }
}

impl<T, const N: usize> MutStorage for [T; N]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> IntoOwned for &[T; N]
where
    T: Clone,
{
    type Owned = [T; N];
    fn into_owned(self) -> Self::Owned {
        self.clone()
    }
}

impl<T, const N: usize> ConstStorage for &[T; N]
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[T] {
        &self[..]
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self.clone();
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        <[T; N] as ConstStorage>::from_iter(iter)
    }
}

impl<T, const N: usize> IntoOwned for &mut [T; N]
where
    T: Clone,
{
    type Owned = [T; N];
    fn into_owned(self) -> Self::Owned {
        self.clone()
    }
}

impl<T, const N: usize> ConstStorage for &mut [T; N]
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[T] {
        &self[..]
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self.clone();
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        <[T; N] as ConstStorage>::from_iter(iter)
    }
}

impl<T, const N: usize> MutStorage for &mut [T; N]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T> IntoOwned for &[T]
where
    T: Clone,
{
    type Owned = Vec<T>;
    fn into_owned(self) -> Self::Owned {
        self.to_owned()
    }
}

impl<T> ConstStorage for &[T]
where
    T: Clone,
{
    type Item = T;
    
    fn slice(&self) -> &[T] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.iter().cloned().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        iter.into_iter().collect()
    }
}

impl<T> IntoOwned for &mut [T]
where
    T: Clone,
{
    type Owned = Vec<T>;
    fn into_owned(self) -> Self::Owned {
        self.to_owned()
    }
}

impl<T> ConstStorage for &mut [T]
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[T] {
        &self[..]
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.iter().cloned().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        iter.into_iter().collect()
    }
}

impl<T> MutStorage for &mut [T]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T> IntoOwned for Vec<T>
where
    T: Clone,
{
    type Owned = Self;
    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T> ConstStorage for Vec<T>
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[T] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.into_iter().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned {
        iter.into_iter().collect()
    }
}

impl<T> MutStorage for Vec<T>
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<'a, T> IntoOwned for Cow<'a, [T]>
where
    T: Clone,
{
    type Owned = Vec<T>;
    fn into_owned(self) -> Vec<T> {
        self.into_owned()
    }
}

impl<'a, T> ConstStorage for Cow<'a, [T]>
where
    T: Clone,
{
    type Item = T;
    fn slice(&self) -> &[Self::Item] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Vec<T> {
        self.into_owned().into_iter().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Vec<T> {
        (0..len).map(f).collect()
    }
    fn from_slice(slice: &[Self::Item]) -> Vec<T> {
        slice.to_owned()
    }
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Vec<T> {
        iter.into_iter().collect()
    }
}