use serde::{Deserialize, Serialize};

pub struct Recipe {
    name: String,
    thumbnail: Option<String>,
    rations: u32,
    ingredients: Vec<RecipeIngredient>,
    steps: Steps,
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

#[derive(Deserialize, Debug)]
//#[serde(try_from = "String")]
pub struct RecipeIngredient {
    pub recipe_name: String,
    pub quantity: u32,
    pub unit: IngredientUnit,
}

#[derive(sqlx::Type, Debug, Deserialize)]
#[sqlx(rename_all = "lowercase")]
pub enum IngredientUnit {
    Grams,
    Units,
}

impl TryFrom<String> for RecipeIngredient {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let ingredient = s.split(',').collect::<Vec<_>>();

        let [name, quantity, unit] = ingredient.as_slice() else {
            anyhow::bail!("missing either name, quantity or unit")
        };

        let quantity: u32 = quantity.parse()?;
        let unit = serde_plain::from_str(unit)?;

        Ok(Self {
            recipe_name: name.to_string(),
            quantity,
            unit,
        })
    }
}
