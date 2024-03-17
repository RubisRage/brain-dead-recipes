use super::extract::{RecipeCreationRequest, StepsPart};
use super::AppError;
use crate::{
    models::Steps,
    templates::{NewRecipeForm, StepsPartial, RecipesTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::State, http::HeaderValue, response::Redirect, routing::get, Router,
};
use axum_extra::extract::Query;
use serde::{self, Deserialize};
use serde_json::json;
use sqlx::SqlitePool;

async fn create_recipe(
    State(db): State<SqlitePool>,
    req: RecipeCreationRequest,
) -> Result<Redirect, AppError> {
    let name = lorust::kebab_case(req.name);
    let image: Option<String> = None;
    let rations = req.rations;
    let (steps, bytes) = match req.steps {
        StepsPart::Text(text) => (Steps::Text(text), None),
        StepsPart::Url(url) => (Steps::Url(url), None),
        StepsPart::Image(bytes, extension) => {
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
    .await?;

    for i in req.ingredients {
        sqlx::query!(
            r#"
            INSERT INTO recipe_ingredients 
            (recipe_name, ingredient_name, quantity, unit)
            VALUES
            (?, ?, ?, ?)
            "#,
            name,
            i.selected,
            i.quantity,
            i.unit
        )
        .execute(&mut *tx)
        .await?;
    }

    if let (Steps::Image(filename), Some(bytes)) = (steps, bytes) {
        tokio::fs::write(format!("images/{filename}"), bytes).await?;
    }

    tx.commit().await?;

    Ok(Redirect::to("/recipe/list"))
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


async fn recipes_view() -> RecipesTemplate {
    todo!("crear template con array de recetas");
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/new", get(new_recipe_form).post(create_recipe))
        .route("/type", get(steps_type))
        .route("/view", get(recipes-view))
}
