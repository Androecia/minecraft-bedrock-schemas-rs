import * as fs from 'fs';
import * as path from 'path';

const sourcePath = 'src/behavior/entities/filters';
const outputFile = 'src/behavior/entities/filters/mod.rs';

fs.readdir(sourcePath, (err, files) => {
  if (err) {
    console.error(`Error reading directory ${sourcePath}: ${err}`);
    return;
  }

  const contents = files
    .filter((file) => file.endsWith('.rs'))
    .map((file) => {
      const fileName = file.replace('.rs', '');
      return `mod ${fileName};\npub use ${fileName}::*;\n`;
    })
    .join('');

  fs.writeFile(outputFile, contents, (err) => {
    if (err) {
      console.error(`Error writing file ${outputFile}: ${err}`);
    } else {
      console.log(`Created file ${outputFile}`);
    }
  });
});
