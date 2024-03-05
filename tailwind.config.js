/** @type {import('tailwindcss').Config} */
module.exports = {
    // content: ["./templates/**/*.html"],
    mode: 'jit',
    purge: [
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
