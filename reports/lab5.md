# Report

## 功能实现

本实验实现的功能是死锁检测，使用系统调用 `fn enable_deadlock_detect(is_enable: i32) -> i32` 来开关死锁检测功能。

在 `ProcessControlBlockInner` 中增加变量 `deadlock_detect` 表示死锁检测功能是否开启。

对于死锁检测功能，在 `ProcessControlBlockInner` 中增加以下数据结构，分别表示mutex锁和semaphore信号量对应的死锁检测需要用到的 `Work, Need, Allocation` 数据

```rust
    /// mutex work
    pub mutex_work: Vec<usize>,
    /// mutex allocated
    pub mutex_allocated: Vec<Vec<usize>>,
    /// mutex need
    pub mutex_need: Vec<Vec<usize>>,
    ...
    /// semaphore work
    pub semaphore_work: Vec<usize>,
    /// semaphore allocated
    pub semaphore_allocated: Vec<Vec<usize>>,
    /// semaphore need
    pub semaphore_need: Vec<Vec<usize>>,
```

由于其中的数据均为 `Vec` 相关类型，因此在初始化进程、添加新的锁、信号量以及添加新的线程时都需要对以上数据结构的大小进行更新。相关代码在 `os/src/syscall/thread.rs` `os/src/syscall/sync.rs` `os/src/task/process.rs` 中。

更新 `Vec` 大小后，我们只需要注意数据结构中数据的更新。以下仅以信号量semaphore为例，mutex为semaphore的特例，故实现几乎相同。

在系统调用 `sys_semaphore_create` 中，在添加新的信号量时，我们需要更新 `semaphore_work` 中对应位置的值为信号量的初始值。

在系统调用 `sys_semaphore_down` 中，我们首先讲对应线程对应信号量的 `Need` 数据进行更新，即

 ```rust
 semaphore_need[tid][sem_id] += 1;
 ```

此时还没有获取到信号量，所以更新的是Need。然后调用 `sem.down();` 之后，说明已经获取到了信号量，此时对 `Need, Work, Allocated` 都进行更新，即

```rust
semaphore_need[tid][sem_id] -= 1;
semaphore_allocated[tid][sem_id] += 1;
semaphore_work[sem_id] -= 1;
```

在系统调用 `sys_semaphore_up` 中，我们需要释放已获取的信号量，因此需对 `Work, Allocated` 进行更新，即

```rust
semaphore_allocated[tid][sem_id] -= 1;
semaphore_work[sem_id] += 1;
```

这样我们就完成了对 `Work, Need, Allocation` 数据的更新

这时，我们在 `sys_semaphore_down` 试图获取信号量之前，更新 `Need` 之后，调用 `semaphore_deadlock` 函数来检测死锁，如果有死锁，则还原 `Need` 并报错退出，如果没死锁则继续调用 `sem.down` 

在函数 `semaphore_deadlock` 中，我们就根据已有的算法，利用数据 `Need, Work, Allocated` ，不断寻找未完成的且 `Need` 全部小于等于 `Work` 的线程，并将其 `Allocated` 的数据释放，直到所有线程结束。能完成则无死锁，否则有死锁。

## 问答题

> 1. 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 - 需要回收的资源有哪些？ - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

需要回收的资源有相应线程的控制块 `TaskControlBlock` 包括其中的内核态栈空间、用户态栈空间（因为该线程还未退出）、跳板页等。

其他线程的TaskControlBlock可能在进程中的 `tasks` 中，或在锁或信号量的 `wait_queue` 中被引用；需要被回收，因为使用的是 `Arc` 指针，会增加引用计数，引用计数为0时才会被销毁。

> 2. 对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？
> ```rust
>  1impl Mutex for Mutex1 {
>  2    fn unlock(&self) {
>  3        let mut mutex_inner = self.inner.exclusive_access();
>  4        assert!(mutex_inner.locked);
>  5        mutex_inner.locked = false;
>  6        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
>  7            add_task(waking_task);
>  8        }
>  9    }
> 10}
> 11
> 12impl Mutex for Mutex2 {
> 13    fn unlock(&self) {
> 14        let mut mutex_inner = self.inner.exclusive_access();
> 15        assert!(mutex_inner.locked);
> 16        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
> 17            add_task(waking_task);
> 18        } else {
> 19            mutex_inner.locked = false;
> 20        }
> 21    }
> 22}
> ```

第一种是先将当前锁置为 $0$ 然后如果 `wait_queue` 中有任务，则将任务加到task manager的task list中。

第二种是先判断 `wait_queue` 中是否有任务，如果有的话加到task manager的task list中，如果没有的话才将锁置为 $0$

第一种实现可能会产生问题，因为当 `wait_queue` 中有任务时将其加到task list中，此时该任务是拥有锁的，但是锁此时还是为 $0$ ，可能会有另外一个试图拿到锁的task拿到锁并加到task list，产生冲突。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

​	无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

​	无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
