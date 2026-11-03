#!/usr/bin/env python3
"""[designed] Compile checked listing structure into native Rust basic blocks;no runtime decoding."""
import argparse,hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SOURCE_ADDRESSES=False
RELOCATED=set()
REGS={'ax':0,'cx':1,'dx':2,'bx':3,'sp':4,'bp':5,'si':6,'di':7,'es':8,'cs':9,'ss':10,'ds':11}
REG32={'e'+r:i for r,i in REGS.items()if i<8}
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
    if not re.fullmatch(r'(?:0x[0-9a-f]+|[0-9]+)',args):raise ValueError('indirect control transfer:'+args)
    return(seg,int(args,0)&65535),False

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
    if 'dword ptr' in v:raise ValueError('unsupported dword operand:'+v)
    if v in REG32 and SOURCE_ADDRESSES:return 32
    if v in BYTES or 'byte ptr' in v:return 8
    if v in REGS or 'word ptr' in v:return 16
    raise ValueError('unknown operand width:'+v)

def read(v,w=16):
    v=v.strip()
    if SOURCE_ADDRESSES and v in REG32:return f'c.get32({REG32[v]})'
    if v in REGS:return f'c.r[{REGS[v]}]'
    if v in BYTES:
        i,shift=BYTES[v];return f'(c.r[{i}] >> {shift}) & 255'
    if '[' in v:
        seg,off=ea(v);return f'c.read({seg},{off},{width(v)})?'
    return f'{int(v,0)&((1<<w)-1)}u16'

