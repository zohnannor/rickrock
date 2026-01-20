//! Infrastructure of social platform.

pub mod storage;

/// [`Repository`] of a [`Service`].
///
/// [`Service`]: crate::service::Service
pub trait Repository<T> {
    /// Error produced upon [`Repository`] failure.
    type Error;

    /// Inserts a new `entity` into the [`Repository`].
    fn insert(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

/// [`Transaction`] of a [`Repository`].
pub trait Transaction {
    /// [`Transacted`] [`Repository`] of this [`Transaction`].
    ///
    /// [`Transacted`]: Transaction::Transacted
    type Transacted;
    /// Error produced upon [`Transaction`] failure.
    type Error;

    /// Starts a new [`Transaction`] or acquires the already started one.
    fn tx(
        &self,
    ) -> impl Future<Output = Result<Self::Transacted, Self::Error>> + Send;
}
