//! Process management syscalls
use core::{mem::size_of, panic};

use crate::{mm::{MapPermission, PTEFlags, StepByOne, VirtAddr, translated_byte_buffer,}, task::{change_program_brk, current_insert_framed_area, current_remove_page, current_translate_addr, current_user_token, exit_current_and_run_next, get_cur_syscall_data, suspend_current_and_run_next}, timer::get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    // Panic on addr not available
    let vec = translated_byte_buffer(current_user_token(),_ts as *const u8, size_of::<TimeVal>());
    if vec.len() == 1{
        unsafe{
            *(vec[0].as_ptr() as *mut TimeVal) = TimeVal{
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            }
        }
    }else{
        //跨页
        if vec[0].len() != size_of::<usize>(){panic!("Not align ptr")}
        if vec[1].len() != size_of::<usize>(){panic!("Not align ptr")}
        unsafe{
            *(vec[0].as_ptr() as *mut usize) = us / 1_000_000;
            *(vec[1].as_ptr() as *mut usize) = us % 1_000_000;
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let Some(addr) = current_translate_addr(VirtAddr::from(_id).floor()) else {return -1};
            if !addr.is_valid() || !addr.readable() || addr.flags() & PTEFlags::U == PTEFlags::empty() {return -1}
            addr.ppn().get_bytes_array()[VirtAddr::from(_id).page_offset()] as isize
        },
        1 => {
            let Some(addr) = current_translate_addr(VirtAddr::from(_id).floor()) else{return -1};
            if !addr.is_valid() || !addr.writable() || addr.flags() & PTEFlags::U == PTEFlags::empty() {return -1}
            unsafe {
                *(addr.ppn().get_bytes_array().as_ptr().add(VirtAddr::from(_id).page_offset()) as *mut u8) = _data as u8;
            }
            0
        },
        2 => {
            get_cur_syscall_data(_id)
        },
        _ => -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if port & !0x7 != 0 || port & 0x7 == 0{
        return -1;
    }
    let mut permission = MapPermission::U;
    if port & 0b1 != 0 {permission |= MapPermission::R}
    if port & 0b10 != 0 {permission |= MapPermission::W}
    if port & 0b100 != 0 {permission |= MapPermission::X}
    let start = VirtAddr::from(_start);
    if start.page_offset() != 0 {
        return -1;
    }
    if _len == 0{return 0}
    let end = VirtAddr::from(start.0 + _len).ceil();
    let mut vpn = start.floor();
    while vpn.0 < end.0{
        if let Some(addr) = current_translate_addr(vpn.clone()){
            if addr.is_valid(){return -1}
        }
        vpn.step()
    };
    current_insert_framed_area(start, end.into(), permission);
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start = VirtAddr::from(_start);
    if start.page_offset() != 0 {
        return -1;
    }
    let end = VirtAddr::from(_start + _len).ceil();
    if current_remove_page(start.floor(), end){
        0isize
    }else{
        -1isize
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
