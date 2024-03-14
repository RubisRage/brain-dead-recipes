-- Add migration script here

create table if not exists recipes (
    name text primary key,
    image text,
    rations integer not null,
    steps text not null
);

create table if not exists ingredients (
    name text primary key,
    diet_type text not null
);

create table if not exists recipe_ingredients (
    recipe_name text primary key,
    quantity integer not null,
    unit text not null,
    ingredient_name text not null references ingredients(name),
    foreign key (recipe_name) references recipes(name) on delete cascade
);
