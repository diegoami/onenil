// [designed] Candidate desktop delivery only; no engine/campaign rules.
const {app, BrowserWindow, dialog, ipcMain} = require('electron');
const fs = require('node:fs/promises');
const path = require('node:path');
let window;
app.whenReady().then(() => {
  window = new BrowserWindow({width:1440,height:1000,minWidth:390,minHeight:600,
    backgroundColor:'#0b1420',autoHideMenuBar:true,
    webPreferences:{preload:path.join(__dirname,'preload.cjs'),nodeIntegration:false,
      contextIsolation:true,sandbox:true}});
  const ui=path.join(__dirname,'ui/index.html');
  window.loadFile(ui);
  window.webContents.setWindowOpenHandler(()=>({action:'deny'}));
  window.webContents.on('will-navigate',event=>event.preventDefault());
  const sender = event => {
    if(event.sender!==window.webContents || event.senderFrame!==window.webContents.mainFrame) throw new Error('main window only');
  };
  ipcMain.handle('spike:save',async(event,text)=>{
    sender(event);
    if(typeof text!=='string' || Buffer.byteLength(text)>65536) throw new Error('invalid save size');
    const result=await dialog.showSaveDialog(window,{defaultPath:'onenil-ui-spike.json',filters:[{name:'Spike save',extensions:['json']}]});
    if(result.canceled || !result.filePath)return false;
    await fs.writeFile(result.filePath,text,'utf8');return true;
  });
  ipcMain.handle('spike:load',async event=>{
    sender(event);
    const result=await dialog.showOpenDialog(window,{properties:['openFile'],filters:[{name:'Spike save',extensions:['json']}]});
    if(result.canceled || !result.filePaths.length)return null;
    const file=await fs.open(result.filePaths[0],'r');
    try {
      const stat=await file.stat();if(!stat.isFile() || stat.size>65536)throw new Error('invalid save file');
      return await file.readFile('utf8');
    } finally {await file.close();}
  });
});
app.on('window-all-closed',()=>app.quit());
