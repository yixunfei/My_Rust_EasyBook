

建议直接阅读Rust的API标准库！

[[https://rustwiki.org/zh-CN/std/]]
可以直接在上方的搜索框内搜索，非常方便！

### 基本类型与操作

**`std::primitive` - 基本数据类型定义**:

 - `u8, i8, u16, i16, u32, i32, u64, i64, u128, i128`: 不同大小的有符号/无符号整数类型。
 - `isize, usize`: 指针宽度相关的有符号/无符号整数类型。
 - `f32, f64`: 单精度和双精度浮点数类型。
 - `char`: Unicode 字符类型。
 - `bool`: 布尔类型。

**`td::ops` - 操作符相关 trait**
- `Add`, `Sub`, Mul`,` `Div`, `Rem`, `BitAnd`, `BitOr`, `BitXor`, `Not`, `Shl`, `Shr` 等 trait：定义算术和位操作符的行为。
 - `std::cmp` - 比较相关 trait 和函数
 - `PartialEq`, `Eq`, `PartialOrd`,` Ord traits`：定义相等性和顺序比较行为。
 - `max`, `min` 函数：返回两个值中的最大值或最小值。

**`std::convert` - 类型转换相关 trait**
 - `From`, `Into` `traits`：定义类型之间的显式和隐式转换规则。
 - `std::clone` - 克隆相关 trait
 - `Clone trait`：定义如何创建类型实例的深拷贝。

**`std::marker` - 标记 trait**
 - `Copy`, `Send`, `Sync`, `Unpin`, `Sized` 等 trait：标记类型具有特定属性（如可复制、可跨线程发送等）。

### 内存管理与生命周期

**`std::mem` - 内存操作相关**
 - `drop`, `forget`, `size_of`, `align_of` 等函数：如前所述。

**`std::boxed` - 智能指针 Box**
 - `Box<T>`：堆上分配的唯一所有者类型。

**`std::rc` - 引用计数智能指针 Rc**
 - `Rc<T>`：允许多个不可变引用共享所有权的类型。

**`std::cell` - 内存安全的内部可变性**
 - `Cell<T>`, `RefCell<T>`：允许在不可变引用存在的条件下修改值。

**`std::borrow` - 引用与借用相关 trait**
 - **Borrow**, **BorrowMut traits**：定义如何借用类型实例。

**`std::slice` - 切片操作**
 - `[T] `类型及其方法：固定长度或动态长度的不可变/可变元素序列。

**`std::str` - 字符串切片操作**
 - &str 类型及其方法：UTF-8 编码的字符串切片。

**`std::string `- 字符串类型**
 - `String` 类型及其方法：可增长、可变的 UTF-8 编码字符串。
### 控制流程与错误处理

**`std::option `- 可选值处理**
 - `Option<T>` 类型及其方法：表示值可能存在或不存在。

**`std::result` - 错误处理**
 - `Result<T, E>` 类型及其方法：表示计算结果可能成功或失败。

**`std::panic` - 异常处理**
 - `catch_unwind` 函数：捕获当前线程的 panic。

`std::iter` - 迭代器相关 trait 和函数
- `Iterator trait`：定义遍历元素的接口

..........
