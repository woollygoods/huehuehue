/**
 * Colors link: https://coolors.co/16141d-1d1a23-2f2d34-f0fbff-542dc1-6f00ff-8666fb-ff63f7
 */

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
                300: '#B7C4C8',
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
