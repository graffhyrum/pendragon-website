/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./templates/pages/*.html", "./src/**/*.rs"],
    theme: {
        fontFamily: {
            display: ['Roboto', 'sans-serif'],
            body: ['Roboto', 'sans-serif'],
        },
        extend: {},
    },
    plugins: [],
}

