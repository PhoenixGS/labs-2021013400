# Report

## 功能实现

本实验首先完成了一个系统调用 `sys_spawn` 。该系统调用首先获取指定文件名的代码段，然后使用fork一个新的进程，并使用exec将代码段替换到新进程的内存空间中，最后将新进程加入到 `ready_queue` 中。

除此之外，实验还完成了stride调度算法。在TCB中添加了两个信息，分别是进程的优先级priority和已运行的长度stride。

运行系统系统调用 `sys_set_priority` 时，会首先判断优先级是否大于等于2，不满足的话直接返回-1。如果满足则修改TCB中的priority，并返回优先级。

在TaskManager中fetch下一个任务时，会在 `ready_queue` 中逐个扫描每个进程的stride，选择最小的进程运行，并给其stride加上 `BIGSTRIDE / priority` 。

## 问答题

stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

* 实际情况是轮到 p1 执行吗？为什么？

> 不是，因为使用8bit无符号位整型储存的话，p2执行后p2.stride溢出，变为4，仍然比p1的stride要小，依旧是p2执行

我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

* 为什么？尝试简单说明（不要求严格证明）。

> 如果当前满足STRIDE_MAX - STRIDE_MIN <= BigStride / 2，故此时为STRIDE_MIN对应的进程执行，由于priority>=2，故步长step = BigStride / priority <= BigStride / 2，则加上后最大不超过STRIDE_MIN + BigStride / 2，故加上之后进程中STRIDE最大值还是不超过STRIDE_MIN + BigStride / 2，而最小值大于等于STRIDE_MIN，依旧保证了对于所有进程来说 STRIDE_MAX' - STRIDE_MIN' <= BigStride / 2。然后由于初始值均等，故归纳可得。

已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if abs(self.0 - other.0) <= BigStride / 2 {
            Some(self.0.cmp(&other.0))
        } else {
            Some(other.0.cmp(&self.0))
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```
TIPS: 使用 8 bits 存储 stride, BigStride = 255, 则: `(125 < 255) == false, (129 < 255) == true`.

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
