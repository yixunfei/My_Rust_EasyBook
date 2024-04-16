
TODO ....



裸指针
使用 `*const T `或 `*mut T `类型的裸指针直接访问内存地址。裸指针允许读写内存而不受 Rust 的所有权和生命周期规则约束，因此需要程序员确保指针始终有效且不违反内存安全。
执行指针的解引用（dereference）、指针算术、内存分配与释放（如使用 alloc crate）等操作通常需要在 unsafe 块中进行。

FFI
与 C/C++ 等非 Rust 语言编写的库进行互操作时，需要定义 extern 函数。unsafe 关键字用于标记这些外部函数声明以及相关的包装代码。

汇编
asm 宏
```
#![feature(asm)]

fn main() {
    // 声明输出寄存器（out）和输入/输出寄存器（inout）
    let mut output: u32;
    let input: u32 = 42;

    unsafe {
        // 使用 asm! 宏执行汇编语句
        asm!(
            // 汇编指令
            "add {0}, {1}",
            // 寄存器绑定
            inout(reg) output => output,
            in(reg) input,
            // 指定汇编代码的选项
            options(nostack),
        );
    }

    println!("Output: {}", output); // 输出: 42
}

```

unsafe函数

类型转换

