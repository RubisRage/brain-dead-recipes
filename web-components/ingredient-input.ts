

export default class IngredientInput extends HTMLInputElement {
    constructor() {
        super()
    }

    connectedCallback() {
        let template = document.getElementById('ingredients') as HTMLTemplateElement;
        this.appendChild(template.content);
    }

}

customElements.define('ingredient-input', IngredientInput, { extends: 'input' });
