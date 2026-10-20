const { app, BrowserWindow, ipcMain, globalShortcut, session } = require('electron');
const path = require('node:path');
const settings = require('electron-settings');
const numiModule = require('./numi-module/index.node');
const isDev = process.argv.includes('--dev') || process.env.NODE_ENV === 'dev';
const isSmoke = process.argv.includes('--smoke-test');

let win;
class NumiModel { constructor() { this.cm = numiModule.numi_model_new(); } start() { return numiModule.numi_model_start.call(this.cm); } calc(text) { return numiModule.numi_model_calc.call(this.cm, text); } }
const numiModel = new NumiModel();

function createWindow() {
  const saved = settings.getSync('window') || {};
  win = new BrowserWindow({
    title: 'Numi',
    width: saved.width || 300,
    height: saved.height || 240,
    x: saved.x,
    y: saved.y,
    minWidth: 300,
    minHeight: 200,
    show: false,
    backgroundColor: '#212226',
    ...(process.platform === 'linux' ? { frame: false } : {}),
    ...(process.platform === 'darwin' ? { titleBarStyle: 'hiddenInset', trafficLightPosition: { x: 8, y: 8 } } : {}),
    webPreferences: { preload: path.join(__dirname, 'preload.js'), contextIsolation: true, sandbox: false, nodeIntegration: false }
  });
  win.setMenuBarVisibility(false);
  win.once('ready-to-show', () => win.show());
  win.on('close', () => settings.setSync('window', win.getBounds()));
  globalShortcut.register('CommandOrControl+Shift+I', () => win.webContents.openDevTools({ mode: 'detach' }));
  if (isDev) win.loadURL('http://127.0.0.1:3000'); else win.loadFile(path.join(__dirname, 'dist/index.html'));
}
ipcMain.handle('numiModelCalc', (_event, input) => numiModel.calc(input));
ipcMain.handle('invoke-rust', () => numiModule.hello());
ipcMain.handle('get-app-version', () => app.getVersion());
ipcMain.handle('install-update', () => false);
ipcMain.on('window-control', (_event, action) => {
  if (!win) return;
  if (action === 'minimize') win.minimize();
  if (action === 'maximize') win.isMaximized() ? win.unmaximize() : win.maximize();
  if (action === 'close') win.close();
});
async function smokeTest() {
  const timeout = setTimeout(() => { console.error('Smoke test timed out'); app.exit(1); }, 15000);
  try {
    if (win.webContents.isLoading()) await new Promise(resolve => win.webContents.once('did-finish-load', resolve));
    if (!win.isVisible()) await new Promise(resolve => win.once('show', resolve));
    const result = await win.webContents.executeJavaScript(`(async () => {
      const deadline = Date.now() + 10000;
      while (!document.querySelector('textarea') || !document.querySelector('.version')?.textContent.includes('${app.getVersion()}')) {
        if (Date.now() > deadline) throw new Error('Calculator interface did not load');
        await new Promise(resolve => setTimeout(resolve, 50));
      }
      const output = await window.electronAPI.numiModelCalc(JSON.stringify({ input: ['2 meters in cm', '1 Hz in s'], options: { precision: 2 } }));
      return JSON.parse(output).result.map(line => line.value);
    })()`);
    if (JSON.stringify(result) !== JSON.stringify(['200 cm', '1 s'])) throw new Error(`Unexpected calculator results: ${JSON.stringify(result)}`);
    if (!win.isVisible()) throw new Error('Calculator window is not visible');
    console.log(`Smoke test passed: Numi ${app.getVersion()}, renderer, native bridge, unit and wave conversions`);
    clearTimeout(timeout);
    app.exit(0);
  } catch (error) {
    console.error(error);
    app.exit(1);
  }
}

app.whenReady().then(async () => { if (process.argv.includes('--clear')) { await session.defaultSession.clearStorageData({ storages: ['localstorage'] }); settings.unsetSync('window'); } createWindow(); app.on('window-all-closed', () => app.quit()); void numiModel.start(); if (win) win.webContents.send('on-engine-start'); if (isSmoke) void smokeTest(); });
