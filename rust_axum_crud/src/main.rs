use axum::{
    extract::{Path,State},
    http::StatusCode,
    routing::{get,patch,post},
    Router,Json,
};


use serde::{Serialize, Deserialize};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions,PgPool};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //expose the env variables

     dotenvy::dotenv().expect("Unable to access .env file");

    // set varibles from the environment varibles
     let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:5432".to_owned());
     println!("Setting server address {}", server_address);
     let database_url = std::env::var("DATABASE_URL").expect("Unable to access DATABASE_URL in .env");
    // create the database pool
     let db_pool = PgPoolOptions::new().max_connections(16).connect(&database_url).await.expect("Can't connect to database");
    // create our TCP listener
    let listener = TcpListener::bind(server_address).await.expect("Couldn't create TCP listener");


    println!("listening on {}", listener.local_addr().unwrap());
    // compose the routes
   let app = Router::new()
   .route("/",get(|| async{"Hello world"}))
   .route("/tasks",get(get_tasks).post(create_task))
   //.route("/tasks",post(create_task))
  // .route("/post_tasks",post(create_task))
   .route("/tasks/:task_id",patch(update_task).delete(delete_task))
   .with_state(db_pool);

    // server the application

    axum::serve(listener,app).await.expect("Error serving the application");


   // println!("Hello, world!");
}
#[derive(Serialize)]
struct TaskRow{
    task_id: i32,
    name: String,
    priority: Option<i32>
}

async fn get_tasks (
    State(pg_pool):State<PgPool>
) -> Result<(StatusCode, String),(StatusCode, String)>{
   let rows = sqlx::query_as!(TaskRow,"SELECT * FROM tasks ORDER BY task_id")
    .fetch_all(&pg_pool)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
        })?;
        Ok((
            StatusCode::OK,
            json!({"success": true, "data": rows}).to_string(),
        ))
}

#[derive(Deserialize)]
struct CreateTaskReq{
    name: String,
    priority: Option<i32>
}

#[derive(Serialize)]
struct CreateTaskRow{
  task_id: i32,
}
async fn create_task(
    State(pg_pool):State<PgPool>,Json(task):Json<CreateTaskReq>,
) -> Result<(StatusCode, String),(StatusCode, String)>{
    let row = sqlx::query_as!(CreateTaskRow, "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id ",
    task.name, 
    task.priority
).fetch_one(&pg_pool)
 .await
 .map_err(|e|{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": e.to_string()}).to_string(),
    )
    })?;
       
        Ok((
            StatusCode::CREATED,
            json!({"success": true, "data": row}).to_string(),
        ))
}

#[derive(Deserialize)]
struct UpdateTaskReq{
    name: Option<String>,
    priority: Option<i32>

}
async fn update_task(
    State(pg_pool):State<PgPool>, Path(task_id):Path<i32>,Json(task):Json<UpdateTaskReq>,
) -> Result<(StatusCode, String),(StatusCode, String)>{
    sqlx::query!("UPDATE tasks SET name = $2,priority = $3 WHERE task_id = $1",
    task_id,
    task.name,
    task.priority)
    .execute(&pg_pool)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
        })?;
    Ok((
        StatusCode::OK,
        json!({"success": true}).to_string(),
    ))
}
async fn delete_task (
    State(pg_pool):State<PgPool>,
    Path(task_id):Path<i32>,
) -> Result<(StatusCode, String),(StatusCode, String)>{
    sqlx::query!("DELETE FROM tasks WHERE task_id = $1",
    task_id)
    .execute(&pg_pool)
    .await
    .map_err(|e|{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
        })?;
    Ok((
        StatusCode::OK,
        json!({"success": true}).to_string(),
    ))
}