def write(v,value):
    if SOURCE_ADDRESSES and v in REG32:return f'c.set32({REG32[v]},{value});'
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
        condition={'je':'c.z','jne':'!c.z','ja':'!c.cf&&!c.z','jae':'!c.cf','jb':'c.cf','jbe':'c.cf||c.z','jg':'!c.z&&(c.sf==c.of)','jge':'c.sf==c.of','jl':'c.sf!=c.of','jle':'c.z||(c.sf!=c.of)','js':'c.sf','jns':'!c.sf','jo':'c.of','jno':'!c.of','jp':'c.pf','jnp':'!c.pf','jcxz':'c.r[1]==0'}.get(op)
        if condition is None:raise ValueError(f'unsupported conditional {seg:04X}:{ip:04X} {op}')
        return prefix+f'label=if {condition}{{{target(seg,args)[0][1]}}}else{{{nxt}}};continue;'
    if op=='call':
        (cs,off),far=target(seg,args);return prefix+f'c.invoke({name(cs,off)},{str(far).lower()},{cs},{off},{nxt})?;label={nxt};continue;'
    if op=='enter':return prefix+f'c.enter({int(a[0],0)})?;'
    if op=='leave':return prefix+'c.r[4]=c.r[5];c.r[5]=c.pop()?;'
    if op=='push':
        value=read(a[0]);value=f'c.segment_const({value})'if (seg,ip)in RELOCATED else value
        return prefix+f'let v={value};c.push(v)?;'
    if op=='pop':return prefix+'let v=c.pop()?;'+write(a[0],'v')
    if op in ('les','lds'):
        ss,off=ea(a[1]);return prefix+f'let o={off};let s={ss};let(v,g)=c.far(s,o)?;'+write(a[0],'v')+f'c.r[{8 if op=="les" else 11}]=g;'
    if op=='lea':return prefix+write(a[0],ea(a[1])[1])
    if op=='mov':
        value=read(a[1],width(a[0]));value=f'c.segment_const({value})'if (seg,ip)in RELOCATED else value
        return prefix+f'let v={value};'+write(a[0],'v')
    if op=='xchg':
        if not all(v in REGS and REGS[v]<8 for v in a):raise ValueError('unsupported exchange operand:'+args)
        return prefix+f'c.exchange_registers({REGS[a[0]]},{REGS[a[1]]});'
    if SOURCE_ADDRESSES and op in ('cld','std'):return prefix+f'c.df={str(op=="std").lower()};'
    if op=='nop':return prefix
    if SOURCE_ADDRESSES and op in ('lodsb','lodsw'):return prefix+f'c.load_string({8 if op=="lodsb"else 16})?;'
    if SOURCE_ADDRESSES and op in ('stosb','stosw'):return prefix+f'c.store_string({8 if op=="stosb"else 16})?;'
    if SOURCE_ADDRESSES and op=='rep':
        operation=args.split()[0]
        if operation in ('movsb','movsw'):return prefix+f'c.repeat_movs({8 if operation=="movsb"else 16})?;'
        if operation in ('stosb','stosw'):return prefix+f'c.repeat_stos({8 if operation=="stosb"else 16})?;'
        raise ValueError('unsupported repeat operation:'+operation)
    if op in ('cwde','cbw'):return prefix+'c.r[0]=i16::from(c.r[0] as u8 as i8) as u16;'
    if SOURCE_ADDRESSES and op=='cdq'and nxt-ip==2:return prefix+'let v=if (c.get32(0)as i32)<0{u32::MAX}else{0};c.set32(2,v);'
    if op in ('cdq','cwd'):return prefix+'c.r[2]=if (c.r[0] as i16)<0 {65535}else{0};'
    if op in ('lodsw','stosw'):
        if op=='lodsw':return prefix+'c.r[0]=c.read(c.r[11],c.r[6],16)?;c.r[6]=c.r[6].wrapping_add(2);'
        return prefix+'c.write(c.r[8],c.r[7],16,c.r[0])?;c.r[7]=c.r[7].wrapping_add(2);'
    if SOURCE_ADDRESSES and op=='shl'and a[0]in REG32:
        return prefix+f'let x={read(a[0])};let n={read(a[1])};let v=c.shl32(x,n);'+write(a[0],'v')
    if SOURCE_ADDRESSES and op in('shld','shrd')and a[0]in REG32 and a[1]in REG32:
        return prefix+f'let x={read(a[0])};let y={read(a[1])};let n={read(a[2])};let v=c.double_shift32({str(op=="shld").lower()},x,y,n);'+write(a[0],'v')
    if SOURCE_ADDRESSES and op=='imul'and len(a)==1 and a[0]in REG32:
        return prefix+f'let v={read(a[0])};c.multiply_signed32(v);'
    if op in ('mul','imul'):
        if len(a)==3:return prefix+f'let x={read(a[1])};let y={read(a[2])};let v=c.imul3(x,y);'+write(a[0],'v')
        if width(a[0])!=16:raise ValueError('unsupported multiply width')
        return prefix+f'let v={read(a[0])};c.multiply(v,{str(op=="imul").lower()});'
    if SOURCE_ADDRESSES and op=='idiv'and a[0]in REG32:return prefix+f'let v={read(a[0])};c.divide_signed32(v)?;'
    if SOURCE_ADDRESSES and op=='rol':return prefix+f'let v={read(a[0])};let n={read(a[1])};let v=c.rol(v,n,{width(a[0])});'+write(a[0],'v')
    if SOURCE_ADDRESSES and op=='rcl':return prefix+f'let v={read(a[0])};let n={read(a[1])};let v=c.rcl(v,n,{width(a[0])});'+write(a[0],'v')
    if op in ('div','idiv'):return prefix+f'let v={read(a[0])};c.divide(v,{str(op=="idiv").lower()})?;'
    if op in ('inc','dec','neg','not'):
        w=width(a[0]);return prefix+f'let x={read(a[0])};let v=c.{op}(x,{w});'+write(a[0],'v')
    if op in ('cmp','test','add','sub','adc','sbb','and','or','xor','shl','shr','sar'):
        w=width(a[0]);method={'cmp':'sub','test':'and'}.get(op,op)
        code=prefix+f'let x={read(a[0])};let y={read(a[1],w)};let v=c.{method}(x,y,{w});'
        return code+('let _=v;' if op in ('cmp','test') else write(a[0],'v'))
    raise ValueError(f'unsupported {seg:04X}:{ip:04X} {op} {args}')

def compile_procedure(seg,entry,rows,partial_errors=None):
    out=[]
    leaders={entry}
    for ip,(op,args,nxt,path) in rows.items():
        if op.startswith('j') or op=='call':
            if op.startswith('j'):leaders.add(target(seg,args)[0][1])
            if nxt in rows:leaders.add(nxt)
    out.append(f'pub(super) fn {name(seg,entry)}(c:&mut Cpu)->Result<(),String>{{let mut label={entry}u16;loop{{match label{{')
    for start in sorted(leaders):
        out.append(f'{start}=>{{');ip=start
        while True:
            row=rows[ip]
            try:out.append(emit_instruction(seg,ip,row))
            except ValueError as error:
                if partial_errors is None:raise
                partial_errors[f'{seg:04X}:{ip:04X}']=str(error)
                out.append(f'c.pc=({seg},{ip});return Err("unported original instruction {seg:04X}:{ip:04X}".into());')
            op,args,nxt,_=row
            if op.startswith('j') or op in ('call','ret','retf'):break
            if nxt in leaders:out.append(f'label={nxt};continue;');break
            ip=nxt
        out.append('}')
    out.append('_=>return Err(format!("invalid compiled label {label:04X}")),}}}')
    return out

