use serde::{Deserialize, Serialize};

// Listar
// Contara con paginacion y limitacion
// Ej: http://localhost:8080/api/notes?page=0&limit=10
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Leer/Eliminar
// Tendra el parametro id para identificar el registro a eliminar/buscar
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

// Crear
// Al igual que UpdateNoteSchema, servira para actualizar o crear la nota
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

// Actualizar
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}
