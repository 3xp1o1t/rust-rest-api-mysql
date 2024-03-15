use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::json;

use crate::{
    model::{NoteModel, NoteModelResponse},
    schema::{AppState, CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

// Health check
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status":"ok",
        "message": MESSAGE
    });

    Json(json_response)
}

// Listar notas
pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serder_json::Value>)> {
    // Parametro
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // Query with macro
    let notes = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
          "status": "error",
          "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    // Respuesta
    let note_response = notes
        .iter()
        .map(|note| to_note_response(&note))
        .collect::<Vec<NoteModelResponse>>();

    let json_response = serde_json::json!({
      "status":"ok",
      "count": note_response.len(),
      "notes": note_response
    });

    Ok(Json(json_response));
}

// crear nota
pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // insertar
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO notes (id, title, content) VALUES (?, ?, ?)"#)
        .bind(id.clone())
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
              "status": "error",
              "message": "La nota ya existe",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": format!("{:?}", err)})),
        ));
    }

    // obtener el id de la nota insertada
    let note = sqlx::query_as!(NoteModel, r#"SELECT * FROM notes WHERE id = ?"#, id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": format!("{:?}", e)})),
            )
        })?;

    let note_response = serde_json::json!({
      "status": "success",
      "data": serde_json::json!({
        "note": to_note_response(&note)
      })
    });

    Ok(Json(note_response))
}

// Obtener nota especifica por id
pub async fn get_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // obtener usando el macro de query
    let query_result = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    // revisar respuesta
    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
              "status": "success",
              "data": serde_json::json!({
                "note": to_note_response(&note)
              })
            });

            return Ok(Json(note_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
              "status": "fail",
              "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status":"error", "message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn edit_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // validate note with query macro
    let query_result = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    // fetch the result
    let note = match query_result {
        Ok(note) => note,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    // parse data
    let is_published = body.is_published.unwrap_or(note.is_published != 0);
    let i8_is_published = is_published as i8;

    // Update (if empty, use old value)
    let update_result =
        sqlx::query(r#"UPDATE notes SET title = ?, content = ?, is_published = ? WHERE id = ?"#)
            .bind(body.title.to_owned().unwrap_or_else(|| note.title.clone()))
            .bind(
                body.content
                    .to_owned()
                    .unwrap_or_else(|| note.content.clone()),
            )
            .bind(i8_is_published)
            .bind(id.to_string())
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;

    // if no data affected (or deleted when wanted to update)
    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    // get updated data
    let updated_note = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!({
            "note": to_note_response(&updated_note)
        })
    });

    Ok(Json(note_response))
}

pub async fn delete_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // delete with query macro
    let query_result = sqlx::query!(r#"DELETE FROM notes WHERE id = ?"#, id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    // response
    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Convertir el modelo a una respuesta
fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published != 0,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
    }
}
