pub struct Schaltpunkt {}

impl Schaltpunkt {
    fn new() -> Self {
        Schaltpunkt {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _schaltpunkt = Schaltpunkt::new();
    }
}
