use super::extract::{RecipeCreationRequest, StepsPart};
use super::AppError;
use crate::models::Ingredient;
use crate::templates::IngredientsList;
use crate::{
    models::Steps,
    templates::{NewRecipeForm, StepsPartial},
};
use askama_axum::IntoResponse;
use axum::http::StatusCode;
use axum::Form;
use axum::{
    extract::State,
    http::HeaderValue,
    response::Redirect,
    routing::{get, post},
    Router,
};
use axum_extra::extract::Query;
use axum_htmx::HxResponseTrigger;
use serde::{self, Deserialize};
use serde_json::json;
use sqlx::SqlitePool;

async fn create_recipe(
    State(db): State<SqlitePool>,
    req: RecipeCreationRequest,
) -> Result<Redirect, AppError> {
    let name = lorust::kebab_case(req.name);
    let rations = req.rations;
    let (steps, bytes) = match req.steps {
        StepsPart::Text(text) => (Steps::Text(text), None),
        StepsPart::Url(url) => (Steps::Url(url), None),
        StepsPart::Image(bytes, extension) => (
            Steps::Image(format!("{}-step.{}", name, extension)),
            Some(bytes),
        ),
    };

    let (thumbnail_bytes, thumbnail_name) =
        req.thumbnail.map_or((None, None), |(bytes, extension)| {
            let name = format!("{}.{}", name, extension);
            (Some(bytes), Some(name))
        });

    let mut tx = db.begin().await.unwrap();
    let steps_serialized = json!(steps);

    sqlx::query!(
        r#"
        INSERT INTO recipes (name, thumbnail, rations, steps) VALUES (?, ?, ?, ?)
        "#,
        name,
        thumbnail_name,
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

    if let (Some(filename), Some(bytes)) = (thumbnail_name, thumbnail_bytes) {
        tokio::fs::write(format!("images/{filename}"), bytes).await?;
    }

    if let (Steps::Image(filename), Some(bytes)) = (steps, bytes) {
        tokio::fs::write(format!("images/{filename}"), bytes).await?;
    }

    tx.commit().await?;

    Ok(Redirect::to("/recipe/view"))
}

#[axum::debug_handler]
async fn new_recipe_form(State(db): State<SqlitePool>) -> NewRecipeForm {
    let ingredients = sqlx::query_as(
        r#"
        SELECT name, diet_type as diet
        FROM ingredients
        ORDER BY name
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

async fn new_ingredient(
    State(db): State<SqlitePool>,
    Form(new_ingredient): Form<Ingredient>,
) -> Result<impl IntoResponse, &'static str> {
    let name = lorust::kebab_case(new_ingredient.name);

    let res = sqlx::query!(
        r#"
        INSERT INTO ingredients (name, diet_type) VALUES (?, ?)
        "#,
        name,
        new_ingredient.diet
    )
    .execute(&db)
    .await;

    if res.is_ok() {
        Ok((
            HxResponseTrigger::normal(["newIngredient"]),
            StatusCode::CREATED,
        ))
    } else {
        Err("Ya existe un ingrediente con este nombre")
    }
}

#[axum::debug_handler]
async fn list_ingredients(State(db): State<SqlitePool>) -> IngredientsList {
    let ingredients = sqlx::query_as(
        r#"
        SELECT name, diet_type as diet
        FROM ingredients
        ORDER BY name
        "#,
    )
    .fetch_all(&db)
    .await
    .unwrap();

    IngredientsList { ingredients }
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/new", get(new_recipe_form).post(create_recipe))
        .route("/type", get(steps_type))
        .route("/ingredient", get(list_ingredients).post(new_ingredient))
}
