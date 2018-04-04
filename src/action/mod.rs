//! Liste von zu schaltenden Ausgängen (`output`)

#[derive(Debug)]
#[derive(Default)]
pub struct Action {}

impl Action {
    /// Erzeut eine neue Action
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate xmz_server;
    /// use xmz_server::action::Action;
    ///
    /// let _action = Action::new();
    /// ```
    pub fn new() -> Self {
        Action {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _action = Action::new();
    }
}
