-- Add migration script here

create table if not exists recipes (
    name text primary key,
    thumbnail text,
    rations integer not null,
    steps text not null
);

create table if not exists ingredients (
    name text primary key,
    diet_type text not null
);

create table if not exists recipe_ingredients (
    recipe_name text references recipes(name) on delete cascade,
    ingredient_name text not null references ingredients(name),
    quantity integer not null,
    unit text not null,
    primary key(recipe_name, ingredient_name)
);
