
function addIngredientInput() {
    const ingredientInput = document.createElement('ingredient-input');
    document.querySelector('#ingredients').insertBefore(ingredientInput, document.getElementById('add-ingredient-input'));
}
