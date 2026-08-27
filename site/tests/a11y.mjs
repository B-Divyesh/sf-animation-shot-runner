import {AxeBuilder} from '@axe-core/playwright';
import {chromium} from 'playwright';
import {writeFile} from 'node:fs/promises';

const url = process.env.TEST_URL || 'http://127.0.0.1:4173/';
const browser = await chromium.launch({headless: true});
const context = await browser.newContext({viewport: {width: 390, height: 844}});
const page = await context.newPage();
await page.goto(url, {waitUntil: 'networkidle'});
const results = await new AxeBuilder({page}).analyze();
await writeFile('.factory/evidence/axe.json', JSON.stringify(results, null, 2));
const serious = results.violations.filter(item => ['serious', 'critical'].includes(item.impact));
console.log(`axe: ${results.passes.length} passes, ${results.violations.length} violations, ${serious.length} serious/critical`);
for (const violation of serious) console.error(`${violation.impact}: ${violation.id} — ${violation.help}`);
await browser.close();
if (serious.length) process.exit(1);
