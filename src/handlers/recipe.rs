use std::path::Path;

use crate::{
    models::{RecipeIngredient, Steps},
    templates::{NewRecipeForm, StepsPartial},
};
use anyhow::anyhow;
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

struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[async_trait]
impl<S> FromRequest<S> for RecipeCreationData
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let mut body = Multipart::from_request(req, state).await?;

        let mut name = None;
        let mut rations = None;
        let mut ingredients = vec![];
        let mut steps = None;

        while let Some(field) = body.next_field().await? {
            match field.name() {
                Some("name") => {
                    name = Some(field.text().await?);
                }

                Some("rations") => {
                    rations = Some(field.text().await?.parse::<u32>()?);
                }

                Some("ingredients[]") => ingredients
                    .push(RecipeIngredient::try_from(field.text().await?)?),

                Some("Text") => steps = Some(Steps::Text(field.text().await?)),

                Some("URL") => {
                    if steps
                        .replace(Steps::URL(field.text().await?.parse()?))
                        .is_some()
                    {
                        return Err(
                            anyhow!("Multiple steps types specified!").into()
                        );
                    }
                }

                Some("Image") => {
                    let content_type = field.content_type();

                    if let Some(content_type) = content_type {
                        if content_type != "image/jpeg" {
                            return Err(anyhow!("invalid content type").into());
                        }
                    } else {
                        return Err(anyhow!("content type not present").into());
                    }

                    let Some(filename) = field.file_name() else {
                        return Err(anyhow!("filename not present").into());
                    };

                    let path = format!("images/{}", filename);
                    let path = Path::new(&path);

                    if steps
                        .replace(Steps::Image(filename.to_string()))
                        .is_some()
                    {
                        return Err(
                            anyhow!("Multiple steps types specified!").into()
                        );
                    }

                    let bytes = field.bytes().await?;

                    tokio::fs::write(&path, bytes).await?;
                }

                _ => return Err(anyhow!("invalid field").into()),
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

            _ => Err(anyhow!(
                "Recipe is either missing name, rations or steps fields"
            )
            .into()),
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
