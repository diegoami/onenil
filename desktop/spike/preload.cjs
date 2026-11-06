const {contextBridge,ipcRenderer}=require('electron');
contextBridge.exposeInMainWorld('spikeFiles',{
  save:text=>ipcRenderer.invoke('spike:save',text),
  load:()=>ipcRenderer.invoke('spike:load')
});
