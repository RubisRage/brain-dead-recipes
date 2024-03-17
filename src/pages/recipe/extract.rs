use super::AppError;

use crate::models::IngredientUnit;
use anyhow::{anyhow, Context};
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Multipart, Request},
};
use serde::Deserialize;

#[derive(Debug)]
pub struct RecipeCreationRequest {
    pub name: String,
    pub rations: u32,
    pub thumbnail: Option<(Bytes, String)>,
    pub ingredients: Vec<RecipeIngredientPart>,
    pub steps: StepsPart,
}

#[derive(Debug)]
pub enum StepsPart {
    Text(String),
    Url(url::Url),
    Image(Bytes, String),
}

#[derive(Debug, Deserialize)]
pub struct RecipeIngredientPart {
    pub selected: String,
    #[serde()]
    pub quantity: u32,
    pub unit: IngredientUnit,
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
        let mut thumbnail = None;
        let mut ingredients = vec![];
        let mut steps = None;

        let mut set_steps = |s| {
            if steps.replace(s).is_none() {
                Ok(())
            } else {
                return Err(anyhow!("multiple steps types specified"));
            }
        };

        let mut get_extension = |ct: &str| -> Result<String, AppError> {
            let (_, extension) = if ct == "image/jpeg" {
                ct.split_once('/')
                    .expect("/ should be present on valid content types")
            } else {
                return Err(anyhow!("invalid content type").into());
            };

            Ok(extension.to_string())
        };

        while let Some(field) = body.next_field().await? {
            match field.name() {
                Some("name") => {
                    name = Some(field.text().await?);
                }

                Some("rations") => {
                    rations = Some(field.text().await?.parse::<u32>()?);
                }

                Some("thumbnail") => {
                    let content_type = field
                        .content_type()
                        .context("content type not present")?
                        .to_string();

                    let extension = get_extension(&content_type)?;

                    thumbnail = Some((field.bytes().await?, extension));
                }

                Some("ingredients[]") => ingredients
                    .push(serde_json::from_str(&field.text().await?)?),

                Some("Text") => {
                    set_steps(StepsPart::Text(field.text().await?))?
                }

                Some("URL") => {
                    set_steps(StepsPart::Url(field.text().await?.parse()?))?
                }

                Some("Image") => {
                    let content_type = field
                        .content_type()
                        .context("content type not present")?
                        .to_string();

                    // TODO Check for other valid image types
                    let extension = get_extension(&content_type)?;

                    set_steps(StepsPart::Image(
                        field.bytes().await?,
                        extension,
                    ))?;
                }

                _ => return Err(anyhow!("invalid field").into()),
            }
        }

        if let (Some(name), Some(rations), Some(steps)) = (name, rations, steps)
        {
            Ok(RecipeCreationRequest {
                name,
                rations,
                thumbnail,
                steps,
                ingredients,
            })
        } else {
            Err(anyhow!(
                "recipe is missing at least one of name, rations or steps fields"
            )
            .into())
        }
    }
}
