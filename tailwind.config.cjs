/** @type {import('tailwindcss').Config}*/
const config = {
    content: ['./src/**/*.{html,js,svelte,ts}'],

    theme: {
        colors: {
            ink: {
                400: '#16141D',
                500: '#211C29',
                600: '#302D3A',
            },
            nebula: {
                400: '#542DC1',
                500: '#7637FF',
                600: '#8666FB',
            },
            neon: {
                500: '#FF63F7',
            },
            snow: {
                400: '#D2DFE3',
                500: '#F0FBFF',
                600: '#FCFEFF',
            },
        },
        extend: {},
    },

    plugins: [require('tailwind-scrollbar')({ nocompatible: true })],
};

module.exports = config;
