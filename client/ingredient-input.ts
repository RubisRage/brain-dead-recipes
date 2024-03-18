import { openDialog } from './ingredient-dialog';

export default class IngredientInput extends HTMLElement {
    static formAssociated = true;

    private shadow: ShadowRoot;
    private internals: ElementInternals;

    private ingredients: HTMLElement[];

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

        this.ingredientsSelect = this.shadow.querySelector('#ingredients-select')

        this.selectedText = this.shadow.querySelector('#selected')

        this.loadIngredients();

        this.selectedText.innerText = this
            .ingredients[0]
            ?.innerText.trim() ?? "No hay ingredientes todavía!"

        this.markSelected()

        this.shadow
            .querySelector('#create-ingredient')
            .addEventListener('click', () => {
                this.hideMenu()
                openDialog()
            })

        this.addVisibilityCallbacks()
    }

    private addVisibilityCallbacks() {
        this.selectedText.addEventListener('focus', () => {
            this.selectMenu.classList.remove('hidden');
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
        this.selectMenu.classList.add('hidden');
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
        return JSON.stringify({
            selected: this.selectedText.innerText,
            quantity: parseInt(this.quantityInput.value),
            unit: this.quantityUnits.value
        })
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

    loadIngredients() {
        const ingredientsContent = document
            .querySelector('#ingredients-template')
            .cloneNode(true) as HTMLDivElement;

        this.ingredients = Array.from(
            ingredientsContent.children as HTMLCollectionOf<HTMLElement>
        );

        this.ingredients
            .forEach((ingredient) => {
                ingredient
                    .querySelector<HTMLButtonElement>('button')
                    .addEventListener('click', (e) => this.setIngredient(e))
            })

        this.ingredientsSelect.replaceChildren(...this.ingredients);
    }

    markSelected() {
        const selected = this.selectedText.innerHTML.trim();

        for (const ingredient of this.ingredients) {
            const ingredientButton = ingredient.querySelector('button') as HTMLButtonElement;
            if (ingredientButton.innerText.trim() === selected) {
                ingredientButton.classList.add('selected');
            }
        }
    }
}

customElements.define('ingredient-input', IngredientInput);

function updateIngredientInputs() {
    const ingredientInputs = document.querySelectorAll<IngredientInput>('ingredient-input');

    for (const ingredientInput of ingredientInputs) {
        ingredientInput.loadIngredients();
        ingredientInput.markSelected();
    }
}

(window as any).updateIngredientInputs = updateIngredientInputs;
