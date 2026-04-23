use rust_todo_list::init_file_todo;

/// This function performs operation through cli arguments, and <br>
/// the list is store in file. This doesn't use client-sever architecture
#[tokio::main]
async fn main() {
    init_file_todo().await;
}
