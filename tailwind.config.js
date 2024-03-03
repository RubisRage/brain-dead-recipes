/** @type {import('tailwindcss').Config} */
module.exports = {
    // content: ["./templates/**/*.html"],
    mode: 'jit',
    purge: [
        './templates/**/*.html',
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}
