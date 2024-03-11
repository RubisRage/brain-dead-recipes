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
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let ingredient = s.split(',').collect::<Vec<_>>();

        let [name, quantity, unit] = ingredient.as_slice() else {
            return Err("expected 3 elements separated by commas");
        };

        let quantity: u32 = quantity.parse().map_err(|_| "invalid quantity")?;
        let unit = serde_plain::from_str(unit).map_err(|_| "invalid unit")?;

        Ok(Self {
            name: name.to_string(),
            quantity,
            unit,
        })
    }
}
