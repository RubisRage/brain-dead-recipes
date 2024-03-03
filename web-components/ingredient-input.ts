

export default class IngredientInput extends HTMLInputElement {
    constructor() {
        super()
    }

    connectedCallback() {
        console.log("Im aliveeee")
    }

}

customElements.define('ingredient-input', IngredientInput, { extends: 'input' });
