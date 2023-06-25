/** @type {import('tailwindcss').Config}*/
const config = {
    content: ['./src/**/*.{html,js,svelte,ts}'],

    theme: {
        colors: {
            ink: {
                400: '#16141D',
                500: '#1D1A23',
                600: '#2F2D34',
            },
            nebula: {
                400: '#542DC1',
                500: '#6F00FF',
                600: '#8666FB',
            },
            neon: {
                500: '#FF63F7',
            },
            snow: {
                500: '#F0FBFF',
            },
        },
        extend: {},
    },

    plugins: [require('tailwind-scrollbar')({ nocompatible: true })],
};

module.exports = config;
