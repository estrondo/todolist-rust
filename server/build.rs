use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    compile_grpc_specification()?;
    Ok(())
}

fn compile_grpc_specification() -> Result<(), Box<dyn Error>> {
    let files: Vec<String> = ["account", "geo", "reminder", "security", "todo"]
        .iter()
        .map(|x| format!("../grpc-specification/todolist/v1/{x}.proto"))
        .collect();

    tonic_prost_build::configure()
        .build_client(false)
        .type_attribute(".todolist.v1.Todo.Item", "#[derive(o2o::o2o)] #[try_map_owned(todolist_core::model::todo::Todo, todolist_core::error::ConvertError)]")
        .field_attribute(".todolist.v1.Todo.Item", "#[into(~.try_into()?)] #[from(~.try_into()?)]")
        .compile_protos(&files, &[String::from("../grpc-specification/")])?;

    Ok(())
}
