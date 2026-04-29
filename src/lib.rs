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
        cli_parser::Actions::Add { task } => todo_list.add(task).await.map(|_| ()),
        cli_parser::Actions::Delete { id } => todo_list.delete(id).await.map(|_| ()),
        cli_parser::Actions::Show { id } => match id {
            Some(i) => {
                let result = todo_list.get(i).await?;
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
        cli_parser::Actions::Update { id, task } => todo_list.update(id, task).await.map(|_| ()),
        cli_parser::Actions::Clear => todo_list.clear().await.map(|_| ()),
    }
}
