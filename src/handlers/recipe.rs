use std::path::Path;

use crate::{
    models::{RecipeIngredient, Steps},
    templates::{NewRecipeForm, StepsPartial},
};
use askama_axum::IntoResponse;
use axum::{
    async_trait,
    extract::{FromRequest, Multipart, Request},
    http::{HeaderValue, StatusCode},
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use serde::{self, Deserialize};

#[derive(Debug)]
struct RecipeCreationData {
    name: String,
    rations: u32,
    ingredients: Vec<RecipeIngredient>,
    steps: Steps,
}

#[async_trait]
impl<S> FromRequest<S> for RecipeCreationData
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let mut body = Multipart::from_request(req, state)
            .await
            .map_err(|e| e.body_text())?;

        let mut name = None;
        let mut rations = None;
        let mut ingredients = vec![];
        let mut steps = None;

        while let Some(mut field) =
            body.next_field().await.map_err(|e| e.body_text())?
        {
            match field.name() {
                Some("name") => {
                    name = Some(field.text().await.map_err(|e| e.body_text())?);
                }

                Some("rations") => {
                    rations = Some(
                        field
                            .text()
                            .await
                            .map_err(|e| e.body_text())?
                            .parse::<u32>()
                            .map_err(|_| {
                                "Rations must be a valid number!".to_string()
                            })?,
                    );
                }

                Some("ingredients[]") => {
                    ingredients.push(RecipeIngredient::try_from(
                        field.text().await.map_err(|e| e.body_text())?,
                    )?)
                }

                Some("Text") => {
                    steps = Some(Steps::Text(
                        field.text().await.map_err(|e| e.body_text())?,
                    ))
                }

                Some("URL") => {
                    let previous = steps.replace(Steps::URL(
                        field
                            .text()
                            .await
                            .map_err(|e| e.body_text())?
                            .parse()
                            .map_err(|_| "Link must be a valid URL!")?,
                    ));

                    if previous.is_some() {
                        return Err(
                            "Multiple steps types specified!".to_string()
                        );
                    }
                }

                Some("Image") => {
                    let content_type = field
                        .content_type()
                        .ok_or("Content type not present")?;

                    if content_type != "image/jpeg" {
                        return Err("Invalid content type".to_string());
                    }

                    let filename = field
                        .file_name()
                        .ok_or("File name not present")?
                        .to_string();

                    let path = format!("images/{}", filename);
                    let path = Path::new(&path);

                    if steps.replace(Steps::Image(filename)).is_some() {
                        return Err(
                            "Multiple steps types specified!".to_string()
                        );
                    }

                    let bytes =
                        field.bytes().await.map_err(|e| e.body_text())?;

                    tokio::fs::write(&path, bytes)
                        .await
                        .map_err(|e| e.to_string())?;
                }

                _ => Err("Invalid parameter!".to_string())?,
            }
        }

        match (name, rations, steps) {
            (Some(name), Some(rations), Some(steps)) => {
                Ok(RecipeCreationData {
                    name,
                    rations,
                    steps,
                    ingredients,
                })
            }

            _ => Err("Recipe is either missing name, rations or steps fields"
                .to_string()),
        }
    }
}

async fn create_recipe(recipe: RecipeCreationData) -> impl IntoResponse {
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
