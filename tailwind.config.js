/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',
    content: [
        './templates/**/*.html',
        './client/**/*.ts',
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}
