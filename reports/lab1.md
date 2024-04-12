# Report

## 实现功能

实现目标：系统调用 `sys_task_info` 。
实现细节：在结构体 `TaskControlBlock` 中多记录了两个信息，第一次调度的时间 `start_time` 以及使用各个系统调用的次数 `sys_cnt` 。在每次 `syscall` 的时候都会调用 `syscall_count` 来进行计数，然后在查询 `sys_task_info` 的时候返回。 `start_time` 则在 `os/src/task/mod.rs` 里 `lazy_static!` 初始化 `Task_Manager` 的时候调用 `get_time_ms` 获得，之后获取 `sys_task_info` 时同样调用，相减即得。

## 问答题

> 1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

`ch2b_bad_address.rs` 中访问了地址为 `0x0` 的数据，为非法地址，故报页错误 `[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.`

`ch2b_bad_instructions.rs` 中使用指令 `sret` ，这是在 U 态无法使用的指令，故报错非法指令 `[kernel] IllegalInstruction in application, kernel killed it.`

`ch2b_bad_register.rs` 中访问寄存器 `sstatus` ，这是在 U 态无法访问的寄存器，故报错非法指令 `[kernel] IllegalInstruction in application, kernel killed it.`

SBI版本： `RustSBI-QEMU Version 0.2.0-alpha.2`

> 2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
> ...

2.1 `a0` 为内核栈的栈顶；在开始运行一个应用程序时要调用 `goto_restore` 在里面调用 `__restore` 初始化应用程序的上下文，以及在解决完Trap之后恢复应用程序的上下文。

2.2 即 `sstatus, sepc, sscratch` 这三个寄存器，`sstatus` 记录Trap之前CPU处在的特权等级， `sepc` 要用于最后 `sret` 返回的地址， `sscratch` 指向用户栈的地址。

2.3 因为 `x2` 寄存器即为 `sp` 寄存器，Trap前原来指着用户栈地址，现在上下文中存在 `sscratch` 对应的位置中，故无需保存。 `x4` 为 `tp` 寄存器，一般不会用到，无需保存。

2.4 该指令即为交换 `sp` 和 `sscratch` 寄存器的值，交换后， `sp` 指向用户栈栈顶， `sscratch` 指向内核栈栈顶。

2.5 `sret` 。在执行 `sret` 之后，会恢复成 `sstatus` 中保存的特权等级

2.6 `sp`指向内核栈， `sscratch` 指向用户栈。

2.7 发生Trap时，CPU会跳到 `stvec` 指向的（也就是 `__alltraps`）位置，并将当前特权态设置为 S

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。