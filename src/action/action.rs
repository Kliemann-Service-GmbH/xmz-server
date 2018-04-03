pub struct Action {}

impl Action {
    fn new() -> Self {
        Action {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let action = Action::new();
    }
}
