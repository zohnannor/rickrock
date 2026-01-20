//! [`Command`] pattern abstractions for domain logic execution.

mod create_user;
mod handler;

pub use self::create_user::CreateUser;

/// Marker trait for an intent to perform an action on a specific entity.
pub trait Command<Ent> {}

/// Logic for executing a [`Command`] against a specific context a specific
/// `Ent`ity.
pub trait Execute<Ent, Ctx> {
    /// The successful result of the [`Command`] execution.
    type Output;

    /// The error type returned if [`Command`] execution fails.
    type Error;

    /// Performs the [`Command`] logic using the provided context.
    fn execute(
        &self,
        _: &Ctx,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;
}

/// [`Register`]s a command in a `Service` to its specific execution
/// [`Pipeline`].
///
/// [`Pipeline`]: Register::Pipeline
pub trait Register<Cmd> {
    /// The pipeline type that [`handle`]s this command.
    ///
    /// [`handle`]: Handle::handle
    type Pipeline;
}

/// A stage in the [`Command`] execution [`Pipeline`].
///
/// [`Pipeline`]: Register::Pipeline
pub trait Handle<Cmd> {
    /// The output of this [`Pipeline`] stage.
    ///
    /// [`Pipeline`]: Register::Pipeline
    type Output;

    /// The error type produced by this [`Pipeline`] stage.
    ///
    /// [`Pipeline`]: Register::Pipeline
    type Error;

    /// Processes the [`Command`].
    fn handle(
        &self,
        cmd: Cmd,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send
    where
        Cmd: Send;
}

/// High-level interface for running domain [`Command`]s.
pub trait Executor<Ent, Cmd> {
    /// The output of the [`Command`]'s [`Pipeline`] execution.
    ///
    /// [`Pipeline`]: Register::Pipeline
    type Output;

    /// The error produced during [`Command`]'s [`Pipeline`] execution.
    ///
    /// [`Pipeline`]: Register::Pipeline
    type Error;

    /// Dispatches and runs the [`Command`].
    fn run(
        &self,
        cmd: Cmd,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;
}

/// Transparent semantic wrapper for a service to facilitate dependency
/// extraction for commands.
///
/// Used to circumvent compiler's rules of trait coherence.
pub struct ServiceWrapper<Svc>(pub Svc);
