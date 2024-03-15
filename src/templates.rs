use askama_axum::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "recipe-form.html")]
pub struct NewRecipeForm {
    pub ingredients: Vec<String>,
    pub steps: StepsPartial,
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
#[template(path = "recipe_view.html")]
pub struct RecipesTemplate {
    pub recipes: Vec<Recipe>,
}

