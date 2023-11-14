use super::*;

impl<Order: Dim, N: Dim, Data> PartialEq for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data_slice()[0] == other.data_slice()[0]
    }
}

impl<Order: Dim, N: Dim, Data> PartialOrd for Diff<Order, N, Data>
where
    Data: ContiguousContainer,
    Data::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data_slice()[0].partial_cmp(&other.data_slice()[0])
    }
}