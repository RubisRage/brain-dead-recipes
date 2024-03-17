-- Add migration script here

insert into recipes 
(name, thumbnail, rations, steps) 
values 
('Pasta', 'pancakes.jpg', 3, '{ "Text": "1. Boil the pasta\n2. Add the sauce\n3. Serve"}');

insert into recipes 
(name, thumbnail, rations, steps) 
values 
('Pancakes', 'pancakes.jpg', 3, '{ "Text": "1. Mix the ingredients\n2. Cook the pancakes\n3. Serve"}');

insert into recipes 
(name, rations, steps) 
values 
('Lentejas', 3, '{ "Text": "1. Boil the pasta\n2. Add the sauce\n3. Serve"}');

insert into recipe_ingredients
(recipe_name, ingredient_name, quantity, unit)
values
('Pasta', 'flour', 200, 'grams');

insert into recipe_ingredients
(recipe_name, ingredient_name, quantity, unit)
values
('Pasta', 'Eggs', 5, 'units');

insert into recipe_ingredients
(recipe_name, ingredient_name, quantity, unit)
values
('Pasta', 'Milk', 500, 'grams');


