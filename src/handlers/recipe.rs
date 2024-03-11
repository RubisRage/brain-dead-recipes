use crate::{
    models::{RecipeIngredient, Steps},
    templates::{NewRecipeForm, StepsPartial},
};
use askama_axum::IntoResponse;
use axum::{
    http::{HeaderValue, StatusCode},
    routing::get,
    Router,
};
use axum_extra::extract::{Form, Query};
use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct RecipeCreationData {
    name: String,
    rations: u32,
    #[serde(rename = "ingredients[]")]
    ingredients: Vec<RecipeIngredient>,
    #[serde(flatten)]
    steps: Steps,
}

async fn create_recipe(
    Form(recipe): Form<RecipeCreationData>,
) -> impl IntoResponse {
    dbg!(&recipe);

    (StatusCode::OK, format!("Recipe: {:?}", recipe))
}

#[axum::debug_handler]
async fn new_recipe_form() -> NewRecipeForm {
    let ingredients = vec!["Flour", "Sugar", "Eggs", "Milk"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let steps = Default::default();

    NewRecipeForm { ingredients, steps }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StepsParam {
    steps_type: StepsType,
}

#[derive(Deserialize)]
enum StepsType {
    Text,
    Url,
    Image,
}

impl From<StepsType> for Steps {
    fn from(steps: StepsType) -> Self {
        match steps {
            StepsType::Text => Steps::Text(Default::default()),
            StepsType::Url => Steps::URL(
                "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                    .parse()
                    .expect("Default URL should be valid"),
            ),
            StepsType::Image => Steps::Image(Default::default()),
        }
    }
}

async fn steps_type(
    Query(StepsParam { steps_type }): Query<StepsParam>,
) -> impl IntoResponse {
    let steps = Steps::from(steps_type);

    let mut response = StepsPartial { steps }.into_response();

    response
        .headers_mut()
        .append("Cache-Control", HeaderValue::from_static("max-age=604800"));

    response
}

pub fn routes() -> Router {
    Router::new()
        .route("/new-recipe", get(new_recipe_form).post(create_recipe))
        .route("/steps-type", get(steps_type))
}
