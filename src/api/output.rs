use prelude::*;


#[derive(Clone, Debug, Serialize)]
pub struct Output {
    name: String,
    output_type: OutputType,
    pins: usize,
}


/// Konvertierung von den Output Trait Objekten `::output::Output` in das `::api::Output` Objekt
///
impl From<Arc<RwLock<BoxedOutput>>> for Output {
    fn from(output: Arc<RwLock<BoxedOutput>>) -> Self {
        let output = output.read().unwrap();
        Output {
            name: output.get_name(),
            output_type: output.get_output_type(),
            pins: output.get_pins(),
        }
    }
}
