'use strict';
const $ = id => document.getElementById(id);
let state, running = false, busy = false, timer, preparationTimer, rendered = 0, requestError = '', runGeneration = 0;
const phases = {preparing:'Preparing match…', ready:'Ready for kickoff', 'first-half':'First half', 'half-time':'Half-time', 'second-half':'Second half', 'full-time':'Full-time', error:'Match preparation failed'};
const minute = n => n > 90 ? `90+${n-90}′` : `${n}′`;
function playerName(id) {
  const value = state.meta.players[String(id)];
  if (!value) throw new Error(`Missing native player identity ${id}`);
  return value;
}
function stop() { running = false; runGeneration += 1; clearTimeout(timer); }
function point(p) {
  // [designed] Shared native coordinates for both teams and ball;no extra mirroring.
  return {x:500 + p.y * 12, y:300 + p.x * 29};
}
function render() {
  const v = state.view, preparing = state.phase === 'preparing';
  const dirty = $('lineup').value !== state.choices.lineup || $('seed').value !== state.choices.seed;
  const canChoose = !busy && ['ready','full-time','error'].includes(state.phase);
  $('lineup').disabled = $('seed').disabled = !canChoose;
  $('configure').disabled = !canChoose || (!dirty && state.phase !== 'error');
  $('selected').textContent = preparing ? 'Preparing your match…' : dirty ? 'Prepare to apply these choices.' : state.selection ? `${playerName(state.selection.starter)} starts · Seed ${state.selection.seed}` : 'Choose a lineup and seed.';
  $('home').textContent = state.meta.clubs[String(state.clubs[0])];
  $('away').textContent = state.meta.clubs[String(state.clubs[1])];
  $('score').textContent = v ? v.score.join(' : ') : '—';
  $('clock').textContent = v ? minute(v.clock) : '—';
  $('phase').textContent = `${phases[state.phase]}${running ? ' · Live' : ''}`;
  $('status-dot').classList.toggle('active', running);
  $('primary').textContent = preparing ? 'Preparing…' : state.phase === 'error' ? 'Preparation failed' : state.phase === 'ready' ? 'Kick off' : state.phase === 'half-time' ? 'Continue second half' : state.phase === 'full-time' ? 'Match finished' : running ? 'Pause' : busy ? 'Pausing…' : 'Resume';
  $('primary').disabled = (busy && !running) || preparing || dirty || ['full-time','error'].includes(state.phase);
  $('restart').disabled = busy || preparing;
  $('live-label').textContent = preparing ? 'PREPARING' : running ? 'LIVE' : state.phase === 'ready' ? 'READY' : state.phase === 'full-time' ? 'FULL-TIME' : state.phase === 'half-time' ? 'HALF-TIME' : 'PAUSED';
  $('pitch-note').textContent = state.phase === 'full-time' ? 'Full-time. The players have left the pitch.' : 'Player and ball positions follow the native match. Gold ring marks the current actor.';
  const problem = state.error || requestError;
  $('error').hidden = !problem;
  $('error').textContent = problem || '';
  const svg = $('players'); svg.replaceChildren();
  const ns = 'http://www.w3.org/2000/svg';
  for (const p of v ? v.players : []) {
    const side = state.clubs.indexOf(p.club), at = point(p);
    const group = document.createElementNS(ns, 'g');
    const dot = document.createElementNS(ns, 'circle');
    dot.setAttribute('cx', at.x); dot.setAttribute('cy', at.y); dot.setAttribute('r', p.focus ? 9 : 7);
    dot.setAttribute('fill', side === 0 ? '#75baff' : '#ed9b91');
    dot.setAttribute('stroke', p.focus ? '#f5d477' : '#fff'); dot.setAttribute('stroke-width', p.focus ? 3 : 1.5);
    dot.dataset.player = p.player; dot.dataset.scene = p.scene;
    const title = document.createElementNS(ns, 'title'); title.textContent = playerName(p.player);
    const label = document.createElementNS(ns, 'text');
    label.setAttribute('x', at.x); label.setAttribute('y', at.y - 13);
    label.setAttribute('text-anchor', 'middle'); label.setAttribute('fill', '#f1f7f4'); label.setAttribute('font-size', '11');
    label.textContent = playerName(p.player);
    group.append(title, dot, label); svg.append(group);
  }
  $('ball').style.display = v ? '' : 'none';
  if (v) {
    const ball = point(v.ball);
    $('ball').setAttribute('cx', ball.x); $('ball').setAttribute('cy', ball.y);
  }
  const lists = $('lineups'); lists.replaceChildren();
  for (const [side, club] of state.clubs.entries()) {
    const box = document.createElement('div');
    const heading = document.createElement('h3'); heading.textContent = state.meta.clubs[String(club)]; box.append(heading);
    for (const p of v ? v.players.filter(p => p.club === club) : []) {
      const item = document.createElement('span'); item.className = `player-name side-${side}${p.focus ? ' focus' : ''}`;
      item.textContent = playerName(p.player); box.append(item);
    }
    lists.append(box);
  }
  const log = $('commentary');
  if (!state.events.length) {
    log.innerHTML = preparing ? '<p class="empty">Preparing the selected match.<br>The pitch and commentary will be ready shortly.</p>' : '<p class="empty">The teams are ready.<br>Kick off to follow every action.</p>';
    rendered = 0;
  } else {
    if (!rendered) log.replaceChildren();
    const nearEnd = log.scrollHeight - log.scrollTop - log.clientHeight < 90;
    for (const event of state.events.slice(rendered)) {
      const row = document.createElement('article'); row.className = `event ${event.kind}`; row.dataset.kind = event.kind; row.dataset.event = event.id;
      const time = document.createElement('time'); time.textContent = minute(event.minute);
      const text = document.createElement('p'); text.textContent = event.text;
      row.append(time, text); log.append(row);
    }
    rendered = state.events.length;
    if (nearEnd) log.scrollTop = log.scrollHeight;
  }
  clearTimeout(preparationTimer);
  if (preparing) preparationTimer = setTimeout(async () => {
    try { await request(); render(); }
    catch (error) { requestError = error.message; render(); }
  }, 250);
}
async function request(action, body) {
  const options = action ? {method:'POST', headers:{'X-Onenil-Action':'1'}} : {};
  if (body) { options.headers['Content-Type'] = 'application/json'; options.body = JSON.stringify(body); }
  const response = await fetch(action ? `/api/${action}` : '/api/state', options);
  const data = await response.json();
  if (!response.ok) throw new Error(data.error || 'Match request failed');
  state = data;
  requestError = '';
  $('lineup').value = state.choices.lineup;
  $('seed').value = state.choices.seed;
}
async function action(kind, body) {
  if (busy) return;
  busy = true; render();
  try { await request(kind, body); requestError = ''; }
  catch (error) { stop(); requestError = error.message; }
  finally { busy = false; render(); }
}
async function tick(generation) {
  if (!running || generation !== runGeneration) return;
  await action('step');
  if (generation !== runGeneration) return;
  if (['half-time','full-time','error'].includes(state.phase)) stop();
  render();
  if (running) timer = setTimeout(() => tick(generation), Number($('speed').value));
}
$('primary').addEventListener('click', async () => {
  if (running) { stop(); render(); return; }
  if (busy) return;
  if (state.phase === 'ready') await action('start');
  else if (state.phase === 'half-time') await action('continue');
  if (['first-half','second-half'].includes(state.phase)) { running = true; runGeneration += 1; render(); tick(runGeneration); }
});
$('restart').addEventListener('click', async () => { stop(); await action('restart'); });
$('lineup').addEventListener('change', render);
$('seed').addEventListener('change', render);
$('configure').addEventListener('click', async () => {
  stop();
  await action('configure', {lineup:$('lineup').value, seed:$('seed').value});
});
request().then(render).catch(error => { $('error').textContent = error.message; $('error').hidden = false; });
