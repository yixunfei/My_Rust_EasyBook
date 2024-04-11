标准库（std）中的常用API
基本类型与数据结构
`Option<T>`：表示值可能存在或不存在，常用于避免空值（null）问题，配合Some和None枚举值以及unwrap、expect、map、unwrap_or等方法使用。
`Result<T, E>`：表示操作的结果可能成功（Ok(T)）或失败（Err(E)），用于错误处理，搭配?运算符、try!宏、map、unwrap、unwrap_or等方法使用。
`Vec<T>`：动态大小的数组，支持高效增删元素。
`String`：可增长和可变的UTF-8字符串类型。
`&str`：不可变字符串切片，用于引用字符串字面量或String的一部分。
`HashMap<K, V>`：基于哈希表的关联数组，提供键值对存储和查找。
`HashSet<T>`：基于哈希表的无序唯一元素集合。

控制流与迭代
`if let` 和 `match`：模式匹配语句，用于处理枚举和结构体等复合类型的不同情况。
`for` 循环：遍历迭代器产生的序列。
`Iterator` 和 `IntoIterator traits`：定义了遍历集合元素的通用接口，以及map、filter、fold、collect等方法。
错误处理与日志
`panic!`：触发程序 panic，通常用于不可恢复的错误情况。
`assert!`、`assert_eq!`、`debug_assert!` 等：断言宏，用于在开发阶段检查程序状态。
`log` 库：提供跨平台的日志记录接口，可通过配置调整日志级别。

I/O与文件系统
`std::fs`：提供了文件和目录操作的API，如read_to_string、write、create_dir、remove_file等。
`std::io`：包含输入/输出相关的类型和函数，如BufReader、BufWriter、Read、Write traits等。
`std::net`：网络编程相关API，如TCP/UDP套接字、域名解析等。
并发与异步编程
`std::thread`：创建和管理线程的API。
`std::sync`：同步原语，如Mutex、RwLock、Arc、Condvar等。
std::future、std::task 和 **`std::