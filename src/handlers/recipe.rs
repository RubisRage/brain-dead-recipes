use crate::{
    models::{RecipeIngredient, Steps},
    templates::{NewRecipeForm, StepsPartial},
};
use anyhow::{anyhow, Context};
use askama_axum::IntoResponse;
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Multipart, Request},
    http::{HeaderValue, StatusCode},
    response::Redirect,
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use serde::{self, Deserialize};

#[derive(Debug)]
enum StepsCreationRequest {
    Text(String),
    Url(url::Url),
    Image(Bytes, String),
}

#[derive(Debug)]
struct RecipeCreationRequest {
    name: String,
    rations: u32,
    ingredients: Vec<RecipeIngredient>,
    steps: StepsCreationRequest,
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
impl<S> FromRequest<S> for RecipeCreationRequest
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

        let mut set_steps = |s| {
            if steps.replace(s).is_none() {
                Ok(())
            } else {
                return Err(anyhow!("multiple steps types specified"));
            }
        };

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

                Some("Text") => {
                    set_steps(StepsCreationRequest::Text(field.text().await?))?
                }

                Some("URL") => set_steps(StepsCreationRequest::Url(
                    field.text().await?.parse()?,
                ))?,

                Some("Image") => {
                    let content_type = field
                        .content_type()
                        .context("content type not present")?
                        .to_string();

                    // TODO Check for other valid image types
                    let (_, extension) = if content_type == "image/jpeg" {
                        content_type.split_once('/').expect(
                            "/ should be present on valid content types",
                        )
                    } else {
                        return Err(anyhow!("invalid content type").into());
                    };

                    set_steps(StepsCreationRequest::Image(
                        field.bytes().await?,
                        extension.to_string(),
                    ))?;
                }

                _ => return Err(anyhow!("invalid field").into()),
            }
        }

        match (name, rations, steps) {
            (Some(name), Some(rations), Some(steps)) => {
                Ok(RecipeCreationRequest {
                    name,
                    rations,
                    steps,
                    ingredients,
                })
            }

            _ => {
                return Err(anyhow!(
                    "recipe is either missing name, rations or steps fields"
                )
                .into());
            }
        }
    }
}

async fn create_recipe(recipe: RecipeCreationRequest) -> impl IntoResponse {
    // TODO design database schema
    Redirect::to("/recipes")
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
