use rtos_macros::rtos_import;

use crate::{arch, task};

#[rtos_import]
static TIME_US_PER_TICK: u64;

#[rtos_import]
static TIME_TICK_FREQUENCY: u32;

#[inline]
fn us_per_tick() -> u64 {
    // Safety: Reads an immutable constant.
    unsafe { TIME_US_PER_TICK }
}

#[inline]
fn tick_frequency() -> u32 {
    // Safety: Reads an immutable constant.
    unsafe { TIME_TICK_FREQUENCY }
}

// Multiply two u64 integers returning the high 64 bits of the result.
#[inline]
fn u64_mul_high(lhs: u64, rhs: u64) -> u64 {
    ((lhs as u128) * (rhs as u128) >> 64) as u64
}

#[inline]
pub fn ticks_to_us(ticks: u64) -> u64 {
    u64_mul_high(ticks, us_per_tick())
}

#[inline]
pub fn us_to_ticks(us: u32) -> u64 {
    const US_PER_S: u64 = 1000000;
    u64_mul_high(us as u64 * tick_frequency() as u64, u64::MAX / US_PER_S)
}

#[inline]
pub fn now_ticks() -> u64 {
    arch::now_ticks()
}

pub fn handle_timer_expiration(
    task_table: &mut task::TaskTable,
    task_idx: task::TaskId,
) -> task::Schedule {
    task::evaluate_timers(task_table, task_idx, now_ticks())
}

pub fn update_deadline(deadline: u64) {
    if deadline < arch::timer_deadline() {
        arch::set_timer_deadline(deadline)
    }
}
