use std::borrow::{Cow, ToOwned};


pub trait IntoOwned {
    type Owned: IntoOwned<Owned = Self::Owned>;
    fn into_owned(self) -> Self::Owned;
}

pub trait ContiguousContainer: IntoOwned {
    type Item: Clone;
    fn slice(&self) -> &[Self::Item];
    fn map_into_owned(self, f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned;
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned;
    fn from_slice(slice: &[Self::Item]) -> Self::Owned;
    //fn from_array<const N2: usize>(array: [Self::Item; N2]) -> Self::Owned {
    //    Self::from_slice(&array[..])
    //}
}

pub trait ContiguousContainerMut: ContiguousContainer {
    fn slice_mut(&mut self) -> &mut [Self::Item];
}

impl<T, const N: usize> IntoOwned for [T; N] {
    type Owned = [T; N];
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T, const N: usize> ContiguousContainer for [T; N]
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        self
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self;
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
}

impl<T, const N: usize> ContiguousContainerMut for [T; N]
where
    T: Clone,
{
    //#[inline(always)]
    fn slice_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> IntoOwned for &[T; N]
where
    T: Clone,
{
    type Owned = [T; N];
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self.clone()
    }
}

impl<T, const N: usize> ContiguousContainer for &[T; N]
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        &self[..]
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self.clone();
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
}

impl<T, const N: usize> IntoOwned for &mut [T; N]
where
    T: Clone,
{
    type Owned = [T; N];
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self.clone()
    }
}

impl<T, const N: usize> ContiguousContainer for &mut [T; N]
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        &self[..]
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        let mut result = self.clone();
        for i in 0..N {
            result[i] = f(result[i].clone(), i);
        }
        result
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        assert!(len == N);
        std::array::from_fn(f)
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        std::array::from_fn(|i| slice[i].clone())
    }
}

impl<T, const N: usize> ContiguousContainerMut for &mut [T; N]
where
    T: Clone,
{
    //#[inline(always)]
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T> IntoOwned for &[T]
where
    T: Clone,
{
    type Owned = Vec<T>;
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self.to_owned()
    }
}

impl<T> ContiguousContainer for &[T]
where
    T: Clone,
{
    type Item = T;
    
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        self
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.iter().cloned().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
}

impl<T> IntoOwned for &mut [T]
where
    T: Clone,
{
    type Owned = Vec<T>;
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self.to_owned()
    }
}

impl<T> ContiguousContainer for &mut [T]
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        &self[..]
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.iter().cloned().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
}

impl<T> ContiguousContainerMut for &mut [T]
where
    T: Clone,
{
    //#[inline(always)]
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T> IntoOwned for Vec<T>
where
    T: Clone,
{
    type Owned = Self;
    //#[inline(always)]
    fn into_owned(self) -> Self::Owned {
        self
    }
}

impl<T> ContiguousContainer for Vec<T>
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[T] {
        self
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Self::Owned {
        self.into_iter().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Self::Owned {
        (0..len).map(f).collect()
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Self::Owned {
        slice.to_owned()
    }
}

impl<T> ContiguousContainerMut for Vec<T>
where
    T: Clone,
{
    //#[inline(always)]
    fn slice_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<'a, T> IntoOwned for Cow<'a, [T]>
where
    T: Clone,
{
    type Owned = Vec<T>;
    //#[inline(always)]
    fn into_owned(self) -> Vec<T> {
        self.into_owned()
    }
}

impl<'a, T> ContiguousContainer for Cow<'a, [T]>
where
    T: Clone,
{
    type Item = T;
    //#[inline(always)]
    fn slice(&self) -> &[Self::Item] {
        self
    }
    //#[inline(always)]
    fn map_into_owned(self, mut f: impl FnMut(Self::Item, usize) -> Self::Item) -> Vec<T> {
        self.into_owned().into_iter().enumerate().map(|(i, x)| f(x, i)).collect()
    }
    //#[inline(always)]
    fn owned_from_fn(len: usize, f: impl Fn(usize) -> Self::Item) -> Vec<T> {
        (0..len).map(f).collect()
    }
    //#[inline(always)]
    fn from_slice(slice: &[Self::Item]) -> Vec<T> {
        slice.to_owned()
    }
}