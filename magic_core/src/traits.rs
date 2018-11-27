use std::borrow::Cow;

pub trait Named {
    /// The name of this object, if it has a single name.
    fn name(&self) -> Option<Cow<str>>;

    /// The names of this object. Default implementation can be used
    /// if the object has a single name.
    fn names(&self) -> Vec<Cow<str>> {
        let name = self.name();
        if let Some(name) = name {
            vec![name]
        } else {
            unimplemented!()
        }
    }

    /// Tests if the names of this object match the provided name.
    fn match_name(&self, name: &str) -> bool {
        self.names().iter().any(|n| n == name)
    }
}
