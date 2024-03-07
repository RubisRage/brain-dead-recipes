use std::{collections::HashMap, str::FromStr};

use askama_axum::IntoResponse;
use axum::{http::StatusCode, routing::get, Router};
use axum_extra::extract::Form;
use serde::{self, Deserialize};
use tower_http::{services::ServeDir, trace::TraceLayer};

mod handlers;
mod templates;

use templates::{Index, RecipeForm};

async fn index() -> Index {
    Index {}
}

async fn recipe_form() -> RecipeForm {
    let ingredients = vec!["Flour", "Sugar", "Eggs", "Milk"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    RecipeForm { ingredients }
}

#[derive(Debug)]
enum IngredientUnit {
    Grams,
    Units,
}

impl FromStr for IngredientUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "g" => Ok(Self::Grams),
            "unds" => Ok(Self::Units),
            _ => Err("Valid units are: g, unds"),
        }
    }
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

        let unit = IngredientUnit::from_str(unit)?;

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

    #[serde(rename = "recipe-steps")]
    steps: String,
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
        .nest_service("/assets", ServeDir::new("dist"))
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
