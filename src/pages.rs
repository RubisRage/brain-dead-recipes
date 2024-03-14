pub mod recipe;

use crate::templates::Index;

pub async fn index() -> Index {
    Index {}
}
