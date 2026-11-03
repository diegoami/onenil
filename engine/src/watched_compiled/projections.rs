//! [confirmed] Original presentation boundaries,static d9e20a4.
//! Numeric queue/counter effects remain in compiled0F94/0E45/0F42.
//! Sound countdown/ISR phase ownership:main amendment8fdce1d and
//! records/T37/presentation-clock-boundary-r0001.json.
use super::compiled::Cpu;
pub(super) fn commentary_display(c: &mut Cpu) -> Result<(), String> {
    c.ret(false, 6)
}
fn sound_request(c: &mut Cpu, burst: bool) -> Result<(), String> {
    c.enter(if burst { 6 } else { 0 })?;
    let (offset, segment) = c.far(c.r[11], 0xD7D2)?;
    if c.read(segment, offset.wrapping_add(2), 8)? != 0 {
        let duration = c.read(c.r[10], c.r[5].wrapping_add(6), 16)?;
        let ticks = if burst {
            duration.wrapping_mul(c.read(c.r[11], 0xD5C, 16)?)
        } else {
            duration
        };
        c.presentation.push(serde_json::json!({"site":if burst{"1998:01E0"}else{"1998:00D6"},"kind":if burst{"tone_burst"}else{"delay"},"ticks":ticks}));
    }
    c.r[4] = c.r[5];
    c.r[5] = c.pop()?;
    c.ret(true, 2)
}
pub(super) fn sound_delay(c: &mut Cpu) -> Result<(), String> {
    sound_request(c, false)
}
pub(super) fn sound_burst(c: &mut Cpu) -> Result<(), String> {
    sound_request(c, true)
}
