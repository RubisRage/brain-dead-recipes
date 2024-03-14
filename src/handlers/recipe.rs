use crate::{
    models::{RecipeIngredient, Steps},
    templates::{NewRecipeForm, StepsPartial},
};
use anyhow::{anyhow, Context};
use askama_axum::IntoResponse;
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Multipart, Request, State},
    http::{HeaderValue, StatusCode},
    response::Redirect,
    routing::get,
    Router,
};
use axum_extra::extract::Query;
use serde::{self, Deserialize};
use serde_json::json;
use sqlx::SqlitePool;

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

async fn create_recipe(
    State(db): State<SqlitePool>,
    req: RecipeCreationRequest,
) -> Redirect {
    let name = lorust::kebab_case(req.name);
    let image: Option<String> = None;
    let rations = req.rations;
    let (steps, bytes) = match req.steps {
        StepsCreationRequest::Text(text) => (Steps::Text(text), None),
        StepsCreationRequest::Url(url) => (Steps::Url(url), None),
        StepsCreationRequest::Image(bytes, extension) => {
            (Steps::Image(format!("{}.{}", name, extension)), Some(bytes))
        }
    };

    let mut tx = db.begin().await.unwrap();
    let steps_serialized = json!(steps);

    sqlx::query!(
        r#"
        INSERT INTO recipes (name, image, rations, steps) VALUES (?, ?, ?, ?)
        "#,
        name,
        image,
        rations,
        steps_serialized
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    // TODO: Store ingredients in db

    if let (Steps::Image(filename), Some(bytes)) = (steps, bytes) {
        tokio::fs::write(format!("images/{filename}"), bytes)
            .await
            .unwrap();
    }

    tx.commit().await.unwrap();

    Redirect::to("/recipes")
}

#[axum::debug_handler]
async fn new_recipe_form(State(db): State<SqlitePool>) -> NewRecipeForm {
    let ingredients = sqlx::query_scalar::<_, String>(
        r#"
        SELECT name 
        FROM ingredients
        "#,
    )
    .fetch_all(&db)
    .await
    .unwrap();

    NewRecipeForm {
        ingredients,
        steps: StepsPartial::default(),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StepsParam {
    steps_type: StepsPartial,
}

async fn steps_type(
    Query(StepsParam { steps_type }): Query<StepsParam>,
) -> impl IntoResponse {
    let mut response = steps_type.into_response();

    response
        .headers_mut()
        .append("Cache-Control", HeaderValue::from_static("max-age=604800"));

    response
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/new-recipe", get(new_recipe_form).post(create_recipe))
        .route("/steps-type", get(steps_type))
}
