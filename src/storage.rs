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
    fn is_owned(&self) -> bool;
    fn slice(&self) -> &[Self::Item];
    fn map_into_owned(self, f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned;
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned;
    fn from_slice(slice: &[Self::Item]) -> Self::Owned;
    fn from_iter(iter: impl IntoIterator<Item = Self::Item>) -> Self::Owned;
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned;
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item>;
}

pub trait MutStorage: ConstStorage {
    fn slice_mut(&mut self) -> &mut [Self::Item];
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>);
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

impl<T: Clone, const N: usize> IntoOwned for [T; N] {
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
    fn is_owned(&self) -> bool {
        true
    }
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned {
        std::array::from_fn(|i| if i == 0 { order_0.clone() } else { zeros() })
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self)
    }
}

impl<T, const N: usize> MutStorage for [T; N]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        self
    }
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>) {
        (&mut self[..]).assign_iter(offset, iter)
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
    fn is_owned(&self) -> bool {
        false
    }
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned {
        <[T; N] as ConstStorage>::from_order_0(order_0, zeros)
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self).cloned()
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
    fn is_owned(&self) -> bool {
        false
    }
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned {
        <[T; N] as ConstStorage>::from_order_0(order_0, zeros)
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self).map(|x| x.clone())
    }
}

impl<T, const N: usize> MutStorage for &mut [T; N]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>) {
        (&mut self[..]).assign_iter(offset, iter)
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
    fn is_owned(&self) -> bool {
        false
    }
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
    fn from_order_0(order_0: Self::Item, _zeros: impl Fn() -> Self::Item) -> Self::Owned {
        vec![order_0]
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self).cloned()
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
    fn is_owned(&self) -> bool {
        false
    }
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned {
        <&[T] as ConstStorage>::from_order_0(order_0, zeros)
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self).map(|x| x.clone())
    }
}

impl<T> MutStorage for &mut [T]
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>) {
        let mut it = iter.into_iter();
        for i in offset..self.len() {
            self[i] = it.next().expect("not enough elements in iterator");
        }
        assert!(it.next().is_none(), "too many elements in iterator");
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
    fn is_owned(&self) -> bool {
        true
    }
    fn slice(&self) -> &[T] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        IntoIterator::into_iter(self).enumerate().map(|(i, x)| f(x, i)).collect()
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Self::Owned {
        <&[T] as ConstStorage>::from_order_0(order_0, zeros)
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self)
    }
}

impl<T> MutStorage for Vec<T>
where
    T: Clone,
{
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
    fn assign_iter(&mut self, offset: usize, iter: impl IntoIterator<Item = Self::Item>) {
        assert!(offset <= self.len());
        self.shrink_to(offset);
        self.extend(iter);
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
    fn is_owned(&self) -> bool {
        if let Cow::Borrowed(_) = self {
            false
        } else {
            true
        }
    }
    fn slice(&self) -> &[Self::Item] {
        self
    }
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Vec<T> {
        IntoIterator::into_iter(self.into_owned()).enumerate().map(|(i, x)| f(x, i)).collect()
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
    fn from_order_0(order_0: Self::Item, zeros: impl Fn() -> Self::Item) -> Vec<T> {
        <&[T] as ConstStorage>::from_order_0(order_0, zeros)
    }
    fn make_into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self.into_owned()).into_iter()
    }
}