
/// Inner data of a [`Differential`](crate::Differential)
///
/// This transparent structure holds the common internal structure for the following:
/// + [`Differential`](crate::Differential)
/// + [`Derivatives`](crate::Derivatives)
#[derive(Debug, Clone, Copy, Hash)]
pub struct DiffInner<Order, N, Data> {
    pub order: Order,
    pub n: N,
    pub data: Data,
}