/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',
    content: [
        './templates/**/*.html',
        './web-components/**/*.ts',
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}
