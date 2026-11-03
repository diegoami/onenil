#!/usr/bin/env python3
"""[designed] Compile checked listing structure into native Rust basic blocks;no runtime decoding."""
import argparse,hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
REGS={'ax':0,'cx':1,'dx':2,'bx':3,'sp':4,'bp':5,'si':6,'di':7,'es':8,'cs':9,'ss':10,'ds':11}
BYTES={a+b:(i,shift) for a,i in [('a',0),('c',1),('d',2),('b',3)] for b,shift in [('l',0),('h',8)]}
def listing(root):
    code={};files={}
    for p in sorted((root/'static/1-0-exe/code').glob('seg_*.asm')):
        seg=int(p.name.split('_')[1],16);files[str(p.relative_to(root))]=hashlib.sha256(p.read_bytes()).hexdigest()
        for line in p.read_text().splitlines():
            m=re.match(r'^([0-9A-F]{4})\s+((?:[0-9A-F]{2} )*[0-9A-F]{2})\s+([a-z]\w*)\s*(.*)',line)
            if m:
                ip=int(m[1],16);args=m[4].split(';')[0].strip();code[seg,ip]=(m[3],args,ip+len(m[2].split()),str(p.relative_to(root)))
    return code,files

def target(seg,args):
    if args.startswith('far '):
        a,b=args[4:].split(':');return(int(a,16),int(b,16)),True
    if not args.startswith('0x'):raise ValueError('indirect control transfer:'+args)
    return(seg,int(args,16)),False

def graph(code,roots):
    result={};todo=list(roots)
    while todo:
        root=todo.pop()
        if root in result:continue
        seen={};pending=[root[1]]
        while pending:
            ip=pending.pop()
            if ip in seen:continue
            op,args,nxt,path=code[root[0],ip];seen[ip]=(op,args,nxt,path)
            if op in ('ret','retf'):continue
            if op=='call':
                dest,_=target(root[0],args);todo.append(dest)
            if op=='jmp' or (op.startswith('j') and op!='jmp'):
                dest,_=target(root[0],args);assert dest[0]==root[0];pending.append(dest[1])
                if op=='jmp':continue
            pending.append(nxt)
        result[root]=seen
    return result

def ea(operand):
    inner=operand[operand.index('[')+1:operand.index(']')];parts=re.findall(r'([+-]?)\s*(0x[0-9a-f]+|[a-z]+|\d+)',inner)
    out='0u16'
    for sign,value in parts:
        v=f'c.r[{REGS[value]}]' if value in REGS else f'{int(value,0)&65535}u16'
        out=f'{out}.wrapping_{"sub" if sign=="-" else "add"}({v})'
    seg=re.search(r'(es|ss|ds|cs):',operand)
    return f'c.r[{REGS[seg[1] if seg else ("ss" if re.search(r"\bbp\b",inner) else "ds")]}]',out

def width(v):
    if v in BYTES or 'byte ptr' in v:return 8
    if v in REGS or 'word ptr' in v:return 16
    raise ValueError('unknown operand width:'+v)

def read(v,w=16):
    v=v.strip()
    if v in REGS:return f'c.r[{REGS[v]}]'
    if v in BYTES:
        i,shift=BYTES[v];return f'((c.r[{i}] >> {shift}) & 255)'
    if '[' in v:
        seg,off=ea(v);return f'c.read({seg},{off},{width(v)})?'
    return f'{int(v,0)&((1<<w)-1)}u16'

def write(v,value):
    if v in REGS:return f'c.r[{REGS[v]}]={value};'
    if v in BYTES:
        i,shift=BYTES[v];mask=0xFF00 if shift==0 else 0x00FF
        return f'c.r[{i}]=(c.r[{i}] & {mask}) | (({value} & 255) << {shift});'
    seg,off=ea(v);return f'c.write({seg},{off},{width(v)},{value})?;'

