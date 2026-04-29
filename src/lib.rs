use crate::{error::TodoError, todo_list::TodoResult};

pub mod cli_parser;
pub mod db_storage;
pub mod error;
pub mod file_storage;
pub mod todo_list;

#[cfg(test)]
pub mod tests;

pub async fn action_handler(
    action: cli_parser::Actions,
    todo_list: &mut impl todo_list::TodoOps,
    buf: &mut Vec<String>,
) -> Result<(), TodoError> {
    match action {
        cli_parser::Actions::Add { value } => todo_list.add(value).await.map(|_| ()),
        cli_parser::Actions::Delete { index } => todo_list.delete(index as i32).await.map(|_| ()),
        cli_parser::Actions::Show { index } => match index {
            Some(i) => {
                let result = todo_list.get(i as i32).await?;
                buf.push(result.to_string());
                Ok(())
            }
            None => {
                let result: Result<todo_list::TodoResult, TodoError> = todo_list.get_all().await;
                if let Ok(TodoResult::GottenAll(list)) = result {
                    buf.extend(list.iter().map(|v| v.task.clone()));
                }
                Ok(())
            }
        },
        cli_parser::Actions::Update { index, value } => {
            todo_list.update(index as i32, value).await.map(|_| ())
        }
        cli_parser::Actions::Clear => todo_list.clear().await.map(|_| ()),
    }
}
