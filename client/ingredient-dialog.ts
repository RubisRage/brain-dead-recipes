
export function openDialog() {
    const container = document.querySelector("#ingredient-dialog");
    const dialog = document.querySelector("#ingredient-dialog form") as HTMLFormElement;
    const error = document.querySelector("#ingredient-dialog #error") as HTMLDivElement;

    error.innerText = "";
    dialog.reset();
    container.classList.remove("hidden");
}

document.querySelector?.("#ingredient-dialog").addEventListener("click", (e) => {
    if (e.target === e.currentTarget) {
        closeDialog();
    }
})

document.body.addEventListener("newIngredient", () => closeDialog());

function closeDialog() {
    const container = document.querySelector("#ingredient-dialog");
    container.classList.add("hidden");
}
