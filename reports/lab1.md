## 诚信声明

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

- 无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

- 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 功能实现

我实现的功能是对于每一个Task的每一个Syscall的记录

由于syscall固定,数目较小且HashMap为std的数据结构,因此采用了简单的二维数组来记录数据

将5个syscall映射到5个index,由于UPSafeCell可以保证独占性,因此不会有并发问题

## 简答作业

1. RustSBI version 0.4.1
```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```
2. 如下

---
1. 对于从陷阱中返回的情况,此时sp就等于本task内核栈的sp
对于从run_first_task进入的情况,此时sp等于init_app_cx生成的内核栈的栈顶

2. sstatus sepc sscratch

sstatus记录了trap前特权级信息
sepc记录了trap前PC地址
sscratch记录了中断触发后内核栈的sp地址


3. x2是sp寄存器,已在13行保存,x4是tp寄存器,程序不会使用/修改

4. sscratch代表当前内核栈栈指针,sp是当前task用户栈栈地址

5. sret,ISA约定

6. sp代表当前内核栈栈指针,sscratch是当前task用户栈栈地址

7. ecall/外部中断
---