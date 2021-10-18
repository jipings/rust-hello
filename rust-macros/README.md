
## [Rust 宏小册子](https://zjp-cn.github.io/tlborm/introduction.html)

## [macros.md](https://rustwiki.org/zh-CN/reference/macros.html)

### 声明宏

句法
```rs
MacroRulesDefinition :
   macro_rules ! IDENTIFIER MacroRulesDef

MacroRulesDef :
      ( MacroRules ) ;
   | [ MacroRules ] ;
   | { MacroRules }

MacroRules :
   MacroRule ( ; MacroRule )* ;?

MacroRule :
   MacroMatcher => MacroTranscriber

MacroMatcher :
      ( MacroMatch* )
   | [ MacroMatch* ]
   | { MacroMatch* }

MacroMatch :
      Token排除 $ 和 定界符
   | MacroMatcher
   | $ IDENTIFIER : MacroFragSpec
   | $ ( MacroMatch+ ) MacroRepSep? MacroRepOp

MacroFragSpec :
      block | expr | ident | item | lifetime | literal
   | meta | pat | path | stmt | tt | ty | vis

MacroRepSep :
   Token排除 定界符 和 重复操作符

MacroRepOp :
   * | + | ?

MacroTranscriber :
   DelimTokenTree
```

每个声明宏都有一个名称和一条或多条规则。每条规则都有两个部分： 一个匹配器（matcher），描述它匹配的句法；
一个转码器（transcriber），描述成功匹配后将执行的替代调用句法。匹配器和转码器都必须由定界符（delimiter）包围。
宏可以拓展位表达式、语句、程序项（包括trait、impl和外来程序项）、类型或模式。

* 元变量

在匹配器中，`$`名称：匹配段选择器这种句法格式匹配符合指定句法类型的 Rust 句法段，并将其绑定到元变量 `$` 名称上。有效的匹配段选择器包括：

* item： 程序项
* block： 块表达式
* stmt：语句，注意此选择器不匹配句尾的分号（如果匹配器中提供了分号，会被当做分隔符），但碰到分号是自身的一部分的程序项语句的情况又会匹配。
* pat: 模式
* expr：表达式
* ty：类型
* ident：标识符或关键字
* path：类型表达式形式的路径
* tt：token树（单个 token 或宏匹配定界符()、[]或{} 中的标记）
* meta：属性，属性中的内容
* lifttime：生存期token
* vis：可能为空的可见性限定符
* literal：匹配- 字面量表达式


* 宏的卫生性

默认情况下，宏中引用的所有标识符都按原样展开，并在宏的调用位置上去查找。如果宏引用的程序项或宏不在调用位置的作用域内，则这可能会导致问题。为了解决这个问题，可以替代在路径的开头使用元变量 `$crate`，强制在定义宏的 crate 中进行查找。

### 过程宏
过程宏有三种形式：
* 类函数宏（function-like macros）- custom!(...)
* 派生宏（derive macros）- `#[derive(CustomDerive)]`
* 属性宏（attribute macros）- `#[CustomAttribute]`

过程宏允许在编译时运行对 Rust 句法进行操作的代码，它可以在消费掉一些 Rust 句法输入的同时产生新的 Rust 句法输出。
可以将过程宏想象成是从一个AST到另一个AST的函数映射。 

* 过程宏的卫生性

过程宏是非卫生的（unhygienic）。这意味着它的行为就好像它输出的 token 流是被简单地内联写入它周围的代码中一样。
这意味着它会受到外部程序项的影响，也会影响外部导入。

