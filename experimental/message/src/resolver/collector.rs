use crate::{parser::slice::Slice, types::MessagePart};
use std::borrow::Cow;

// MPV - message part value type
pub trait MessagePartCollector<'s> {
    fn push_part(&mut self, part: &dyn MessagePart<'s>);
}

pub struct MessagePartsList<'s>(pub Vec<Box<dyn MessagePart<'s> + 's>>);
pub struct MessageString<'s>(pub Cow<'s, str>);
pub struct MessageSink<W>(W);

impl<'s> MessagePartsList<'s> {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl<'s> MessageString<'s> {
    pub fn new() -> Self {
        Self("".into())
    }
}

impl<W> MessageSink<W> {
    pub fn new(sink: W) -> Self {
        Self(sink)
    }
}

impl<'s> MessagePartCollector<'s> for MessagePartsList<'s> {
    fn push_part(&mut self, part: &dyn MessagePart<'s>) {
        self.0.push(part.to_part());
    }
}

impl<'s> MessagePartCollector<'s> for MessageString<'s> {
    fn push_part(&mut self, part: &dyn MessagePart<'s>) {
        let new_part = part.to_cow();

        if !new_part.is_empty() {
            if self.0.is_empty() {
                self.0 = new_part;
            } else {
                self.0.to_mut().push_str(&new_part);
            }
        }
    }
}

impl<'s, W: std::fmt::Write> MessagePartCollector<'s> for MessageSink<W> {
    fn push_part(&mut self, part: &dyn MessagePart<'s>) {
        let cow = part.to_cow();
        self.0.write_str(&cow).unwrap();
    }
}
