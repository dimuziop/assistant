use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde_json::{json, Value};


#[test]
fn it_should_fail_when_get_tasks_without_auth() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/tasks").send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn it_should_get_tasks() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/tasks").headers(header_map).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn it_should_create_task() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/tasks").headers(header_map).json(&json!({
        "title": "this is a test task",
        "description": "some test description"
    })).send().unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let task: Value = response.json().unwrap();
    assert_eq!(task, json!({
        "id": task["id"],
        "title": "this is a test task",
        "description": "some test description",
        "estimated_time": null,
        "created_at": task["created_at"],
        "updated_at": null,
        "deleted_at": null,
    }));
}

#[test]
fn it_should_get_a_task() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/tasks").headers(header_map.clone()).json(&json!({
        "title": "this is a test task",
        "description": "some test description"
    })).send().unwrap();

    let task: Value = response.json().unwrap();

    let client = Client::new();
    let response = client.get(format!("http://127.0.0.1:8000/tasks/{}", task["id"].to_string().replace("\"", ""))).headers(header_map).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let actual: Value = response.json().unwrap();

    assert_eq!(task, actual);
}

#[test]
fn it_should_update_a_task() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/tasks").headers(header_map.clone()).json(&json!({
        "title": "this is a test task",
        "description": "some test description"
    })).send().unwrap();

    let task: Value = response.json().unwrap();

    let update_fields = &json!({
        "title": "this is an updated task",
        "description": "some test updated description"
    });

    let client = Client::new();
    let response = client
        .put(format!("http://127.0.0.1:8000/tasks/{}", task["id"]
            .to_string()
            .replace("\"", "")))
        .headers(header_map)
        .json(update_fields)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let actual: Value = response.json().unwrap();

    assert_eq!(json!({
        "id": task["id"],
        "title": "this is an updated task",
        "description": "some test updated description",
        "estimated_time": null,
        "created_at": task["created_at"],
        "updated_at": actual["updated_at"],
        "deleted_at": null,
    }), actual);
}

#[test]
fn it_should_soft_delete_a_task() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/tasks").headers(header_map.clone()).json(&json!({
        "title": "this is a test task",
        "description": "some test description"
    })).send().unwrap();

    let task: Value = response.json().unwrap();

    let client = Client::new();
    let response = client.delete(format!("http://127.0.0.1:8000/tasks/{}", task["id"].to_string().replace("\"", ""))).headers(header_map).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let actual: Value = response.json().unwrap();

    assert_eq!(task, json!({
        "id": task["id"],
        "title": "this is a test task",
        "description": "some test description",
        "estimated_time": null,
        "created_at": task["created_at"],
        "updated_at": null,
        "deleted_at": task["deleted_at"],
    }));
}

#[test]
fn it_should_return_not_found_on_a_soft_deletes_a_task() {
    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_static("Basic ZHNhZGFzOjEyMzQ2NQ=="));

    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/tasks").headers(header_map.clone()).json(&json!({
        "title": "this is a test task",
        "description": "some test description"
    })).send().unwrap();

    let task: Value = response.json().unwrap();

    let client = Client::new();
    let response = client.delete(format!("http://127.0.0.1:8000/tasks/{}", task["id"].to_string().replace("\"", ""))).headers(header_map.clone()).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let client = Client::new();
    let response = client.get(format!("http://127.0.0.1:8000/tasks/{}", task["id"].to_string().replace("\"", ""))).headers(header_map).send().unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}