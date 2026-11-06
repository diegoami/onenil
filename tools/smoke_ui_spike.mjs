// [designed] Equivalent delivery acceptance; never an original-engine model.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import {execFileSync} from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath, pathToFileURL} from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(path.join(root, 'desktop/spike/package.json'));
const {chromium, _electron: electron} = require('playwright');
const out = path.resolve(process.argv[2] || path.join(root, 'records/T48'));
fs.mkdirSync(out, {recursive: true});
let n = 1;
while (fs.existsSync(path.join(out, `ui-r${String(n).padStart(4, '0')}.json`))) n++;
const stem = `ui-r${String(n).padStart(4, '0')}`;
const sha = data => createHash('sha256').update(data).digest('hex');
const git = (...args) => execFileSync('git', args, {cwd: root, encoding: 'utf8'}).trim();
const record = {schema: 'ui-delivery-spike/1', source: git('rev-parse', 'HEAD'),
  started: new Date().toISOString(), status: 'running', cases: [], screenshots: {},
  playwright: require('playwright/package.json').version, runtime: process.version,
  designed_data: true, campaign_engine_integration: false};
let browser, desktop;
async function image(page, name) {
  const target = path.join(out, `${stem}-${name}.png`);
  assert(!fs.existsSync(target));
  await page.screenshot({path: target, fullPage: true});
  record.screenshots[path.basename(target)] = sha(fs.readFileSync(target));
}
async function scenario(page, mode) {
  const failures = [];
  page.on('pageerror', e => failures.push(e.message));
  await page.getByRole('heading', {name: 'Northbridge Athletic'}).waitFor();
  assert.equal(await page.locator('tbody tr').count(), 24);
  // Arrow movement, detail identity and cancellation must preserve actual state.
  await page.locator('tbody tr').first().focus();
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('ArrowDown');
  assert.equal(await page.locator('#player-name').textContent(), 'Alex Morgan');
  const before = await page.evaluate(() => JSON.stringify(window.spikeState()));
  await page.keyboard.press('Enter');
  await page.getByRole('button', {name: 'Move player'}).click();
  await page.locator('#destination').selectOption('17');
  await page.keyboard.press('Escape');
  assert.equal(await page.evaluate(() => JSON.stringify(window.spikeState())), before);
  await page.getByRole('button', {name: 'Move player'}).click();
  await page.locator('#destination').selectOption('17');
  await page.getByRole('button', {name: 'Confirm move'}).click();
  assert.equal(await page.locator('#player-name').textContent(), 'Alex Morgan');
  assert.match(await page.locator('#player-position').textContent(), /Reserve 2/);
  // A pending transaction cannot be discarded by file operations.
  await page.getByRole('button', {name: 'Move player'}).click();
  assert(await page.locator('#move-dialog').isVisible());
  await page.getByRole('button', {name: 'Cancel move'}).click();
  await image(page, `${mode}-squad`);
  const saved = path.join(root, '.cache', `${stem}-${mode}-save.json`);
  if (mode === 'browser') {
    const download = page.waitForEvent('download');
    await page.getByRole('button', {name: 'Save file'}).click();
    await (await download).saveAs(saved);
  } else {
    await desktop.evaluate(({dialog}, target) => {
      dialog.showSaveDialog = async () => ({canceled: false, filePath: target});
    }, saved);
    await page.getByRole('button', {name: 'Save file'}).click();
    await page.getByRole('status').filter({hasText: 'Saved'}).waitFor();
  }
  assert(fs.existsSync(saved));
  const state = JSON.parse(fs.readFileSync(saved, 'utf8'));
  assert.equal(state.slots[17], 'p03');
  const load = async target => {
    if (mode === 'browser') await page.locator('#load-input').setInputFiles(target);
    else {
      await desktop.evaluate(({dialog}, file) => {
        dialog.showOpenDialog = async () => ({canceled: false, filePaths: [file]});
      }, target);
      await page.getByRole('button', {name: 'Load file'}).click();
    }
  };
  for (const [name, data] of [['version', {...state, version: 99}],
    ['duplicate', {...state, slots: state.slots.map(() => 'p03')}],
    ['malformed', '{'], ['html-name', {...state, club: '<img src=x>'}]]) {
    const bad = path.join(root, '.cache', `${stem}-${mode}-${name}.json`);
    fs.writeFileSync(bad, typeof data === 'string' ? data : JSON.stringify(data), {flag: 'wx'});
    const live = await page.evaluate(() => JSON.stringify(window.spikeState()));
    await load(bad);
    await page.getByRole('status').filter({hasText: 'Load refused'}).waitFor();
    assert.equal(await page.evaluate(() => JSON.stringify(window.spikeState())), live);
    record.cases.push(`${mode}: rejected ${name} atomically`);
  }
  // Real reload after a change, not a snapshot supplied to the page.
  await page.getByRole('button', {name: 'Move player'}).click();
  await page.locator('#destination').selectOption('2');
  await page.getByRole('button', {name: 'Confirm move'}).click();
  await load(saved);
  await page.getByRole('status').filter({hasText: 'Loaded'}).waitFor();
  assert.deepEqual(await page.evaluate(() => window.spikeState()), state);
  record.cases.push(`${mode}: keyboard/detail identity, cancel, swap, file round-trip`);
  assert.equal(await page.locator('iframe').count(), 0);
  assert.match(await page.locator('#exhibition-boundary').textContent(), /does not use this edited squad/);
  assert.deepEqual(failures, []);
  return {saved, state, load};
}
try {
  assert.equal(git('status', '--porcelain', '--', 'web/spike', 'desktop/spike', 'tools/smoke_ui_spike.mjs'), '',
    'commit spike producer sources before measurement');
  browser = await chromium.launch({headless: true,
    ...(process.env.CHROMIUM_PATH ? {executablePath: process.env.CHROMIUM_PATH} : {})});
  record.chromium = browser.version();
  let context = await browser.newContext({acceptDownloads: true, viewport: {width: 1440, height: 1000}, offline: true});
  let page = await context.newPage();
  await page.goto(pathToFileURL(path.join(root, 'web/spike/index.html')).href);
  const browserResult = await scenario(page, 'browser');
  await context.close();
  context = await browser.newContext({offline: true, viewport: {width: 390, height: 844}});
  page = await context.newPage();
  await page.goto(pathToFileURL(path.join(root, 'web/spike/index.html')).href);
  await page.locator('#load-input').setInputFiles(browserResult.saved);
  await page.getByRole('status').filter({hasText: 'Loaded'}).waitFor();
  assert.deepEqual(await page.evaluate(() => window.spikeState()), browserResult.state);
  assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
  await image(page, 'browser-mobile-offline-reopened');
  record.cases.push('browser: new offline context loads real exported file; mobile page has no viewport overflow');
  await browser.close();browser = undefined;

  const executable = path.join(root, '.cache/T48-packages/OneNilUISpike-linux-x64/OneNilUISpike');
  desktop = await electron.launch({executablePath: executable, args: ['--no-sandbox', `--user-data-dir=${path.join(root, '.cache', stem+'-desktop-profile')}`]});
  record.electron = await desktop.evaluate(() => process.versions);
  page = await desktop.firstWindow();
  const desktopResult = await scenario(page, 'desktop');
  assert.equal(await page.evaluate(() => typeof window.require), 'undefined');
  const packaged = await desktop.evaluate(({app}) => app.isPackaged);
  assert.equal(packaged, true);
  record.cases.push('desktop: packaged application; renderer has no Node require');
  await desktop.close();desktop = undefined;
  desktop = await electron.launch({executablePath: executable, args: ['--no-sandbox', `--user-data-dir=${path.join(root, '.cache', stem+'-desktop-profile')}`]});
  page = await desktop.firstWindow();
  await desktop.evaluate(({dialog}, file) => {dialog.showOpenDialog = async () => ({canceled: false, filePaths: [file]});}, desktopResult.saved);
  await page.getByRole('button', {name: 'Load file'}).click();
  await page.getByRole('status').filter({hasText: 'Loaded'}).waitFor();
  assert.deepEqual(await page.evaluate(() => window.spikeState()), desktopResult.state);
  await image(page, 'desktop-reopened');
  record.cases.push('desktop: fresh packaged process loads saved file');
  record.package_binary_sha256 = sha(fs.readFileSync(executable));
  record.package_asar_sha256 = sha(fs.readFileSync(path.join(path.dirname(executable), 'resources/app.asar')));
  record.status = 'passed';
} catch (error) {
  record.status = 'failed';record.error = String(error.stack || error);process.exitCode = 1;
} finally {
  if (browser) await browser.close();
  if (desktop) await desktop.close();
  record.finished = new Date().toISOString();
  fs.writeFileSync(path.join(out, stem+'.json'), JSON.stringify(record, null, 2)+'\n', {flag: 'wx'});
  console.log(JSON.stringify({record: path.join(out, stem+'.json'), status: record.status, error: record.error}));
}
