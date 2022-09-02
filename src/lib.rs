use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum NumberEnum {
    Foo = 1,
    Bar = 2,
    Qux = 3,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum StringEnum {
    Foo = "foo",
    Bar = "bar",
    Qux = "qux",
}
