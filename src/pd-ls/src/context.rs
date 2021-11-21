use crate::dispatch::NotificationDispatcher;

use super::{Event, Result};
use crossbeam_channel::{select, Receiver};

pub(crate) struct LspContext {}

impl LspContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next_event(&self, inbox: &Receiver<lsp_server::Message>) -> Option<Event> {
        select! {
            recv(inbox) -> msg => msg.ok().map(Event::Lsp),

        }
    }

    pub fn handle_event(&self, event: Event) -> Result<()> {
        match event {
            Event::Lsp(msg) => match msg {
                lsp_server::Message::Request(req) => self.handle_request(req),
                lsp_server::Message::Response(res) => self.handle_response(res),
                lsp_server::Message::Notification(notif) => self.handle_notif(notif),
            },
        }
    }

    pub(crate) fn handle_request(&self, req: lsp_server::Request) -> Result<()> {
        let _ = req;
        todo!()
    }

    pub(crate) fn handle_response(&self, res: lsp_server::Response) -> Result<()> {
        let _ = res;
        todo!()
    }

    fn handle_notif(&self, notif: lsp_server::Notification) -> Result<()> {
        NotificationDispatcher { lcx: self, notif: Some(notif) }
            .on::<lsp_types::notification::DidChangeTextDocument>(|this, params| {
            this.handle_did_change_text_document(params)
        })?;
        Ok(())
    }

    pub(crate) fn handle_did_change_text_document(
        &self,
        params: lsp_types::DidChangeTextDocumentParams,
    ) -> Result<(), anyhow::Error> {
        trace!("handle_did_change_text_document");
        Ok(())
    }
}
