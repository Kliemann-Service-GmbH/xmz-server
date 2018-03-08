

pub struct Schwellwert {

}

impl Schwellwert {
    fn new() -> Self {
        Schwellwert {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _schwellwert = Schwellwert::new();
    }
}
