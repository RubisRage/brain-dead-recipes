-- Add migration script here

create table if not exists recipes (
    name text primary key,
    rations integer not null,
    steps text not null
);

create table if not exists ingredients (
    name text primary key,
    quantity integer not null,
    unit text not null,
    recipe text not null,
    foreign key(recipe) references recipes(name)
);
