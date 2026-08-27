import './style.css';

const SAMPLE = `{
  "version": 1,
  "project": "paper-courier",
  "output": "previews",
  "shots": [
    {"name":"sq010-arrival","source":"scenes/arrival.blend","fps":24,"colorspace":"sRGB","command":["blender","-b","{source}","-o","{frames}/frame_","-a"]},
    {"name":"sq020-door","source":"scenes/door.blend","fps":24,"colorspace":"sRGB","command":["blender","-b","{source}","-o","{frames}/frame_","-a"]},
    {"name":"sq030-crossing","source":"scenes/crossing.blend","fps":24,"colorspace":"sRGB","command":["blender","-b","{source}","-o","{frames}/frame_","-a"]},
    {"name":"sq040-turn","source":"scenes/turn.blend","fps":24,"colorspace":"sRGB","command":["blender","-b","{source}","-o","{frames}/frame_","-a"]},
    {"name":"sq050-exit","source":"scenes/exit.blend","fps":24,"colorspace":"sRGB","command":["blender","-b","{source}","-o","{frames}/frame_","-a"]}
  ]
}`;

document.querySelectorAll('[data-copy]').forEach(button => button.addEventListener('click', async () => {
  const original = button.textContent;
  try {
    await navigator.clipboard.writeText(button.dataset.copy);
    button.textContent = 'Copied';
  } catch {
    button.textContent = 'Select text';
  }
  window.setTimeout(() => { button.textContent = original; }, 1600);
}));

const editor = document.querySelector('#manifest');
const planButton = document.querySelector('#plan');
const output = document.querySelector('#plan-output');
const status = document.querySelector('#plan-status');
if (editor && planButton && output && status) {
  editor.value = SAMPLE;
  planButton.addEventListener('click', () => {
    status.textContent = 'CHECKING';
    output.className = 'loading';
    output.innerHTML = '<strong>Reading the proof…</strong>';
    window.setTimeout(() => {
      try {
        const data = JSON.parse(editor.value);
        const errors = validateManifest(data);
        if (errors.length) throw new Error(errors.join(' '));
        const commands = [...new Set(data.shots.map(shot => shot.command[0]))];
        status.textContent = 'READY';
        status.className = 'status success';
        output.className = 'planned';
        output.innerHTML = `<div class="plan-summary"><strong>${escapeHtml(data.shots.length)} shots ready</strong><span>${escapeHtml(data.project)} / ${escapeHtml(commands.join(', '))}</span></div><ol>${data.shots.map((shot, index) => `<li><span>${String(index + 1).padStart(2, '0')}</span><div><strong>${escapeHtml(shot.name)}</strong><small>${escapeHtml(shot.fps)} FPS · ${escapeHtml(shot.colorspace)} · ${escapeHtml(shot.source)}</small></div><b>HELD</b></li>`).join('')}</ol><p class="trust-note">Nothing ran. In the CLI, continue only after review:<br><code>shot-runner run shots.json --allow-command ${escapeHtml(commands[0])} --yes</code></p>`;
      } catch (error) {
        status.textContent = 'ERROR';
        status.className = 'status danger';
        output.className = 'error';
        output.innerHTML = `<strong>The manifest could not be planned.</strong><p>${escapeHtml(error.message)}</p><p>Correct the JSON or required fields, then validate again.</p>`;
      }
    }, 180);
  });
}

function validateManifest(data) {
  const errors = [];
  if (!data || data.version !== 1) errors.push('“version” must be 1.');
  if (typeof data?.project !== 'string' || !data.project.trim()) errors.push('Add a project name.');
  if (!Array.isArray(data?.shots) || !data.shots.length) errors.push('Add at least one shot.');
  else data.shots.forEach((shot, index) => {
    if (!/^[A-Za-z0-9_-]+$/.test(shot?.name || '')) errors.push(`Shot ${index + 1} needs a safe name.`);
    if (!shot?.source || typeof shot.source !== 'string') errors.push(`Shot ${index + 1} needs a source.`);
    if (!(shot?.fps > 0)) errors.push(`Shot ${index + 1} needs a positive FPS.`);
    if (!shot?.colorspace) errors.push(`Shot ${index + 1} needs a colorspace.`);
    if (!Array.isArray(shot?.command) || !shot.command.length || typeof shot.command[0] !== 'string') errors.push(`Shot ${index + 1} needs a tokenized command.`);
  });
  return errors;
}

