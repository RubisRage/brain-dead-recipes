

export default class IngredientInput extends HTMLElement {
    static formAssociated = true;

    private shadow: ShadowRoot;
    private internals: ElementInternals;

    private ingredients: HTMLInputElement[];

    private filterInput: HTMLInputElement;
    private quantityInput: HTMLInputElement;
    private quantityUnits: HTMLInputElement;
    private ingredientsSelect: HTMLElement;
    private selectMenu: HTMLElement;
    private selectedText: HTMLElement;

    constructor() {
        super()
        this.shadow = this.attachShadow({ mode: 'open' });
        this.internals = this.attachInternals();
    }

    connectedCallback() {
        const template = document
            .getElementById('ingredient-input-template') as HTMLTemplateElement;

        this.shadow.appendChild(template.content.cloneNode(true));

        this.shadow.querySelector('#delete').addEventListener('click', () => {
            this.deleteSelf();
        })

        this.selectMenu = this.shadow.querySelector('#menu')

        this.quantityInput = this.shadow.querySelector('#ingredient-quantity')
        this.quantityUnits = this.shadow.querySelector('#ingredient-units')

        this.filterInput = this.shadow.querySelector('#filter')
        this.filterInput.addEventListener(
            'input', () => this.filterIngredients()
        );

        const ingredientsContent = document
            .querySelector<HTMLTemplateElement>('#ingredients-template')
            .content
            .cloneNode(true) as HTMLElement;

        this.ingredients = Array.from(
            ingredientsContent.children as HTMLCollectionOf<HTMLInputElement>
        );

        this.ingredients
            .forEach((ingredient) => {
                ingredient
                    .querySelector<HTMLButtonElement>('button')
                    .addEventListener('click', (e) => this.setIngredient(e))
            })

        this.selectedText = this.shadow.querySelector('#selected')

        this.selectedText.innerText = this
            .ingredients[0]
            ?.innerText.trim() ?? "No hay ingredientes todavía!"

        this.ingredientsSelect = this.shadow.querySelector('#ingredients-select')
        this.ingredientsSelect.append(...this.ingredients);

        this.addVisibilityCallbacks()
    }

    private addVisibilityCallbacks() {
        this.selectedText.addEventListener('focus', () => {
            this.selectMenu.style.display = 'block';
        })

        this.selectedText.addEventListener('blur', (e: any) => {
            if (!this.shadow.contains(e.relatedTarget as Node)) {
                this.hideMenu();
            }
        })

        this.selectMenu.addEventListener('focusout', (e: any) => {
            if (!this.shadow.contains(e.relatedTarget)) {
                this.hideMenu();
            }
        })
    }

    private hideMenu() {
        this.selectMenu.style.display = 'none';
        this.filterInput.value = "";
        this.ingredientsSelect.replaceChildren(...this.ingredients);
    }

    private setIngredient(e: any) {
        const selectedClasses = ["selected"];

        this.selectedText.innerText = e.originalTarget.innerText;

        for (const ingredient of this.ingredients) {
            const ingredientButton = ingredient.querySelector('button') as HTMLButtonElement;
            ingredientButton.classList.remove(...selectedClasses);
        }

        e.originalTarget.classList.add(...selectedClasses);
    }

    private filterIngredients() {
        const filter = this.filterInput.value.toLowerCase();

        const filtered = this.ingredients.filter((ingredient) => {
            return ingredient.innerText.toLowerCase().includes(filter);
        })

        this.ingredientsSelect.replaceChildren(...filtered);
    }

    private deleteSelf() {
        this.remove();
    }

    get value() {
        return `${this.selectedText.innerText},${this.quantityInput.value},${this.quantityUnits.value}`;
        //return this.selectedText.innerText;
    }

    set value(value) {
        this.selectedText.innerText = value;
        this.internals.setFormValue(value);
    }

    get form() {
        return this.internals.form;
    }

    get name() {
        return this.getAttribute('name');
    }

    get type() {
        return this.localName;
    }
}

customElements.define('ingredient-input', IngredientInput);
