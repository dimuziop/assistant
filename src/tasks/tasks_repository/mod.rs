use diesel::dsl;
use diesel::prelude::*;
use crate::schema::tasks;
use crate::tasks::task::Task;

pub struct TasksRepository;

impl TasksRepository {
    pub fn find(c: &mut PgConnection, id: String) -> QueryResult<Task> {
        tasks::table.find(id).get_result::<Task>(c)
    }

    pub fn all(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Task>> {
        tasks::table
            .order(tasks::created_at.desc())
            .filter(tasks::deleted_at.is_null())
            .limit(limit)
            .load::<Task>(c)
    }

    pub fn add(c: &mut PgConnection, task: Task) -> QueryResult<Task> {
        diesel::insert_into(tasks::table)
            .values(task)
            .get_result(c)
    }

    pub fn replace(c: &mut PgConnection, id: String, task: Task) -> QueryResult<Task> {
        diesel::update(tasks::table
            .filter(tasks::id.eq(id)))
            .filter(tasks::deleted_at.is_null())
            .set((
                tasks::title.eq(task.title),
                tasks::description.eq(task.description),
                tasks::updated_at.eq(dsl::now)
            ))
            .get_result(c)
    }

    pub fn soft_delete(c: &mut PgConnection, id: String)  -> QueryResult<Task> {
        diesel::update(tasks::table
            .filter(tasks::id.eq(id)))
            .filter(tasks::deleted_at.is_null())
            .set(tasks::deleted_at.eq(dsl::now))
            .get_result(c)
    }

    pub fn delete(c: &mut PgConnection, id: String) -> QueryResult<usize> {
        diesel::delete(tasks::table.filter(tasks::id.eq(id))).execute(c)
    }
}