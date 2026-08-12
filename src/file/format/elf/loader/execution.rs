use super::error::Error;
use super::load::load_path;
use super::stack::build_initial_stack;
use super::types::PreparedExecution;

pub fn prepare_execution(
    path: &str,
    path_pointer: *const u8,
    initial_stack: crate::target::arch::PointerType,
) -> Result<PreparedExecution, Error> {
    let image = load_path(path)?;
    let (entry, interpreter_base) = match image.interpreter {
        Some(interpreter) => {
            let interpreter_path = interpreter.as_str().ok_or(Error::InvalidInterpreter)?;
            let interpreter_image = load_path(interpreter_path)?;
            (interpreter_image.entry, interpreter_image.base as usize)
        }
        None if image.direct_entry => (image.entry, 0),
        None => return Err(Error::InterpreterUnavailable),
    };

    let stack_pointer =
        build_initial_stack(initial_stack, path, path_pointer, &image, interpreter_base)?;
    Ok(PreparedExecution {
        image,
        entry,
        stack_pointer,
    })
}
