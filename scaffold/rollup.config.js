import resolve from 'rollup-plugin-node-resolve';
import babel from 'rollup-plugin-babel';
import postcss from 'rollup-plugin-postcss';
import postcssPresetEnv from 'postcss-preset-env';
import cssnano from 'cssnano';

export default {
    input: 'src/index.js',
    output: {
        file: 'dist/bundle.js',
        format: 'iife'
    },
    plugins: [
        resolve(),
        babel({
            exclude: 'node_modules/**',
            presets: [
                ['@babel/preset-env']
            ]
        }),
        postcss({
            extract: true,
            plugins: [
                postcssPresetEnv(),
                cssnano()
            ]
        })
    ]
}