

export default class IngredientInput extends HTMLElement {
    private shadow: ShadowRoot;

    private ingredients: HTMLOptionElement[];

    private filterInput: HTMLInputElement;
    private ingredientsSelect: HTMLElement;
    private selectMenu: HTMLElement;
    private selected: HTMLElement;

    constructor() {
        super()
        this.shadow = this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        const template = document
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

        const ingredientsTemplate = document
            .getElementById('ingredients-template') as HTMLTemplateElement;

        const ingredientsContent = ingredientsTemplate
            .content.cloneNode(true) as HTMLElement;

        this.ingredients = Array.from(
            ingredientsContent.children as HTMLCollectionOf<HTMLOptionElement>
        );

        this.selected = this.shadow
            .getElementById('selected') as HTMLElement;

        this.selected.innerText = this.ingredients[0]?.innerText ?? "No hay ingredientes todavía!";

        this.ingredientsSelect = this
            .shadow
            .getElementById('ingredients-select') as HTMLElement;

        this.ingredientsSelect.append(...this.ingredients);

        this.addVisibilityCallbacks()
    }

    private addVisibilityCallbacks() {
        this.selected.addEventListener('focus', () => {
            this.selectMenu.style.display = 'block';
        })

        this.selected.addEventListener('blur', (e: FocusEvent) => {
            if (!this.shadow.contains(e.relatedTarget as Node)) {
                this.selectMenu.style.display = 'none';
            }
        })

        this.selectMenu.addEventListener('focusout', () => {
            this.selectMenu.style.display = 'none';
        })
    }

    filterIngredients() {
        const filter = this.filterInput.value.toLowerCase();

        const filtered = this.ingredients.filter((ingredient) => {
            return ingredient.innerText.toLowerCase().includes(filter);
        })

        this.ingredientsSelect.replaceChildren(...filtered);
    }

}

customElements.define('ingredient-input', IngredientInput);
