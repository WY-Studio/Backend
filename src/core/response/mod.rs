use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Base<T> {
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "Some(data)")]
    pub data: Option<T>,
    #[schema(example = "성공했다구~")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paged<T> {
    pub code: u16,
    pub data: PagedData<T>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedData<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
    pub total: u64,
    pub total_pages: u32,
}

impl<T> Base<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: "성공했다구~".to_string(),
        }
    }

    pub fn success_msg(data: T, message: String) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message,
        }
    }

    pub fn error(code: u16, message: String) -> Base<()> {
        Base {
            code,
            data: None,
            message,
        }
    }
}

impl<T> Paged<T> {
    pub fn success(items: Vec<T>, page: u32, size: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (size as f64)).ceil() as u32;

        Self {
            code: 200,
            data: PagedData {
                items,
                pagination: Pagination {
                    page,
                    size,
                    total,
                    total_pages,
                },
            },
            message: "성공했다구~".to_string(),
        }
    }
}
