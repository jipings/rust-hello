
## 了解链路跟踪实现的原理及OpenTracing
## Rust生态中关于实现链路跟踪的现状
## 通过Datafuse理解全链路跟踪 ｜ OpenTracing让监控一个分布式调用过程简单化

## Google Dapper
Google 是最早在大规模分布式系统中实践分布式跟踪的公司之一，Google在2010年发表论文 Dapper，a Large-Scale Distributed Systems Tracing Infrastructure 也成为了其他厂商进行全链路跟踪应用的重要参考文档。

论文地址 http://research.google/pubs/pub36356/

## 什么是 Datafuse

Datafuse 是一个开源的、完全面向云架构的新式数仓，它提供快速的弹性拓展能力，并结合云的弹性、简单性和低成本，使 Data Cloud 构建变得更加容易。 

Datafuse 把数据存储在像AWS S3， Azure Blob 这些云上的存储系统，可以使不同的计算节点挂载同一份数据，从而做到较高的弹性，实现对资源的精细化控制。 

Github: https://github.com/datafuselabs/datafuse

datafuse tracing: https://github.com/datafuselabs/databend/pull/1566

tracing:

https://github.com/tokio-rs/tracing

Rustracing
minitrace


## OpenTracing 主要数据模型

* Trace:
代表了一个事务或者流程（分布式）系统中的执行过程，是多个span组成的一个有向无环图

* Span:
具有开始时间和执行时长的逻辑运行单元，span之间通过嵌套或者顺序排列建立逻辑因果关系

* Tag：
每个span可以多个健值对（key:value）形式的Tag，Tag是没有时间戳的，
Tag 用于记录跟踪系统感兴趣的 metric，不同类型的 span 可能会记录不同的 metric，比如Http类型的span
会记录http.status_code, mysql 类型 span 可能使用 db.statement 来记录执行的 sql 语句。

* SpanContext
SpanContext 代表跨越进程边界，传递到下级 span 的状态，至少包含 `<trace_id, span_id, sampled>` 元组，
以及可选的 Baggage. SpanContext 在整个链路中会向下传递。
SpanContext是跨链路传输中非常重要的概念，它包含了需要在链路中传递的全部信息。

