//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    mm::{
        translated_byte_buffer, VPNRange, VirtAddr
    },
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, task_get_entry, task_get_status_and_time, task_get_syscall_cnt, task_mmap, TaskStatus
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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

    let buffers = translated_byte_buffer(current_user_token(), _ts as *mut u8, core::mem::size_of::<TimeVal>());

    let ts  = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let mut ptr = &ts as *const TimeVal as *const u8;
    for buffer in buffers {
        let len = buffer.len();
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(ptr, len));
            ptr = ptr.add(len);
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let (status, start_time) = task_get_status_and_time();
    let syscall_cnt = task_get_syscall_cnt();

    let ti = TaskInfo {
        status: status,
        syscall_times: syscall_cnt,
        time: get_time_ms() - start_time,
    };

    let mut ptr = &ti as *const TaskInfo as *const u8;
    let buffers = translated_byte_buffer(current_user_token(), _ti as *mut u8, core::mem::size_of::<TaskInfo>());
    for buffer in buffers {
        let len = buffer.len();
        unsafe {
            buffer.copy_from_slice(core::slice::from_raw_parts(ptr, len));
            ptr = ptr.add(len);
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    
    // check
    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }

    
    for vpn in VPNRange::new(VirtAddr::from(_start).floor(), VirtAddr::from(_start + _len).ceil()) {
        let pte = task_get_entry(vpn);
        if pte.is_some() && pte.unwrap().is_valid() {
            return -1;
        }
    }

    task_mmap(VirtAddr::from(_start).floor().into(), VirtAddr::from(_start + _len).ceil().into(), _port);

    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
