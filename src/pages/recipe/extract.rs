use super::AppError;

use crate::models::RecipeIngredient;
use anyhow::{anyhow, Context};
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Multipart, Request},
};

#[derive(Debug)]
pub enum StepsCreationRequest {
    Text(String),
    Url(url::Url),
    Image(Bytes, String),
}

#[derive(Debug)]
pub struct RecipeCreationRequest {
    pub name: String,
    pub rations: u32,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: StepsCreationRequest,
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

        if let (Some(name), Some(rations), Some(steps)) = (name, rations, steps)
        {
            Ok(RecipeCreationRequest {
                name,
                rations,
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