def name(seg,ip):return f'p_{seg:04x}_{ip:04x}'
def emit_instruction(seg,ip,row):
    op,args,nxt,_=row;a=[x.strip() for x in args.split(',')]
    prefix=f'c.pc=({seg},{ip});c.step()?;'
    if op in ('ret','retf'):return prefix+f'c.ret({str(op=="retf").lower()},{int(args,0) if args else 0})?;return Ok(());'
    if op=='jmp':return prefix+f'label={target(seg,args)[0][1]};continue;'
    if op.startswith('j'):
        condition={'je':'c.z','jne':'!c.z','ja':'!c.cf&&!c.z','jae':'!c.cf','jb':'c.cf','jbe':'c.cf||c.z','jg':'!c.z&&(c.sf==c.of)','jge':'c.sf==c.of','jl':'c.sf!=c.of','jle':'c.z||(c.sf!=c.of)','js':'c.sf','jns':'!c.sf','jo':'c.of','jno':'!c.of','jp':'c.pf','jnp':'!c.pf'}[op]
        return prefix+f'label=if {condition}{{{target(seg,args)[0][1]}}}else{{{nxt}}};continue;'
    if op=='call':
        (cs,off),far=target(seg,args);return prefix+f'c.invoke({name(cs,off)},{str(far).lower()},{cs},{off},{nxt})?;label={nxt};continue;'
    if op=='enter':return prefix+f'c.enter({int(a[0],0)})?;'
    if op=='leave':return prefix+'c.r[4]=c.r[5];c.r[5]=c.pop()?;'
    if op=='push':return prefix+f'let v={read(a[0])};c.push(v)?;'
    if op=='pop':return prefix+'let v=c.pop()?;'+write(a[0],'v')
    if op in ('les','lds'):
        ss,off=ea(a[1]);return prefix+f'let o={off};let s={ss};let v=c.read(s,o,16)?;let g=c.read(s,o.wrapping_add(2),16)?;'+write(a[0],'v')+f'c.r[{8 if op=="les" else 11}]=g;'
    if op=='lea':return prefix+write(a[0],ea(a[1])[1])
    if op=='mov':return prefix+f'let v={read(a[1],width(a[0]))};'+write(a[0],'v')
    if op in ('cwde','cbw'):return prefix+'c.r[0]=i16::from(c.r[0] as u8 as i8) as u16;'
    if op in ('cdq','cwd'):return prefix+'c.r[2]=if (c.r[0] as i16)<0 {65535}else{0};'
    if op in ('lodsw','stosw'):
        if op=='lodsw':return prefix+'c.r[0]=c.read(c.r[11],c.r[6],16)?;c.r[6]=c.r[6].wrapping_add(2);'
        return prefix+'c.write(c.r[8],c.r[7],16,c.r[0])?;c.r[7]=c.r[7].wrapping_add(2);'
    if op in ('mul','imul'):
        if len(a)==3:return prefix+f'let v=({read(a[1])} as i16 as i32).wrapping_mul({read(a[2])} as i16 as i32) as u16;'+write(a[0],'v')
        return prefix+f'let v={read(a[0])};c.multiply(v,{str(op=="imul").lower()});'
    if op in ('div','idiv'):return prefix+f'let v={read(a[0])};c.divide(v,{str(op=="idiv").lower()})?;'
    if op in ('inc','dec','neg','not'):
        w=width(a[0]);return prefix+f'let x={read(a[0])};let v=c.{op}(x,{w});'+write(a[0],'v')
    if op in ('cmp','test','add','sub','adc','sbb','and','or','xor','shl','shr','sar'):
        w=width(a[0]);method={'cmp':'sub','test':'and'}.get(op,op)
        code=prefix+f'let x={read(a[0])};let y={read(a[1],w)};let v=c.{method}(x,y,{w});'
        return code+('let _=v;' if op in ('cmp','test') else write(a[0],'v'))
    raise ValueError(f'unsupported {seg:04X}:{ip:04X} {op} {args}')

def compile_graph(g):
    out=['//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.','//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.','use super::compiled::Cpu;']
    for (seg,entry),rows in sorted(g.items()):
        leaders={entry}
        for ip,(op,args,nxt,path) in rows.items():
            if op.startswith('j') or op=='call':
                if op.startswith('j'):leaders.add(target(seg,args)[0][1])
                if nxt in rows:leaders.add(nxt)
        out.append(f'pub(super) fn {name(seg,entry)}(c:&mut Cpu)->Result<(),String>{{let mut label={entry}u16;loop{{match label{{')
        for start in sorted(leaders):
            out.append(f'{start}=>{{');ip=start
            while True:
                row=rows[ip];out.append(emit_instruction(seg,ip,row));op,args,nxt,_=row
                if op.startswith('j') or op in ('call','ret','retf'):break
                if nxt in leaders:out.append(f'label={nxt};continue;');break
                ip=nxt
            out.append('}')
        out.append('_=>return Err(format!("invalid compiled label {label:04X}")),}}}')
    return '\n'.join(out)+'\n'

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);p.add_argument('--out',type=Path,default=ROOT/'engine/src/watched_compiled/adjustment.rs');p.add_argument('--check',action='store_true');args=p.parse_args()
    code,files=listing(args.research_root);g=graph(code,[(0x0AB9,0x35F8)]);source=compile_graph(g)
    import subprocess,tempfile
    with tempfile.TemporaryDirectory()as d:
        temp=Path(d)/'generated.rs';temp.write_text(source);subprocess.run(['rustfmt','--edition','2021',str(temp)],check=True);source=temp.read_bytes()
    if args.check:
        if args.out.read_bytes()!=source:raise ValueError('compiled Rust differs from listing translation')
    else:args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_bytes(source)
    used={row[3] for rows in g.values()for row in rows.values()}
    print(json.dumps({'roots':['0AB9:35F8'],'procedures':len(g),'instructions':sum(map(len,g.values())),'listing_digests':{p:files[p] for p in sorted(used)},'generated_sha256':hashlib.sha256(source).hexdigest()},indent=2))
if __name__=='__main__':main()
