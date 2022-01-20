const sh = require("shelljs");

sh.cd(__dirname);

// build command && clear folder after finish
const buildCmd = `cd ${process.argv.pop()} && cargo run && rm -rf target/ && cd ..`;

// Execute the build command
const { code } = sh.exec(buildCmd);

process.exit(code);
