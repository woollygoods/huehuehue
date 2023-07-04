import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

const config = {
    preprocess: [
        preprocess({
            postcss: true,
        }),
    ],
    kit: {
        adapter: adapter(),
    },
};

export default config;
