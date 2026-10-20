const path = require('node:path');
module.exports = {
  packagerConfig: {
    name: 'Numi', executableName: 'numi', appBundleId: 'local.numi.desktop', icon: path.join(__dirname, 'assets/icons/icon'),
    asar: { unpack: '**/*.node' },
    ignore: file => {
      if (!file) return false;
      const relative = file.replaceAll('\\', '/');
      return !/^\/(dist|assets|node_modules)(\/|$)/.test(relative) && !/^\/(main\.js|preload\.js|package\.json)$/.test(relative) && !/^\/numi-module(\/index\.node)?$/.test(relative);
    },
  },
  makers: [{ name: '@electron-forge/maker-zip', platforms: ['linux', 'win32', 'darwin'] }],
};
