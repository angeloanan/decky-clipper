use usdpl_back::core::serdes::Primitive;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn version(_: Vec<Primitive>) -> Vec<Primitive> {
    vec![VERSION.into()]
}

pub fn name(_: Vec<Primitive>) -> Vec<Primitive> {
    vec![NAME.into()]
}

pub fn ping(params: Vec<Primitive>) -> Vec<Primitive> {
    params
}

pub fn hello(params: Vec<Primitive>) -> Vec<Primitive> {
    if let Some(Primitive::String(name)) = params.get(0) {
        vec![Primitive::String(format!("Hello {}", name))]
    } else {
        vec![]
    }
}
