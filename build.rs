use paperclip::v2::{
    self,
    codegen::{DefaultEmitter, Emitter, EmitterState},
    models::{DefaultSchema, ResolvableApi},
};

use std::fs::File;

fn main() {
    // let fd = File::open("swagger.yaml").expect("schema?");
    // let raw: ResolvableApi<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
    // let schema = raw.resolve().expect("resolution");

    // let mut state = EmitterState::default();
    // // set prefix for using generated code inside `codegen` module (see main.rs).x
    // state.mod_prefix = "crate::codegen::";
    // state.working_dir = "src/codegen".into();

    // let emitter = DefaultEmitter::from(state);
    // emitter.generate(&schema).expect("codegen");

    // swagger-codegen generate -i https://petstore.swagger.io/v2/swagger.json -l ruby -o /tmp/test/
}
