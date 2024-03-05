
export default function addIngredientInput() {
    const ingredientInput = document.createElement('ingredient-input');
    document.querySelector('#ingredients').insertBefore(ingredientInput, document.getElementById('add-ingredient-input'));
}

document.getElementById('add-ingredient').addEventListener('click', addIngredientInput);
