use crate::models::Steps;
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "recipe-form.html")]
pub struct NewRecipeForm {
    pub ingredients: Vec<String>,
    pub steps: StepsPartial,
}

#[derive(Template, Default)]
#[template(path = "steps-partial.html")]
pub struct StepsPartial {
    pub steps: Steps,
}
