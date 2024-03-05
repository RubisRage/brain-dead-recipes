

export default class IngredientInput extends HTMLElement {
    private shadow: ShadowRoot;

    private ingredients: HTMLInputElement[];

    private filterInput: HTMLInputElement;
    private ingredientsSelect: HTMLElement;
    private selectMenu: HTMLElement;
    private selectedText: HTMLElement;

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
            ingredientsContent.children as HTMLCollectionOf<HTMLInputElement>
        );

        this.ingredients
            .forEach((ingredient) => {
                const input = ingredient.querySelector('input') as HTMLInputElement;
                input.addEventListener('click', (e) => this.setIngredient(e))
            })

        this.selectedText = this.shadow
            .getElementById('selected') as HTMLElement;

        this.selectedText.innerText = this.ingredients[0]?.innerText.trim() ?? "No hay ingredientes todavía!";

        this.ingredientsSelect = this
            .shadow
            .getElementById('ingredients-select') as HTMLElement;

        this.ingredientsSelect.append(...this.ingredients);

        this.addVisibilityCallbacks()
    }

    private addVisibilityCallbacks() {
        this.selectedText.addEventListener('focus', () => {
            this.selectMenu.style.display = 'block';
        })

        this.selectedText.addEventListener('blur', (e: any) => {
            if (this.shadow.contains(e.relatedTarget as Node)) {
                this.selectMenu.style.display = 'none';
            }
        })

        this.selectMenu.addEventListener('focusout', (e: any) => {
            if (!this.shadow.contains(e.relatedTarget)) {
                this.selectMenu.style.display = 'none';
            }
        })
    }

    private setIngredient(e: any) {
        this.selectedText.innerText = e.target.value;

        for (const ingredient of this.ingredients) {
            const input = ingredient.querySelector('input') as HTMLInputElement;

            if (input.value !== e.target.value) {
                input.checked = false;
                input.blur();
            }
        }
    }

    private filterIngredients() {
        const filter = this.filterInput.value.toLowerCase();

        const filtered = this.ingredients.filter((ingredient) => {
            return ingredient.innerText.toLowerCase().includes(filter);
        })

        this.ingredientsSelect.replaceChildren(...filtered);
    }

}

customElements.define('ingredient-input', IngredientInput);
