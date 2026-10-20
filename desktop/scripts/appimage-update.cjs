const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');

const updateInformation = 'gh-releases-zsync|MX-Goliath|numi|latest|Numi-*-linux-x86_64.AppImage.zsync';

module.exports = async ({ artifactPaths }) => {
  const files = [];
  for (const artifact of artifactPaths.filter(file => file.endsWith('.AppImage'))) {
    // Patch the runtime's reserved section in place. Running objcopy on the
    // whole AppImage would discard the SquashFS payload appended to the ELF.
    const sections = execFileSync('readelf', ['--section-headers', '--wide', '--', artifact], { encoding: 'utf8' });
    const section = sections.match(/\s\.upd_info\s+\S+\s+\S+\s+([0-9a-f]+)\s+([0-9a-f]+)/i);
    if (!section) throw new Error(`AppImage has no reserved update information section: ${artifact}`);
    const offset = parseInt(section[1], 16);
    const size = parseInt(section[2], 16);
    const information = Buffer.from(updateInformation + '\0');
    if (information.length > size) throw new Error('AppImage update information exceeds the reserved section');
    const descriptor = fs.openSync(artifact, 'r+');
    try {
      const buffer = Buffer.alloc(size);
      information.copy(buffer);
      fs.writeSync(descriptor, buffer, 0, size, offset);
    } finally {
      fs.closeSync(descriptor);
    }
    const zsync = artifact + '.zsync';
    execFileSync('zsyncmake', ['-u', path.basename(artifact), '-o', zsync, artifact], { stdio: 'inherit' });
    files.push(zsync);
    console.log(`AppImage update channel: ${updateInformation}`);
  }
  return files;
};