function escapeHtml(value) {
  return String(value).replace(/[&<>'"]/g, character => ({'&':'&amp;','<':'&lt;','>':'&gt;',"'":'&#39;','"':'&quot;'}[character]));
}

const PRODUCT = 'animation-shot-runner';
const API = `https://api.sociobot.in/api/v1/products/${PRODUCT}`;
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
const locked = document.querySelector('#license-locked');
const unlocked = document.querySelector('#license-unlocked');
const licenseMessage = document.querySelector('#license-message');
const restoreForm = document.querySelector('#restore-form');
const restoreToggle = document.querySelector('.restore-toggle');

function storageGet(key) { try { return localStorage.getItem(key); } catch { return null; } }
function storageSet(key, value) { try { localStorage.setItem(key, value); return true; } catch { return false; } }
function storageRemove(key) { try { localStorage.removeItem(key); } catch {} }
function showUnlocked(active) { if (!locked || !unlocked) return; locked.hidden = active; unlocked.hidden = !active; }
function setLicenseMessage(message) { if (licenseMessage) licenseMessage.textContent = message; }

async function verifyLicense(token, announce = true) {
  if (announce) setLicenseMessage('Verifying your license…');
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token)}`, {headers: {'Accept':'application/json'}});
    if (!response.ok) throw new Error(`verification returned ${response.status}`);
    const verdict = await response.json();
    storageSet(VERDICT_KEY, JSON.stringify({...verdict, checked_at: Date.now()}));
    showUnlocked(Boolean(verdict.valid));
    setLicenseMessage(verdict.valid ? 'License verified on this browser.' : 'License no longer active. You can restore another token or buy a license.');
    return Boolean(verdict.valid);
  } catch {
    setLicenseMessage(navigator.onLine ? 'License verification is temporarily unavailable. Your cached access is unchanged.' : 'You are offline. Your cached access is unchanged; verification will retry later.');
    return false;
  }
}

if (locked && unlocked) {
  const query = new URLSearchParams(location.search);
  const incoming = query.get('license');
  if (incoming) {
    storageSet(LICENSE_KEY, incoming);
    query.delete('license');
    history.replaceState({}, '', `${location.pathname}${query.size ? `?${query}` : ''}${location.hash}`);
  }
  const token = incoming || storageGet(LICENSE_KEY);
  let cached = null;
  try { cached = JSON.parse(storageGet(VERDICT_KEY) || 'null'); } catch {}
  if (cached?.valid) showUnlocked(true);
  if (token && (!cached?.checked_at || Date.now() - cached.checked_at > 86_400_000 || incoming)) verifyLicense(token, !cached?.valid);

  restoreToggle?.addEventListener('click', () => {
    const open = restoreForm.hidden;
    restoreForm.hidden = !open;
    restoreToggle.setAttribute('aria-expanded', String(open));
    if (open) document.querySelector('#license-token')?.focus();
  });
  restoreForm?.addEventListener('submit', async event => {
    event.preventDefault();
    const tokenValue = new FormData(restoreForm).get('license')?.toString().trim();
    if (!tokenValue) return;
    storageSet(LICENSE_KEY, tokenValue);
    await verifyLicense(tokenValue);
  });
  document.querySelector('#forget-license')?.addEventListener('click', () => {
    storageRemove(LICENSE_KEY); storageRemove(VERDICT_KEY); showUnlocked(false); setLicenseMessage('License removed from this browser.');
  });
}

const network = document.querySelector('#network');
function updateNetwork() { if (!network) return; network.hidden = navigator.onLine; }
window.addEventListener('online', updateNetwork);
window.addEventListener('offline', updateNetwork);
updateNetwork();

if ('serviceWorker' in navigator && location.protocol !== 'file:') window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => {}));
