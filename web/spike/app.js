/* [designed] Disposable UI state and synthetic roster; no original game rules. */
'use strict';
(() => {
  const $ = id => document.getElementById(id);
  const names = ['Jamie Ellis','Sam Reed','Alex Morgan','Jordan Blake','Chris Shaw','Taylor Brooks',
    'Robin Clarke','Lee Harper','Casey Ward','Drew Palmer','Charlie Lane','Morgan Bell',
    'Jesse Cole','Rowan Hill','Cameron Scott','Finley Price','Avery James','Bailey West',
    'Riley Hayes','Frankie Green','Ash Turner','Sidney King','Elliot Gray','Harley Fox'];
  const roster = names.map((name, i) => ({id:`p${String(i+1).padStart(2,'0')}`, name,
    role:i<2?'GK':i<9?'DEF':i<17?'MID':'FWD', age:19+i%14, skill:55+i%30,
    fitness:80+i%21, availability:i===7?'Injured (sample)':'Available'}));
  let state = {format:'onenil-ui-spike',version:1,club:'Northbridge Athletic',week:1,money:250000,
    instruction:'Balanced',slots:roster.map(p=>p.id)};
  let selected = state.slots[0];
  let pending = null;
  const position = index => index<11?`First XI ${index+1}`:index<16?`Substitute ${index-10}`:`Reserve ${index-15}`;
  const message = text => {$('status').textContent=text;};
  const player = id => roster.find(p=>p.id===id);
  window.spikeState = () => JSON.parse(JSON.stringify(state));
  function validate(value) {
    const keys=['format','version','club','week','money','instruction','slots'].sort();
    if (!value || Array.isArray(value) || JSON.stringify(Object.keys(value).sort())!==JSON.stringify(keys)
      || value.format!=='onenil-ui-spike' || value.version!==1 || value.club!=='Northbridge Athletic'
      || !Number.isSafeInteger(value.week) || value.week<1 || value.week>52
      || !Number.isSafeInteger(value.money) || value.money<0 || value.money>2147483647
      || !['Balanced','Attacking','Defensive'].includes(value.instruction)
      || !Array.isArray(value.slots) || value.slots.length!==roster.length
      || new Set(value.slots).size!==roster.length || value.slots.some(id=>!player(id))) {
      throw new Error('unsupported or invalid spike save');
    }
    return structuredClone(value);
  }
  function select(id, focus=false) {
    selected=id;
    for (const row of $('players').rows) {
      row.setAttribute('aria-selected',String(row.dataset.player===id));
      row.tabIndex=row.dataset.player===id?0:-1;
      if (focus && row.dataset.player===id) row.focus();
    }
    const p=player(id);
    $('player-name').textContent=p.name;
    $('player-position').textContent=position(state.slots.indexOf(id));
    $('player-details').replaceChildren();
    for (const [label,value] of [['Role',p.role],['Age',p.age],['Skill',p.skill],['Fitness',p.fitness],['Availability',p.availability]]) {
      const dt=document.createElement('dt'),dd=document.createElement('dd');
      dt.textContent=label;dd.textContent=value;$('player-details').append(dt,dd);
    }
  }
  function render() {
    $('players').replaceChildren();
    state.slots.forEach((id,index)=>{
      const p=player(id),row=document.createElement('tr');row.dataset.player=id;
      for (const value of [position(index),p.name,p.role,p.age,p.skill,`${p.fitness}%`,p.availability]) {
        const cell=document.createElement('td');cell.textContent=value;row.append(cell);
      }
      row.onclick=()=>select(id);
      row.onkeydown=event=>{
        if (['ArrowDown','ArrowUp','Home','End'].includes(event.key)) {
          event.preventDefault();
          const destination=event.key==='Home'?0:event.key==='End'?23:Math.max(0,Math.min(23,index+(event.key==='ArrowDown'?1:-1)));
          select(state.slots[destination],true);
        } else if(event.key==='Enter') {event.preventDefault();select(id);$('move').focus();}
      };
      $('players').append(row);
    });
    $('money').textContent=new Intl.NumberFormat('en-GB',{style:'currency',currency:'GBP',maximumFractionDigits:0}).format(state.money);
    $('week').textContent=state.week;$('instructions').value=state.instruction;
    select(selected);
  }
  $('move').onclick=()=>{
    pending=selected;$('move-summary').textContent=`Move ${player(pending).name} from ${position(state.slots.indexOf(pending))}.`;
    $('destination').replaceChildren();
    state.slots.forEach((id,index)=>{
      const option=new Option(`${position(index)} · ${player(id).name}`,String(index));
      option.disabled=id===pending;$('destination').append(option);
    });
    $('destination').value=String((state.slots.indexOf(pending)+1)%24);
    $('move-dialog').returnValue='cancel';$('move-dialog').showModal();$('destination').focus();
  };
  $('move-dialog').addEventListener('close',()=>{
    if ($('move-dialog').returnValue==='confirm' && pending) {
      const source=state.slots.indexOf(pending),dest=Number($('destination').value);
      [state.slots[source],state.slots[dest]]=[state.slots[dest],state.slots[source]];
      selected=pending;render();message(`Moved ${player(selected).name} to ${position(dest)}.`);
    } else message('Move cancelled · squad unchanged.');
    pending=null;$('move').focus();
  });
  $('instructions').onchange=event=>{state.instruction=event.target.value;message(`Sample instruction: ${state.instruction}. Save to keep it.`);};
  async function loadText(text) {
    if (text===null) return;
    try {
      if (text.length>65536) throw new Error('file exceeds 64 KiB');
      const next=validate(JSON.parse(text));
      state=next;render();message('Loaded spike save · squad and instructions restored.');
    } catch(error) {message(`Load refused · ${error.message}. Current state unchanged.`);}
  }
  $('save').onclick=async()=>{
    try {
      const text=JSON.stringify(state,null,2)+'\n';
      if (window.spikeFiles) {
        const result=await window.spikeFiles.save(text);
        message(result?'Saved spike state to file.':'Save cancelled · state unchanged.');
      } else {
        const url=URL.createObjectURL(new Blob([text],{type:'application/json'}));
        const link=document.createElement('a');link.href=url;link.download='onenil-ui-spike.json';link.click();
        setTimeout(()=>URL.revokeObjectURL(url),1000);message('Save download requested · keep the JSON file to reopen it.');
      }
    } catch(error) {message(`Save failed · ${error.message}.`);}
  };
  $('load').onclick=async()=>{
    if(window.spikeFiles) {
      try {const text=await window.spikeFiles.load();if(text===null) message('Load cancelled · state unchanged.');else await loadText(text);}
      catch(error) {message(`Load refused · ${error.message}. Current state unchanged.`);}
    } else $('load-input').click();
  };
  $('load-input').onchange=async event=>{
    const file=event.target.files[0];
    if(file) {
      if(file.size>65536) message('Load refused · file exceeds 64 KiB. Current state unchanged.');
      else await loadText(await file.text());
    }
    event.target.value='';
  };
  for(const name of ['squad','fixtures','match']) $(name+'-tab').onclick=()=>{
    for(const other of ['squad','fixtures','match']) {
      $(other+'-view').hidden=other!==name;$(other+'-tab').setAttribute('aria-pressed',String(other===name));
    }
  };
  function fixtures() {
    $('fixtures').replaceChildren();
    const page=Number($('fixture-page').value);$('fixtures').start=page*8+1;
    for(let i=page*8;i<page*8+8;i++) {
      const li=document.createElement('li');li.textContent=`Week ${i+1} · ${i%2?'Away':'Home'} · Sample opponent ${i+1} · Not played`;$('fixtures').append(li);
    }
  }
  $('fixture-page').onchange=fixtures;
  $('open-match').onclick=()=>{
    if(!$('match-container').firstChild) {
      const frame=document.createElement('iframe');frame.title='Existing native watched exhibition';frame.src='http://127.0.0.1:8041';$('match-container').append(frame);
    }
  };
  fixtures();render();
})();
