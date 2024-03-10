use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "recipe-form.html")]
pub struct RecipeForm {
    pub ingredients: Vec<String>,
}


#[derive(Template)]
#[template(path = "recipe_view.html")]
pub struct RecipesTemplate {
    pub recipes: Vec<String>,
}

