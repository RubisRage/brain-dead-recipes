use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
pub struct Recipe {
    pub name: String,
    pub thumbnail: Option<String>,
    pub rations: u32,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Steps,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Steps {
    Text(String),
    Url(url::Url),
    Image(String),
}

impl Default for Steps {
    fn default() -> Self {
        Self::Text("".to_string())
    }
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct RecipeIngredient {
    pub recipe_name: String,
    pub ingredient_name: String,
    pub quantity: u32,
    pub unit: IngredientUnit,
}

#[derive(sqlx::Type, Debug, Deserialize)]
#[sqlx(rename_all = "lowercase")]
pub enum IngredientUnit {
    Grams,
    Units,
}
