use crate::schema::todo;

#[derive(Queryable)]
pub struct Todo {
    pub todo_id: i32,
    pub task: String,
    pub priority: i16
}

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub priority: &'a i16
}