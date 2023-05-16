## Rust 语言入门
rust语言是前端基础建设的未来
### 关于性能：
1. rust提前做了编译优化，为每个变量｜函数都做了预处理，避免动态为语言分配内存，所以编译会比较久，但运行是真的块。
2. rust主要依赖栈的数据结构，无须像堆那种，需要为一些特定的数据结构查找一块足够连续的内存空间。
3. 无GC，没有其他语言那样运行一个GC去处理没有用到的变量内存空间，三色标记｜新老生代（坏处就是生命周期需要手动申明变量的存在时长，否则编译报错）。
4. 所有语言中性能仅此于C，但拥有很多高级语言的特性，如async,解构等



## 文章推荐

## 学习地址（官方）
https://www.rustwiki.org.cn/zh-CN
https://course.rs/
## cargo 文档
https://cargo.budshome.com/index.html
## 更换cargo源
https://cargo.budshome.com/reference/source-replacement.html
## hello_cargo
rust基础入门demo
## runjs
v8运行时调用rust函数
## wasm 
rsut编译wasm,可处理一些cpu密集型任务。
https://rustwasm.github.io/wasm-pack/book/quickstart.html

