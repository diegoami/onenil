//! [confirmed] 09BD (static d9e20a4) formats queue text at rows2–4.
//! [designed] The native viewer formats accepted numeric events separately.
//! Numeric queue/counter effects remain in compiled 0F94/0E45/0F42.
use super::compiled::Cpu;
pub(super) fn commentary_display(c:&mut Cpu)->Result<(),String>{c.ret(false,6)}

pub(super) fn sound_delay(_c:&mut Cpu)->Result<(),String>{Err("native presentation delay not implemented".into())}
pub(super) fn sound_burst(_c:&mut Cpu)->Result<(),String>{Err("native presentation burst not implemented".into())}
