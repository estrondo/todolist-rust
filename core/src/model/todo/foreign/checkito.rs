use checkito::{FullGenerate, Generate, boxed::Boxed};
use orn::or2::Or;

use crate::model::foreign::checkito::{
    date_generator, duration_generator, non_empty_string_generator, utc_date_time_generator,
    uuid_generator,
};

use super::super::{Todo, TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle};

fn todo_id_generator() -> Boxed<TodoId> {
    uuid_generator().map(|uuid| TodoId(uuid)).boxed()
}

fn todo_title_generator() -> Boxed<TodoTitle> {
    non_empty_string_generator()
        .map(|title| TodoTitle(title))
        .boxed()
}

fn status_generator() -> Boxed<TodoStatus> {
    Generate::map(0..=4, |x| match x {
        0 => TodoStatus::Unspecified,
        1 => TodoStatus::Active,
        2 => TodoStatus::Postponed,
        3 => TodoStatus::Cancelled,
        4 => TodoStatus::Done,
        _ => panic!("Invalid TodoStatus:{x}"),
    })
    .boxed()
}

fn content_generator() -> Boxed<TodoContent> {
    let markdown = non_empty_string_generator();
    let plain = non_empty_string_generator();
    Generate::any((markdown, plain))
        .map(|or| match or {
            Or::T0(mark_down) => TodoContent::Markdown(mark_down),
            Or::T1(plain) => TodoContent::Plain(plain),
        })
        .boxed()
}

fn todo_due_date_generator() -> Boxed<TodoDueDate> {
    let date = date_generator();
    let date_time = utc_date_time_generator();
    let duration = duration_generator();

    Generate::any((date, (date_time, duration)))
        .map(|or| match or {
            Or::T0(date) => TodoDueDate::WholeDay(date),
            Or::T1((date_time, duration)) => TodoDueDate::Period(date_time, duration),
        })
        .boxed()
}

impl FullGenerate for Todo {
    type Item = Todo;

    type Generator = Boxed<Todo>;

    fn generator() -> Self::Generator {
        let id = todo_id_generator();
        let title = todo_title_generator();
        let due_date = todo_due_date_generator();
        let status = status_generator();
        let content = content_generator();

        Generate::map(
            (id, title, due_date, status, content),
            |(id, title, due_date, status, content)| Todo {
                id,
                title,
                due_date,
                status,
                content,
            },
        )
        .boxed()
    }
}


impl FullGenerate for TodoId {
    type Item = TodoId;

    type Generator = Boxed<TodoId>;

    fn generator() -> Self::Generator {
        todo_id_generator()
    }
}