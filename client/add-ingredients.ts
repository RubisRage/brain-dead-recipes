
export default function addIngredientInput() {
    const ingredientInput = document.createElement('ingredient-input');
    ingredientInput.setAttribute('name', 'ingredients[]');
    document.querySelector('#ingredients').appendChild(ingredientInput);
}

document.getElementById('add-ingredient').addEventListener('click', addIngredientInput);
