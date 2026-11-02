'use strict';
const $ = id => document.getElementById(id);
let state, running = false, busy = false, timer, rendered = 0;
const phases = {'ready':'Ready for kickoff','first-half':'First half','half-time':'Half-time','second-half':'Second half','full-time':'Full-time'};
const minute = n => n > 90 ? `90+${n-90}′` : `${n}′`;
const name = id => state.meta.players[String(id)] || `Player ${id}`;
function stop() { running = false; clearTimeout(timer); }
function render() {
  const v = state.view;
  $('home').textContent = state.meta.clubs[String(v.clubs[0])];
  $('away').textContent = state.meta.clubs[String(v.clubs[1])];
  $('score').textContent = v.score_side.join(' : ');
  $('clock').textContent = minute(v.clock.elapsed);
  $('phase').textContent = `${phases[state.phase]}${running ? ' · Live' : ''}`;
  $('status-dot').classList.toggle('active', running);
  $('primary').textContent = state.phase === 'ready' ? 'Kick off' : state.phase === 'half-time' ? 'Continue second half' : state.phase === 'full-time' ? 'Match finished' : running ? 'Pause' : 'Resume';
  $('primary').disabled = busy || state.phase === 'full-time';
  $('restart').disabled = busy;
  $('live-label').textContent = running ? 'LIVE' : state.phase === 'full-time' ? 'FULL-TIME' : state.phase === 'half-time' ? 'HALF-TIME' : 'PAUSED';
  const svg = $('players'); svg.replaceChildren();
  const ns = 'http://www.w3.org/2000/svg';
  for (const p of v.formation) {
    const side = v.clubs.indexOf(p.club);
    const g = document.createElementNS(ns, 'g');
    // Designed screen projection only. Raw native coordinates remain in the API.
    const x = 500 + p.y * 10.5 * (side === 0 ? 1 : -1), y = 300 + p.x * 23;
    const dot = document.createElementNS(ns, 'circle');
    dot.setAttribute('cx', x); dot.setAttribute('cy', y); dot.setAttribute('r', p.focus ? 10 : 7);
    dot.setAttribute('fill', side === 0 ? '#75baff' : '#ed9b91');
    dot.setAttribute('stroke', p.focus ? '#f5d477' : '#fff'); dot.setAttribute('stroke-width', p.focus ? 4 : 1.5);
    dot.dataset.player = p.player;
    const title = document.createElementNS(ns, 'title'); title.textContent = `${name(p.player)}${p.focus ? ' · match focus' : ''}`;
    g.append(title, dot); svg.append(g);
  }
  const lists = $('lineups'); lists.replaceChildren();
  v.clubs.forEach((club, side) => {
    const box = document.createElement('div');
    const heading = document.createElement('h3'); heading.textContent = state.meta.clubs[String(club)]; box.append(heading);
    for (const p of v.formation.filter(p => p.club === club)) {
      const item = document.createElement('span'); item.className = `player-name side-${side}${p.focus ? ' focus' : ''}`;
      item.textContent = name(p.player); box.append(item);
    }
    lists.append(box);
  });
  const log = $('commentary');
  if (!state.events.length) { log.innerHTML = '<p class="empty">The teams are ready.<br>Kick off to follow the match.</p>'; rendered = 0; }
  else {
    if (!rendered) log.replaceChildren();
    const nearEnd = log.scrollHeight - log.scrollTop - log.clientHeight < 70;
    for (const event of state.events.slice(rendered)) {
      const row = document.createElement('article'); row.className = `event ${event.kind}`; row.dataset.kind = event.kind;
      const time = document.createElement('time'); time.textContent = minute(event.minute);
      const text = document.createElement('p'); text.textContent = event.text;
      row.append(time, text); log.append(row);
    }
    rendered = state.events.length;
    if (nearEnd) log.scrollTop = log.scrollHeight;
  }
}
async function request(action) {
  const response = await fetch(action ? `/api/${action}` : '/api/state', action ? {method:'POST', headers:{'X-Onenil-Action':'1'}} : {});
  const data = await response.json();
  if (!response.ok) throw new Error(data.error || 'Match request failed');
  state = data;
}
async function action(kind) {
  if (busy) return;
  busy = true; render();
  try { await request(kind); $('error').hidden = true; }
  catch (error) { stop(); $('error').textContent = error.message; $('error').hidden = false; }
  finally { busy = false; render(); }
}
async function tick() {
  if (!running) return;
  await action('step');
  if (['half-time','full-time'].includes(state.phase)) stop();
  render();
  if (running) timer = setTimeout(tick, Number($('speed').value));
}
$('primary').addEventListener('click', async () => {
  if (busy) return;
  if (running) { stop(); render(); return; }
  if (state.phase === 'ready') await action('start');
  else if (state.phase === 'half-time') await action('continue');
  if (!$('error').hidden || !['first-half','second-half'].includes(state.phase)) return;
  running = true; render(); timer = setTimeout(tick, Number($('speed').value));
});
$('restart').addEventListener('click', async () => { stop(); rendered = 0; await action('restart'); });
request().then(render).catch(error => { $('error').textContent = `Cannot start native engine: ${error.message}`; $('error').hidden = false; $('primary').disabled = true; });
