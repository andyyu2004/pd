use crate::{LspContext, Result};
use serde::de::DeserializeOwned;

pub(crate) struct RequestDispatcher<'lcx> {
    pub(crate) req: Option<lsp_server::Request>,
    pub(crate) lcx: &'lcx mut LspContext,
}

impl<'lcx> RequestDispatcher<'lcx> {
    pub fn on<R>(
        &mut self,
        f: fn(&mut LspContext, R::Params) -> Result<R::Result>,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + Send + 'static,
    {
        let req = match self.req.take() {
            Some(req) if req.method == R::METHOD => req,
            req => {
                self.req = req;
                return Ok(self);
            }
        };

        let (request_id, params) =
            req.extract::<R::Params>(R::METHOD).expect("invalid request parameters");

        let res = f(self.lcx, params)?;
        self.lcx.respond(request_id, res);
        Ok(self)
    }
}

pub(crate) struct NotificationDispatcher<'lcx> {
    pub(crate) notif: Option<lsp_server::Notification>,
    pub(crate) lcx: &'lcx mut LspContext,
}

impl<'lcx> NotificationDispatcher<'lcx> {
    pub fn on<N>(&mut self, f: fn(&mut LspContext, N::Params) -> Result<()>) -> Result<&mut Self>
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
