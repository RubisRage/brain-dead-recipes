use anyhow::bail;
use serde::Deserialize;

pub struct Recipe {
    name: String,
    diners: u32,
    ingredients: Vec<RecipeIngredient>,
    steps: Steps,
    image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Steps {
    Text(String),
    URL(url::Url),
    Image(String),
}

impl Default for Steps {
    fn default() -> Self {
        Self::Text("".to_string())
    }
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct RecipeIngredient {
    name: String,
    quantity: u32,
    unit: IngredientUnit,
}

#[derive(Debug, Deserialize)]
enum IngredientUnit {
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
            name: name.to_string(),
            quantity,
            unit,
        })
    }
}
