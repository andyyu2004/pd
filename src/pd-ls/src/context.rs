use super::{Event, Result};
use crate::dispatch::{NotificationDispatcher, RequestDispatcher};
use crate::lsp_ext;
use crate::vfs::Vfs;
use crossbeam_channel::{select, Receiver, Sender};
use lsp_server::{ErrorCode, Message, RequestId, Response};
use lsp_types::request::Request;
use pd_ide::{Analysis, AnalysisCtxt, Change, FileChange};
use serde::Serialize;

pub(crate) struct LspContext {
    sender: Sender<Message>,
    acx: AnalysisCtxt,
    vfs: Vfs,
}

impl LspContext {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender, acx: Default::default(), vfs: Default::default() }
    }

    pub fn respond<R: Serialize>(&self, request_id: RequestId, res: R) -> Result<()> {
        self.sender.send(Message::Response(Response::new_ok(request_id, res)))?;
        Ok(())
    }

    pub fn respond_err(&self, request_id: RequestId, code: ErrorCode, msg: String) -> Result<()> {
        self.sender.send(Message::Response(Response::new_err(request_id, code as i32, msg)))?;
        Ok(())
    }

    pub fn next_event(&self, inbox: &Receiver<lsp_server::Message>) -> Option<Event> {
        select! {
            recv(inbox) -> msg => msg.ok().map(Event::Lsp),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        info!("received event: {:?}", event);
        match event {
            Event::Lsp(msg) => match msg {
                lsp_server::Message::Request(req) => self.handle_request(req),
                lsp_server::Message::Response(res) => self.handle_response(res),
                lsp_server::Message::Notification(notif) => self.handle_notif(notif),
            },
        }
    }

    pub(crate) fn handle_request(&mut self, req: lsp_server::Request) -> Result<()> {
        let mut dispatcher = RequestDispatcher { lcx: self, req: Some(req) };
        dispatcher.on::<lsp_ext::SyntaxTree>(Self::syntax_tree)?;
        assert!(dispatcher.req.is_none(), "unhandled request `{:?}`", dispatcher.req.unwrap());
        Ok(())
    }

    pub(crate) fn handle_response(&mut self, res: lsp_server::Response) -> Result<()> {
        let _ = res;
        todo!()
    }

    fn handle_notif(&mut self, notif: lsp_server::Notification) -> Result<()> {
        let mut dispatcher = NotificationDispatcher { lcx: self, notif: Some(notif) };
        dispatcher
            .on::<lsp_types::notification::DidChangeTextDocument>(Self::did_change_document)?
            .on::<lsp_types::notification::DidOpenTextDocument>(Self::did_open_document)?;

        assert!(
            dispatcher.notif.is_none(),
            "unhandled notification `{:?}`",
            dispatcher.notif.unwrap()
        );

        Ok(())
    }

    fn syntax_tree(
        &mut self,
        params: lsp_ext::SyntaxTreeParams,
    ) -> Result<<lsp_ext::SyntaxTree as Request>::Result> {
        let file_id = self.vfs.intern_path(params.text_document.uri.path());
        let analysis = self.acx.analysis();
        let parsed = analysis.parse(file_id)?;
        Ok(format!("{:#?}", parsed.syntax()))
    }

    fn did_change_document(
        &mut self,
        params: lsp_types::DidChangeTextDocumentParams,
    ) -> Result<()> {
        trace!("handle_did_change_text_document");
        let file_id = self.vfs.intern_path(params.text_document.uri.path());
        assert_eq!(params.content_changes.len(), 1, "implementing full sync only for now");
        let mut new_text = String::new();
        for content_change in params.content_changes {
            match content_change.range {
                Some(_range) => todo!(),
                None => new_text = content_change.text,
            };
        }

        let change = Change::single(file_id, FileChange::Modified(new_text));
        self.acx.apply_change(change);

        Ok(())
    }

    fn did_open_document(&mut self, params: lsp_types::DidOpenTextDocumentParams) -> Result<()> {
        trace!("handle_did_open_text_document");
        let doc = params.text_document;
        let file_id = self.vfs.intern_path(doc.uri.path());
        let change = Change::single(file_id, FileChange::Created(doc.text));
        self.acx.apply_change(change);
        Ok(())
    }
}
