use askama_axum::IntoResponse;
use axum::{http::StatusCode, routing::get, Router};
use axum_extra::extract::{Form, Query};
use serde::{self, Deserialize};
use std::str::FromStr;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod handlers;
mod templates;

use templates::{Index, RecipeForm};

use crate::templates::StepsPartial;

async fn index() -> Index {
    Index {}
}

async fn recipe_form() -> RecipeForm {
    let ingredients = vec!["Flour", "Sugar", "Eggs", "Milk"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let steps = Default::default();

    RecipeForm { ingredients, steps }
}

#[derive(Debug, Deserialize)]
enum IngredientUnit {
    Grams,
    Units,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
struct RecipeIngredient {
    name: String,
    quantity: u32,
    unit: IngredientUnit,
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

#[derive(Deserialize, Debug)]
struct RecipeCreationData {
    #[serde(rename = "recipe-name")]
    name: String,

    #[serde(rename = "diners-number")]
    diners: u32,

    #[serde(rename = "ingredients[]")]
    ingredients: Vec<RecipeIngredient>,

    #[serde(flatten)]
    steps: Steps,
}

struct Recipe {
    name: String,
    diners: u32,
    ingredients: Vec<RecipeIngredient>,
    steps: Steps,
    image: Option<String>,
}

#[derive(Debug, Deserialize)]
enum Steps {
    Text(String),
    URL(url::Url),
    Image(String),
}

impl Default for Steps {
    fn default() -> Self {
        Self::Text("".to_string())
    }
}

#[derive(Deserialize)]
#[serde(rename = "steps-type")]
enum StepsParam {
    Text,
    Url,
    Image,
}

#[derive(Deserialize)]
struct StepsParamWrapper {
    #[serde(rename = "steps-type")]
    steps: StepsParam,
}

impl From<StepsParam> for Steps {
    fn from(steps: StepsParam) -> Self {
        match steps {
            StepsParam::Text => Steps::Text(Default::default()),
            StepsParam::Url => Steps::URL(
                "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                    .parse()
                    .expect("Default URL should be valid"),
            ),
            StepsParam::Image => Steps::Image(Default::default()),
        }
    }
}

#[axum::debug_handler]
async fn steps_type(
    Query(StepsParamWrapper { steps }): Query<StepsParamWrapper>,
) -> StepsPartial {
    let steps = Steps::from(steps);

    StepsPartial { steps }
}

async fn create_recipe(
    Form(recipe): Form<RecipeCreationData>,
) -> impl IntoResponse {
    dbg!(&recipe);

    (StatusCode::OK, format!("Recipe: {:?}", recipe))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/recipe", get(recipe_form).post(create_recipe))
        .route("/recipe/steps", get(steps_type))
        .nest_service("/assets", ServeDir::new("dist"))
        .nest_service("/images", ServeDir::new("images"))
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
