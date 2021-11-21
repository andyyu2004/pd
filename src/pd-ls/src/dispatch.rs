use crate::{LspContext, Result};
use serde::de::DeserializeOwned;

pub(crate) struct NotificationDispatcher<'lcx> {
    pub(crate) notif: Option<lsp_server::Notification>,
    pub(crate) lcx: &'lcx LspContext,
}

// sum two numbers
fn sum() {
}

impl<'lcx> NotificationDispatcher<'lcx> {
    pub fn on<N>(&mut self, f: fn(&LspContext, N::Params) -> Result<()>) -> Result<&mut Self>
    where
        N: lsp_types::notification::Notification + 'static,
        N::Params: DeserializeOwned + Send + 'static,
    {
        let notif = match self.notif.take() {
            Some(notif) => notif,
            None => return Ok(self),
        };

        let params = match notif.extract::<N::Params>(N::METHOD) {
            Ok(params) => params,
            Err(notif) => {
                self.notif = Some(notif);
                return Ok(self);
            }
        };

        f(self.lcx, params)?;
        Ok(self)
    }
}
