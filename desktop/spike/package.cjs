// Stage shared UI sources without committing duplicate files or a runtime binary.
const fs=require('node:fs');
const path=require('node:path');
const {packager}=require('@electron/packager');
const root=path.resolve(__dirname,'../..');
const stage=path.join(root,'.cache/T48-stage');
fs.mkdirSync(stage,{recursive:true});
for(const file of ['main.cjs','preload.cjs'])fs.copyFileSync(path.join(__dirname,file),path.join(stage,file));
fs.cpSync(path.join(root,'web/spike'),path.join(stage,'ui'),{recursive:true});
fs.writeFileSync(path.join(stage,'package.json'),JSON.stringify({name:'onenil-ui-spike',version:'0.0.1',main:'main.cjs'}));
packager({dir:stage,name:'OneNilUISpike',platform:'linux',arch:'x64',
  electronVersion:require('electron/package.json').version,
  out:path.join(root,'.cache/T48-packages'),overwrite:true,asar:true,
  download:{cacheRoot:path.join(root,'.cache/T48-electron-download')}})
  .then(paths=>console.log(paths.join('\n'))).catch(error=>{console.error(error);process.exitCode=1;});
