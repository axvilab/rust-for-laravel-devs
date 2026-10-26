//! `?page=2&per_page=20` -> `Page<T>` with `meta` — like `LengthAwarePaginator`.

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u32,
    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    20
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl Pagination {
    /// Computed in `i64`: `(page - 1) * per_page` in `u32` would overflow
    /// (and panic in debug builds) for `?page=4294967295`.
    pub fn offset(&self) -> i64 {
        (i64::from(self.page).max(1) - 1) * i64::from(self.per_page)
    }

    pub fn limit(&self) -> i64 {
        i64::from(self.per_page)
    }
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct PageMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub last_page: u32,
}

impl<T> Page<T> {
    pub fn new(data: Vec<T>, p: &Pagination, total: i64) -> Self {
        let per_page = i64::from(p.per_page.max(1));
        // integer ceil division; an empty result still has one (empty) page
        let last_page = ((total + per_page - 1) / per_page).max(1);
        Self {
            data,
            meta: PageMeta {
                page: p.page,
                per_page: p.per_page,
                total,
                last_page: u32::try_from(last_page).unwrap_or(u32::MAX),
            },
        }
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        Page {
            data: self.data.into_iter().map(f).collect(),
            meta: self.meta,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(page: u32, per_page: u32) -> Pagination {
        Pagination { page, per_page }
    }

    #[test]
    fn offset_and_limit() {
        assert_eq!(p(1, 20).offset(), 0);
        assert_eq!(p(3, 20).offset(), 40);
        assert_eq!(p(3, 20).limit(), 20);
        assert_eq!(p(u32::MAX, 100).offset(), (i64::from(u32::MAX) - 1) * 100);
    }

    #[test]
    fn validation_bounds() {
        assert!(p(1, 100).validate().is_ok());
        assert!(p(0, 20).validate().is_err());
        assert!(p(1, 0).validate().is_err());
        assert!(p(1, 1000).validate().is_err());
    }

    #[test]
    fn last_page() {
        assert_eq!(Page::<()>::new(vec![], &p(1, 20), 0).meta.last_page, 1);
        assert_eq!(Page::<()>::new(vec![], &p(1, 20), 20).meta.last_page, 1);
        assert_eq!(Page::<()>::new(vec![], &p(1, 20), 21).meta.last_page, 2);
    }

    #[test]
    fn negative_per_page_is_rejected_by_the_type() {
        // `u32` cannot be negative: `?per_page=-5` fails to deserialize at all.
        let r: Result<Pagination, _> = serde_json::from_str(r#"{"per_page": -5}"#);
        assert!(r.is_err());
    }
}