def compile_graph(g,deferred=None,adapters=None,auto_defer=False,report=None,roots=None,partial=None):
    out=['//! [confirmed] Mechanically compiled numeric control flow;F39/c7637bd,static d9e20a4.','//! [designed] Direct Rust calls/blocks;no runtime decoder or original executable.','use super::compiled::Cpu;']
    deferred=deferred or set();adapters=adapters or {};report=report if report is not None else {};bodies={}
    for (seg,entry),rows in sorted(g.items()):
        key=(seg,entry)
        if key in adapters:
            bodies[key]=[f'pub(super) fn {name(seg,entry)}(c:&mut Cpu)->Result<(),String>{{super::projections::{adapters[key]}(c)}}']
            report[f'{seg:04X}:{entry:04X}']={'kind':'projection','function':adapters[key]};continue
        if key in(partial or set()):
            errors={};bodies[key]=compile_procedure(seg,entry,rows,errors)
            report[f'{seg:04X}:{entry:04X}']={'kind':'partial','unsupported_instructions':errors};continue
        reason='explicit unported boundary'if key in deferred else None
        if reason is None:
            try:bodies[key]=compile_procedure(seg,entry,rows);continue
            except ValueError as error:
                if not auto_defer:raise
                reason=str(error)
        report[f'{seg:04X}:{entry:04X}']={'kind':'unported','reason':reason}
        bodies[key]=[f'pub(super) fn {name(seg,entry)}(_c:&mut Cpu)->Result<(),String>{{Err("unported original {seg:04X}:{entry:04X}".into())}}']
    reached=set();pending=list(roots)if roots else list(g)
    while pending:
        key=pending.pop()
        if key in reached:continue
        reached.add(key)
        boundary=report.get(f'{key[0]:04X}:{key[1]:04X}')
        if boundary and boundary['kind']!='partial':continue
        for op,args,*_ in g[key].values():
            if op=='call':pending.append(target(key[0],args)[0])
    for key in sorted(reached):out.extend(bodies[key])
    for key in list(report):
        if tuple(int(v,16)for v in key.split(':'))not in reached:report.pop(key)
    return '\n'.join(out)+'\n'

def main():
    global SOURCE_ADDRESSES,RELOCATED
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);p.add_argument('--out',type=Path,default=ROOT/'engine/src/watched_compiled/adjustment.rs');p.add_argument('--check',action='store_true');p.add_argument('--root',action='append');p.add_argument('--defer',action='append',default=[]);p.add_argument('--partial',action='append',default=[]);p.add_argument('--projection',action='append',default=[]);p.add_argument('--defer-unsupported',action='store_true');p.add_argument('--source-addresses',action='store_true');args=p.parse_args()
    code,files=listing(args.research_root);roots=args.root or ['0AB9:35F8'];parse=lambda value:tuple(int(v,16)for v in value.split(':'))
    SOURCE_ADDRESSES=args.source_addresses
    if SOURCE_ADDRESSES:
        for path in files:
            segment=int(Path(path).name.split('_')[1],16)
            for line in(args.research_root/path).read_text().splitlines():
                m=re.match(r'^([0-9A-F]{4}).*; relocated segment word ',line)
                if m:RELOCATED.add((segment,int(m[1],16)))
    deferred={parse(v)for v in args.defer};adapters={} 
    for item in args.projection:
        address,function=item.split('=')
        if not re.fullmatch('[a-z_][a-z_0-9]*',function):raise ValueError('invalid projection name')
        adapters[parse(address)]=function
    g=graph(code,[parse(v)for v in roots]);report={};source=compile_graph(g,deferred,adapters,args.defer_unsupported,report,[parse(v)for v in roots],{parse(v)for v in args.partial})
    import subprocess,tempfile
    with tempfile.TemporaryDirectory()as d:
        temp=Path(d)/'generated.rs';temp.write_text(source);subprocess.run(['rustfmt','--edition','2021',str(temp)],check=True);source=temp.read_bytes()
    if args.check:
        if args.out.read_bytes()!=source:raise ValueError('compiled Rust differs from listing translation')
    else:args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_bytes(source)
    used={row[3] for rows in g.values()for row in rows.values()}
    result={'roots':roots,'procedures':len(g),'instructions':sum(map(len,g.values())),'listing_digests':{p:files[p] for p in sorted(used)},'generated_sha256':hashlib.sha256(source).hexdigest()}
    if report:result['boundaries']=report
    if SOURCE_ADDRESSES:result['source_addresses']=True
    print(json.dumps(result,indent=2))
if __name__=='__main__':main()
