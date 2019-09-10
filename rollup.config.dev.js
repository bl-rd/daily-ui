import config from './rollup.config';
import serve from 'rollup-plugin-serve';

config.plugins.push(serve());

export default config;