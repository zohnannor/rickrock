//! [`Command`] execution [`Handle`]s implementation.

use std::marker::PhantomData;

use crate::service::command::{Execute, Handle, ServiceWrapper};

/// A [`Pipeline`] component that executes a [`Command`] within a specific
/// context.
///
/// [`Pipeline`]: crate::service::command::Register::Pipeline
/// [`Command`]: crate::service::command::Command
pub struct ExecuteCommand<Ent, Ctx> {
    /// [`Command`]'s entity type.
    ///
    /// [`Command`]: crate::service::command::Command
    _entity: PhantomData<Ent>,

    /// The context used for [`Command`] execution.
    ///
    /// [`Command`]: crate::service::command::Command
    ctx: Ctx,
}

impl<Svc, Ent, Ctx> From<ServiceWrapper<Svc>> for ExecuteCommand<Ent, Ctx>
where
    Ctx: From<Svc>,
{
    fn from(value: ServiceWrapper<Svc>) -> Self {
        Self { _entity: PhantomData, ctx: value.0.into() }
    }
}

impl<Ent, Cmd, Ctx> Handle<Cmd> for ExecuteCommand<Ent, Ctx>
where
    Cmd: Execute<Ent, Ctx>,
    Ctx: Sync,
    Ent: Sync,
{
    type Output = Cmd::Output;
    type Error = Cmd::Error;

    async fn handle(&self, cmd: Cmd) -> Result<Self::Output, Self::Error> {
        cmd.execute(&self.ctx).await
    }
}
