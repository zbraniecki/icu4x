use super::parser::slice::Slice;
use std::borrow::Cow;

pub trait VariableType<'s> {
    fn to_part<'p>(&self) -> Box<dyn MessagePart<'p> + 'p>
    where
        's: 'p;

    fn as_cow(&self) -> Cow<'s, str>;
}

impl<'s> VariableType<'s> for String {
    fn to_part<'p>(&self) -> Box<dyn MessagePart<'p> + 'p>
    where
        's: 'p,
    {
        Box::new(self.clone())
    }

    fn as_cow(&self) -> Cow<'s, str> {
        Cow::Owned(self.clone())
    }
}

impl<'s> VariableType<'s> for Cow<'s, str> {
    fn to_part<'p>(&self) -> Box<dyn MessagePart<'p> + 'p>
    where
        's: 'p,
    {
        Box::new(self.clone())
    }

    fn as_cow(&self) -> Cow<'s, str> {
        self.clone()
    }
}

pub trait MessagePart<'s>: std::fmt::Debug + std::fmt::Display + PartialEq<str> {
    fn to_cow(&self) -> Cow<'s, str>;
    fn to_part(&self) -> Box<dyn MessagePart<'s> + 's> {
        todo!();
    }
}

impl<'s> MessagePart<'s> for String {
    fn to_cow(&self) -> Cow<'s, str> {
        Cow::Owned(self.clone())
    }

    fn to_part(&self) -> Box<dyn MessagePart<'s> + 's> {
        Box::new(self.clone())
    }
}

impl<'s> MessagePart<'s> for Cow<'s, str> {
    fn to_cow(&self) -> Cow<'s, str> {
        self.clone()
    }

    fn to_part(&self) -> Box<dyn MessagePart<'s> + 's> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MarkupElement<S> {
    pub name: S,
}

impl<S: std::fmt::Debug> std::fmt::Debug for MarkupElement<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarkupElement")
            .field("name", &self.name)
            .finish()
    }
}

impl<S> std::fmt::Display for MarkupElement<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{{+{self.name}}}")
    }
}

impl<'s, S> PartialEq<str> for MarkupElement<S>
where
    S: Slice<'s>,
{
    fn eq(&self, other: &str) -> bool {
        self.name.as_str() == other
    }
}

impl<'s, S> MessagePart<'s> for MarkupElement<S>
where
    S: Slice<'s> + 's,
{
    fn to_cow(&self) -> Cow<'s, str> {
        self.name.as_cow()
    }

    fn to_part(&self) -> Box<dyn MessagePart<'s> + 's> {
        Box::new(MarkupElement {
            name: self.name.clone(),
        })
    }
}
