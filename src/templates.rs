use askama_axum::Template;
use serde::Deserialize;

use crate::models::{Ingredient, Recipe};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "recipe-form.html")]
pub struct NewRecipeForm {
    pub ingredients: Vec<Ingredient>,
    pub steps: StepsPartial,
}

#[derive(Template)]
#[template(
    source = r#" 
        {%- import "ingredient-input.html" as ingredient_input -%}
        {% call ingredient_input::ingredients_template(ingredients) %}
    "#,
    ext = "html"
)]
pub struct IngredientsList {
    pub ingredients: Vec<Ingredient>,
}

#[derive(Template, Deserialize, Default)]
#[template(path = "steps-partial.html")]
pub enum StepsPartial {
    #[default]
    Text,
    Url,
    Image,
}

#[derive(Template)]
#[template(path = "recipe.html", print = "code")]
pub struct RecipeTemplate {
    pub recipe: Recipe,
}
