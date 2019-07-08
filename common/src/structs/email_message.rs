use lettre_email::{mime::Mime, Mailbox};
use std::path::Path;

#[derive(Debug)]
pub struct AttachFile<'a> {
    pub path: &'a Path,
    pub filename: Option<&'a str>,
    pub content_type: &'a Mime,
}

#[derive(Debug)]
pub struct EmailMessage<'a, T: Into<Mailbox>, K: Into<Mailbox>, H: Into<String>, J: Into<String>> {
    pub tos: Vec<T>,
    pub from: K,
    pub subject: J,
    pub content: H,
    pub attachment_file: Option<AttachFile<'a>>,
}
