#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::todo;
use rocket::{self, get, post, put, delete, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    checked: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "todo"]
struct NewTodo {
    title: String,
}

#[post("/", data = "<new_todo>")]
fn create_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo> {
    let result = diesel::insert_into(todo::table)
        .values(&*new_todo)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[put("/<id>/check")]
fn check_todo(conn: DbConn, id: i32) -> Json<Todo> {
    let target = todo::table.filter(todo::columns::id.eq(id));

    let result = diesel::update(target)
        .set(todo::columns::checked.eq(true))
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[put("/<id>/uncheck")]
fn uncheck_todo(conn: DbConn, id: i32) -> Json<Todo> {
    let target = todo::table.filter(todo::columns::id.eq(id));

    let result = diesel::update(target)
        .set(todo::columns::checked.eq(false))
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/")]
fn get_todos(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();

    Json(todos)
}

#[delete("/<id>")]
fn delete_todo(conn: DbConn, id: i32) -> Json<Todo> {
    let target = todo::table.filter(todo::columns::id.eq(id));

    let result = diesel::delete(target)
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/todos", routes![
            get_todos,
            create_todo,
            check_todo,
            uncheck_todo,
            delete_todo
        ])
        .launch();
}

