
export default function addIngredientInput() {
    const ingredientInput = document.createElement('ingredient-input');
    ingredientInput.setAttribute('name', 'ingredients[]');

    const ingredients = document.querySelector('#ingredients');
    ingredients.appendChild(ingredientInput);
    ingredients.scrollTop = ingredients.scrollHeight;
}

document.getElementById('add-ingredient').addEventListener('click', addIngredientInput);
