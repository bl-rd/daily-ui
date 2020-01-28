import path from 'path';
import fs from 'fs-extra';
import chalk from 'chalk';

console.log('');

const number = parseInt(process.argv[2], 10);

go(number);

/**
 * 
 * @param {Number} index 
 * @param {String} [scaffoldDirectory = 'scaffold']
 */
async function go(index, scaffoldDirectory = 'scaffold') {    

    if (isNaN(number)) {
        logError('Please pass a valid number');
        process.exit();
    }

    const directory = path.resolve(path.resolve(), number.toString());
    const alreadyExists = await directoryExists(directory);

    if (alreadyExists) {
        logError('Directory already exists - choose a new number');
        process.exit();
    }
    
    logInformation('Attempting to copy files...');

    try {
        await fs.copy(path.resolve(path.resolve(), scaffoldDirectory), directory);
        logSuccess('Successfully created new scaffolded directory...');
    } catch {
        logError('Unable to move scaffolded files');
    }

    logInformation('Doing final adjustments...');

    logInformation('Updating package.json');

    try {
        const packageJson = await fs.readJSON(`${directory}/package.json`);    
        packageJson.name += `-${number}`;
        await fs.writeJSON(`${directory}/package.json`, packageJson, { spaces: 2 });
        logSuccess('Successfully updated package.json');
    } catch (e) {
        logError('Unable to update package.json');
        console.error(e);
        process.exit();
    }

    logInformation('Updating index.html file');
    // don't actually do anything yet!
    logSuccess('Successfully updated index.html file');

    console.log('\n🚀', ' Good to go! Go to', directory, 'and run `pnpm` to install dependencies');
}

/**
 * 
 * @param {*} path 
 */
async function directoryExists(path) {
    try {
        const result = await fs.ensureDir(path);
        return !!!result;
    } catch {
        return false;
    }
}

/**
 * 
 * @param {String} message 
 * @param {String} [type = 'Problem!']
 */
function logError(message, type = 'Problem!') {
    const error = chalk.bold.blue.bgRed;
    console.log('🙈', '', error(type), chalk.red(message));
}

/**
 * 
 * @param {String} message
 * @param {String} [type = 'Info:']
 */
function logInformation(message, type = 'Info:') {
    console.log('👉', '', message);
}

function logSuccess(message) {
    console.log('✨', '', chalk.green(message));
}