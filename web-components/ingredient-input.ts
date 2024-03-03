

export default class IngredientInput extends HTMLElement {
    private shadow: ShadowRoot;

    private ingredients: HTMLOptionElement[];

    private filterInput: HTMLInputElement;
    private ingredientsSelect: HTMLElement;
    private selectMenu: HTMLElement;
    private selected: HTMLElement;

    private isFocused: boolean = false;

    constructor() {
        super()
        this.shadow = this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        let template = document
            .getElementById('ingredient-input-template') as HTMLTemplateElement;

        this.shadow.appendChild(template.content.cloneNode(true));


        this.selectMenu = this
            .shadow
            .getElementById('menu') as HTMLElement;

        this.filterInput = this
            .shadow
            .getElementById('filter') as HTMLInputElement;

        this.filterInput.addEventListener(
            'input', () => this.filterIngredients()
        );

        this.ingredients = Array.from(
            (document
                .getElementById('ingredients-template') as HTMLTemplateElement)
                .content
                .children as HTMLCollectionOf<HTMLOptionElement>
        );

        this.selected = this.shadow
            .getElementById('selected') as HTMLElement;

        this.selected.innerText = this.ingredients[0]?.innerText ?? "No hay ingredientes todavía!";

        this.selected.addEventListener('click', () => {
            this.isFocused = !this.isFocused;

            if (this.isFocused) {
                this.selectMenu.style.display = 'block';
            } else {
                this.selectMenu.style.display = 'none';
            }
        })

        this.ingredientsSelect = this
            .shadow
            .getElementById('ingredients-select') as HTMLElement;

        this.ingredientsSelect.append(...this.ingredients);

    }

    filterIngredients() {
        let ingredients = this.ingredients;
        let filter = this.filterInput.value.toLowerCase();

        let filtered = ingredients.filter((ingredient) => {
            return ingredient.innerText.toLowerCase().includes(filter);
        })

        this.ingredientsSelect.innerHTML = '';
        this.ingredientsSelect.append(...filtered);
    }

}

customElements.define('ingredient-input', IngredientInput);
