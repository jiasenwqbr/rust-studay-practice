# The Rust Programming Language

## Foreword 前言

It wasn’t always so clear, but the Rust programming language is fundamentally about empowerment: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.

Rust 程序设计语言的本质实际在于 **赋能**（*empowerment*）：无论你现在编写的是何种代码，Rust 能让你在更为广泛的编程领域走得更远，写出自信。（这一点并不显而易见）

Take, for example, “systems-level” work that deals with low-level details of memory management, data representation, and concurrency. Traditionally, this realm of programming is seen as arcane, accessible only to a select few who have devoted the necessary years learning to avoid its infamous pitfalls. And even those who practice it do so with caution, lest their code be open to exploits, crashes, or corruption.

举例来说，那些“系统层面”的工作涉及内存管理、数据表示和并发等底层细节。从传统角度来看，这是一个神秘的编程领域，只为浸润多年的极少数人所触及，也只有他们能避开那些臭名昭著的陷阱。即使谨慎的实践者，亦唯恐代码出现漏洞、崩溃或损坏。

Rust breaks down these barriers by eliminating the old pitfalls and providing a friendly, polished set of tools to help you along the way. Programmers who need to “dip down” into lower-level control can do so with Rust, without taking on the customary risk of crashes or security holes, and without having to learn the fine points of a fickle toolchain. Better yet, the language is designed to guide you naturally towards reliable code that is efficient in terms of speed and memory usage.

Rust 破除了这些障碍：它消除了旧的陷阱，并提供了伴你一路同行的友好、精良的工具。想要 “深入” 底层控制的程序员可以使用 Rust，无需时刻担心出现崩溃或安全漏洞，也无需因为工具链不靠谱而被迫去了解其中的细节。更妙的是，语言设计本身会自然而然地引导你编写出可靠的代码，并且运行速度和内存使用上都十分高效。

Programmers who are already working with low-level code can use Rust to raise their ambitions. For example, introducing parallelism in Rust is a relatively low-risk operation: the compiler will catch the classical mistakes for you. And you can tackle more aggressive optimizations in your code with the confidence that you won’t accidentally introduce crashes or vulnerabilities.

已经在从事编写底层代码的程序员可以使用 Rust 来提升信心。例如，在 Rust 中引入并行是相对低风险的操作，因为编译器会替你找到经典的错误。同时你可以自信地采取更加激进的优化，而不会意外引入崩溃或漏洞。

But Rust isn’t limited to low-level systems programming. It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write — you’ll find simple examples of both later in the book. Working with Rust allows you to build skills that transfer from one domain to another; you can learn Rust by writing a web app, then apply those same skills to target your Raspberry Pi.

但 Rust 并不局限于底层系统编程。它表达力强、写起来舒适，让人能够轻松地编写出命令行应用、网络服务器等各种类型的代码——在本书中就有这两者的简单示例。使用 Rust 能让你把在一个领域中学习的技能延伸到另一个领域：你可以通过编写网页应用来学习 Rust，接着将同样的技能应用到你的 Raspberry Pi（树莓派）上。

This book fully embraces the potential of Rust to empower its users. It’s a friendly and approachable text intended to help you level up not just your knowledge of Rust, but also your reach and confidence as a programmer in general. So dive in, get ready to learn—and welcome to the Rust community!

本书全面介绍了 Rust 为用户赋予的能力。其内容平易近人，致力于帮助你提升 Rust 的知识，并且提升你作为程序员整体的理解与自信。欢迎你加入 Rust 社区，让我们准备深入学习 Rust 吧！

— Nicholas Matsakis and Aaron Turon

## Introduction 简介

```text
Note: This edition of the book is the same as The Rust Programming Language available in print and ebook format from No Starch Press.
注意：此书的英文原版与 No Starch Press 出版的纸质版和电子版《The Rust Programming Language》一致。
```

Welcome to *The Rust Programming Language*, an introductory book about Rust. The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

欢迎阅读《Rust 程序设计语言》，这是一本 Rust 语言的入门书。Rust 程序设计语言能帮助你编写更快、更可靠的软件。在编程语言设计中，上层的编程效率和底层的细粒度控制往往不能兼得，而 Rust 则试图挑战这一矛盾。Rust 通过平衡技术能力和开发体验，允许你控制内存使用等底层细节，同时也不需要担心底层控制带来的各种麻烦。

### Who Rust Is For Rust适合那些人

Rust is ideal for many people for a variety of reasons. Let’s look at a few of the most important groups.<br>Rust 适合很多开发者，其原因多种多样。下面讨论几个最重要的群体。

#### Teams of Developers 开发团队

Rust is proving to be a productive tool for collaborating among large teams of developers with varying levels of systems programming knowledge. Low-level code is prone to various subtle bugs, which in most other languages can be caught only through extensive testing and careful code review by experienced developers. In Rust, the compiler plays a gatekeeper role by refusing to compile code with these elusive bugs, including concurrency bugs. By working alongside the compiler, the team can spend their time focusing on the program’s logic rather than chasing down bugs.<br>实践证明，对于大规模的开发团队，即使成员的系统编程水平不同，Rust 仍是高效的协作工具。底层代码中容易出现多种不易察觉的 bug。在其他编程语言中想要找到它们，只能设计大量的测试，并且让经验丰富的开发者细心审核代码。在 Rust 中，编译器充当了守门员的角色。如果代码中存在这些难找的 bug，比如并发的 bug，它会拒绝编译。只要与编译器协同工作，团队就可以花更多的时间聚焦在程序逻辑上，无需费心找 bug。

Rust also brings contemporary developer tools to the systems programming world:

- Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.

  Cargo 是内置的依赖管理器和构建工具，它能轻松增加、编译和管理依赖，并使依赖在 Rust 生态系统中保持一致。

- The Rustfmt formatting tool ensures a consistent coding style across developers.

  Rustfmt 格式化工具确保开发者遵循一致的代码风格。

- The Rust Language Server powers Integrated Development Environment (IDE) integration for code completion and inline error messages.

  Rust Language Server 为集成开发环境（IDE）提供了强大的代码补全和内联错误信息功能。

By using these and other tools in the Rust ecosystem, developers can be productive while writing systems-level code.

通过使用 Rust 生态系统中丰富的工具，开发者可以更高效地编写系统层面代码。

#### Students 学生

Rust is for students and those who are interested in learning about systems concepts. Using Rust, many people have learned about topics like operating systems development. The community is very welcoming and happy to answer student questions. Through efforts such as this book, the Rust teams want to make systems concepts more accessible to more people, especially those new to programming.

Rust 适合学生群体，也适合有兴趣学习系统概念的人。很多人利用 Rust 学习了操作系统开发等内容。Rust 社区欢迎学生提问，也乐于解答问题。Rust 团队希望系统概念能让更多人了解，尤其是编程新手，为此编写了这本书以及其他内容。

#### Companies 公司

Hundreds of companies, large and small, use Rust in production for a variety of tasks, including command line tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even major parts of the Firefox web browser.

大大小小的公司都已在生产中使用 Rust 来完成各种任务，包括命令行工具、网络服务、DevOps 工具、嵌入式设备、音视频分析与转码、加密货币、生物信息学、搜索引擎、物联网（IOT）程序、机器学习等，甚至 Firefox 浏览器的主要部分也是用 Rust 编写的。

#### [Open Source Developers](https://doc.rust-lang.org/stable/book/ch00-00-introduction.html#open-source-developers)  [开源开发者](https://kaisery.github.io/trpl-zh-cn/ch00-00-introduction.html#开源开发者)

Rust is for people who want to build the Rust programming language, community, developer tools, and libraries. We’d love to have you contribute to the Rust language.

Rust 适合那些希望构建 Rust 编程语言、社区、开发工具和库的开发者。我们欢迎你为 Rust 语言贡献力量。

#### [People Who Value Speed and Stability](https://doc.rust-lang.org/stable/book/ch00-00-introduction.html#people-who-value-speed-and-stability)[重视速度和稳定性的开发者](https://kaisery.github.io/trpl-zh-cn/ch00-00-introduction.html#重视速度和稳定性的开发者)

Rust is for people who crave speed and stability in a language. By speed, we mean both how quickly Rust code can run and the speed at which Rust lets you write programs. The Rust compiler’s checks ensure stability through feature additions and refactoring. This is in contrast to the brittle legacy code in languages without these checks, which developers are often afraid to modify. By striving for zero-cost abstractions, higher-level features that compile to lower-level code as fast as code written manually, Rust endeavors to make safe code be fast code as well.

Rust 适合追求编程语言的速度与稳定性的开发者。对于速度来说，既是指 Rust 可以运行的多快，也是指编写 Rust 程序的速度。Rust 编译器会检查代码，确保增加功能、重构代码时的稳定性。与之相比，其他的一些语言没有检查功能，导致开发者往往不敢修改脆弱的祖传代码。Rust 力求将高级语言特性编译成底层代码，并且与手写的代码运行速度同样快，这一概念称为零开销抽象（zero-cost abstractions），可以让代码又安全又快速。

The Rust language hopes to support many other users as well; those mentioned here are merely some of the biggest stakeholders. Overall, Rust’s greatest ambition is to eliminate the trade-offs that programmers have accepted for decades by providing safety *and* productivity, speed *and*ergonomics. Give Rust a try and see if its choices work for you.

这里提到的只是较大的几个受益群体，Rust 语言也希望能支持更多其他用户。总之，Rust 最重要的目标是消除数十年来程序员习以为常的取舍，让安全和高效、速度和易读易用**可以兼得**。试试看 Rust，说不定它的选择就适合你。

### [Who This Book Is For](https://doc.rust-lang.org/stable/book/ch00-00-introduction.html#who-this-book-is-for)[本书适合哪些人](https://kaisery.github.io/trpl-zh-cn/ch00-00-introduction.html#本书适合哪些人)

This book assumes that you’ve written code in another programming language but doesn’t make any assumptions about which one. We’ve tried to make the material broadly accessible to those from a wide variety of programming backgrounds. We don’t spend a lot of time talking about what programming *is* or how to think about it. If you’re entirely new to programming, you would be better served by reading a book that specifically provides an introduction to programming.

本书假设你已经有其他编程语言的经验，任何语言均可，我们尽可能让各种语言背景的人都能读懂。本书的重点不是程序设计**本身**，也不是程序设计思维。如果你完全没学过编程，建议你先阅读专门介绍程序设计的书籍。

### [How to Use This Book](https://doc.rust-lang.org/stable/book/ch00-00-introduction.html#how-to-use-this-book)  [如何阅读本书](https://kaisery.github.io/trpl-zh-cn/ch00-00-introduction.html#如何阅读本书)

In general, this book assumes that you’re reading it in sequence from front to back. Later chapters build on concepts in earlier chapters, and earlier chapters might not delve into details on a particular topic but will revisit the topic in a later chapter.

本书大体上假设您按从头到尾的顺序阅读。后面的章节建立于前面章节的基础上。前面的章节可能不会深入介绍部分主题，而是留待后续章节重新讨论。

You’ll find two kinds of chapters in this book: concept chapters and project chapters. In concept chapters, you’ll learn about an aspect of Rust. In project chapters, we’ll build small programs together, applying what you’ve learned so far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.

本书分为两类章节：概念章节和项目章节。在概念章节中，我们学习 Rust 的某个方面。在项目章节中，我们应用目前所学的知识一同构建小型程序。第 2、12、20 章是项目章节；其余都是概念章节。

Chapter 1 explains how to install Rust, how to write a “Hello, world!” program, and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a hands-on introduction to writing a program in Rust, having you build up a number guessing game. Here we cover concepts at a high level, and later chapters will provide additional detail. If you want to get your hands dirty right away, Chapter 2 is the place for that. Chapter 3 covers Rust features that are similar to those of other programming languages, and in Chapter 4 you’ll learn about Rust’s ownership system. If you’re a particularly meticulous learner who prefers to learn every detail before moving on to the next, you might want to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when you’d like to work on a project applying the details you’ve learned.

第 1 章介绍如何安装 Rust，如何编写 “Hello, world!” 程序，以及如何使用 Rust 的包管理器和构建工具 Cargo。第 2 章是一个编写 Rust 语言的实战介绍，我们会构建一个猜猜看游戏。我们会站在较高的层次介绍一些概念，而将详细的介绍放在稍后的章节中。如果你希望立刻就动手实践一下，第 2 章正好适合你。第 3 章介绍 Rust 中类似其他编程语言的特性，你可以选择跳过，直接阅读第 4 章学习 Rust 的所有权（ownership）系统。不过，如果你注重细节，可以跳过第 2 章直接看第 3 章，之后想要写项目的时候再回来看第 2 章。

Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match` expressions, and the `if let` control flow construct. You’ll use structs and enums to make custom types in Rust.

第 5 章讨论结构体（struct）和方法，第 6 章介绍枚举（enum）、`match` 表达式和 `if let` 控制流结构。在 Rust 中，创建自定义类型需要用到结构体和枚举。

In Chapter 7, you’ll learn about Rust’s module system and about privacy rules for organizing your code and its public Application Programming Interface (API). Chapter 8 discusses some common collection data structures that the standard library provides, such as vectors, strings, and hash maps. Chapter 9 explores Rust’s error-handling philosophy and techniques.

第 7 章介绍 Rust 的模块（module）系统，其中的私有性规则用来组织代码和公开的 API（应用程序接口）。第 8 章讨论标准库提供的常见集合数据结构，例如 Vector（向量）、字符串和 Hash Map（散列表）。第 9 章探索 Rust 的错误处理技术和理念。

Chapter 10 digs into generics, traits, and lifetimes, which give you the power to define code that applies to multiple types. Chapter 11 is all about testing, which even with Rust’s safety guarantees is necessary to ensure your program’s logic is correct. In Chapter 12, we’ll build our own implementation of a subset of functionality from the `grep` command line tool that searches for text within files. For this, we’ll use many of the concepts we discussed in the previous chapters.

第 10 章深入介绍泛型（generic）、Trait 和生命周期（lifetime），用这些特性可以写出适用多种类型的代码。第 11 章介绍测试，因为就算 Rust 有安全保证，也需要测试确保程序逻辑正确。`grep` 命令可以查找文件中的文本，第 12 章中我们将会构建一个命令行工具实现它的部分功能，为此会用到之前章节讨论的很多概念。

Chapter 13 explores closures and iterators: features of Rust that come from functional programming languages. In Chapter 14, we’ll examine Cargo in more depth and talk about best practices for sharing your libraries with others. Chapter 15 discusses smart pointers that the standard library provides and the traits that enable their functionality.

第 13 章探索闭包（closure）和迭代器（iterator），这两个特性来自函数式编程语言。第 14 章会深入探讨 Cargo 并介绍分享代码库的最佳实践。第 15 章讨论标准库提供的智能指针以及相关的 Trait。

In Chapter 16, we’ll walk through different models of concurrent programming and talk about how Rust helps you to program in multiple threads fearlessly. Chapter 17 looks at how Rust idioms compare to object-oriented programming principles you might be familiar with.

第 16 章介绍几类并发编程模型，并讨论 Rust 如何助你无畏地编写多线程程序。第 17 章着眼于比较 Rust 风格与 OOP（面向对象编程）原则。

Chapter 18 is a reference on patterns and pattern matching, which are powerful ways of expressing ideas throughout Rust programs. Chapter 19 contains a smorgasbord of advanced topics of interest, including unsafe Rust, macros, and more about lifetimes, traits, types, functions, and closures.

第 18 章介绍模式和模式匹配，它是在 Rust 程序中表达思想的有效方式。第 19 章是一个高级主题大杂烩，包括不安全 Rust（unsafe Rust）、宏（macro）和更多关于生命周期、Trait、类型、函数和闭包的内容。

In Chapter 20, we’ll complete a project in which we’ll implement a low-level multithreaded web server!

第 20 章我们将会完成一个项目，实现一个底层的、多线程的网络服务器！

Finally, some appendices contain useful information about the language in a more reference-like format. Appendix A covers Rust’s keywords, Appendix B covers Rust’s operators and symbols, Appendix C covers derivable traits provided by the standard library, Appendix D covers some useful development tools, and Appendix E explains Rust editions. In Appendix F, you can find translations of the book, and in Appendix G we’ll cover how Rust is made and what nightly Rust is.

最后的附录包含一些实用信息，格式类似参考文档。附录 A 介绍 Rust 的关键字，附录 B 介绍 Rust 的运算符和符号，附录 C 介绍标准库提供的可派生 Trait，附录 D 涉及一些有用的开发工具，附录 E 介绍 Rust 的不同版本。

There is no wrong way to read this book: if you want to skip ahead, go for it! You might have to jump back to earlier chapters if you experience any confusion. But do whatever works for you.

本书没有错误的阅读方式，可以尽管跳过部分内容，遇到困惑时再回看前面的章节。

An important part of the process of learning Rust is learning how to read the error messages the compiler displays: these will guide you toward working code. As such, we’ll provide many examples that don’t compile along with the error message the compiler will show you in each situation. Know that if you enter and run a random example, it may not compile! Make sure you read the surrounding text to see whether the example you’re trying to run is meant to error. Ferris will also help you distinguish code that isn’t meant to work:

学习 Rust 的一个重点在于了解如何阅读编译器提供的错误信息，它们会指导你编写出能运行的代码。为此，我们会提供很多无法通过编译的示例，并附上错误信息。运行示例代码的时候务必阅读上下文，看看这段代码是否是故意写错的，部分示例代码就是无法编译的！Ferris 也会帮助你分辨哪些代码是故意写错的：

| Ferris                                                       | Meaning    **含义**                                          |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| <img src="https://doc.rust-lang.org/stable/book/img/ferris/does_not_compile.svg" alt="Ferris with a question mark" style="zoom: 10%;" /> | This code does not compile!这段代码无法通过编译！            |
| <img src="https://doc.rust-lang.org/stable/book/img/ferris/panics.svg" alt="Ferris throwing up their hands" style="zoom:10%;" /> | This code panics!这段代码会 Panic！                          |
| <img src="https://doc.rust-lang.org/stable/book/img/ferris/not_desired_behavior.svg" alt="Ferris with one claw up, shrugging" style="zoom:10%;" /> | This code does not produce the desired behavior.这段代码的运行结果不符合预期。 |

In most situations, we’ll lead you to the correct version of any code that doesn’t compile.

在大部分情况，我们会指导你将无法通过编译的代码修改为正确版本。

### [Source Code](https://doc.rust-lang.org/stable/book/ch00-00-introduction.html#source-code)  [源代码](https://kaisery.github.io/trpl-zh-cn/ch00-00-introduction.html#源代码)

The source files from which this book is generated can be found on [GitHub](https://github.com/rust-lang/book/tree/main/src).

生成本书的源码可以在 [GitHub](https://github.com/rust-lang/book/tree/main/src) 上找到。



## 1.[Getting Started ](https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html#getting-started)[入门指南](https://kaisery.github.io/trpl-zh-cn/ch01-00-getting-started.html#入门指南)

Let’s start your Rust journey! There’s a lot to learn, but every journey starts somewhere. In this chapter, we’ll discuss:

让我们开始 Rust 之旅！有很多内容需要学习，但每次旅程总有起点。在本章中，我们会讨论：

- Installing Rust on Linux, macOS, and Windows

  在 Linux、macOS 和 Windows 上安装 Rust

- Writing a program that prints `Hello, world!`

  编写一个打印 `Hello, world!` 的程序

- Using `cargo`, Rust’s package manager and build system

  使用 Rust 的包管理器和构建系统 `cargo`



### 1.1 Installation 安装

The first step is to install Rust. We’ll download Rust through `rustup`, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.

第一步是安装 Rust。我们会通过 `rustup` 下载 Rust，这是一个管理 Rust 版本和相关工具的命令行工具。下载时需要联网。

> Note: If you prefer not to use `rustup` for some reason, please see the [Other Rust Installation Methods page](https://forge.rust-lang.org/infra/other-installation-methods.html) for more options.
>
> 注意：如果你出于某些理由倾向于不使用 `rustup`，请到 [Rust 的其他安装方法页面](https://forge.rust-lang.org/infra/other-installation-methods.html) 查看其它安装选项。

The following steps install the latest stable version of the Rust compiler. Rust’s stability guarantees ensure that all the examples in the book that compile will continue to compile with newer Rust versions. The output might differ slightly between versions because Rust often improves error messages and warnings. In other words, any newer, stable version of Rust you install using these steps should work as expected with the content of this book.

接下来的步骤会安装最新的稳定版 Rust 编译器。Rust 的稳定性确保本书所有示例在最新版本的 Rust 中能够继续编译。不同版本的输出可能略有不同，因为 Rust 经常改进错误信息和警告。也就是说，任何通过这些步骤安装的最新稳定版 Rust，都应该能正常运行本书中的内容。

#### [Command Line Notation](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#command-line-notation)[命令行标记](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#命令行标记)

> In this chapter and throughout the book, we’ll show some commands used in the terminal. Lines that you should enter in a terminal all start with `$`. You don’t need to type the `$`character; it’s the command line prompt shown to indicate the start of each command. Lines that don’t start with `$` typically show the output of the previous command. Additionally, PowerShell-specific examples will use `>` rather than `$`.
>
> 本章和全书中，我们会展示一些在终端中使用的命令。所有需要输入到终端的行都以 `$` 开头。你不需要输入`$`字符；这里显示的`$`字符表示命令行提示符，仅用于提示每行命令的起点。不以 `$` 起始的行通常展示前一个命令的输出。另外，PowerShell 专用的示例会采用 `>` 而不是 `$`。

#### [Installing `rustup` on Linux or macOS](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos) [在 Linux 或 macOS 上安装 `rustup`](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#在-linux-或-macos-上安装-rustup)

If you’re using Linux or macOS, open a terminal and enter the following command:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup` tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

此命令下载一个脚本并开始安装 `rustup` 工具，这会安装最新稳定版 Rust。过程中可能会提示你输入密码。如果安装成功，将会出现如下内容：

```text
Rust is installed now. Great!
```

You will also need a *linker*, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

On macOS, you can get a C compiler by running:

另外，你还需要一个 *链接器（linker）*，这是 Rust 用来将其编译的输出连接到一个文件中的程序。很可能你已经有一个了。如果你遇到了链接器错误，请尝试安装一个 C 编译器，它通常包括一个链接器。C 编译器也很有用，因为一些常见的 Rust 包依赖于 C 代码，因此需要安装一个 C 编译器。

在 macOS 上，你可以通过运行以下命令获得 C 语言编译器：

```console
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the `build-essential` package.

Linux 用户通常需要根据发行版（distribution）文档安装 GCC 或 Clang。比如，如果你使用 Ubuntu，可以安装 `build-essential` 包。

#### [Installing `rustup` on Windows](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-windows) [在 Windows 上安装 `rustup`](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#在-windows-上安装-rustup)

On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later.

To acquire the build tools, you’ll need to install [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). When asked which workloads to install, include:

- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing

The rest of this book uses commands that work in both *cmd.exe* and PowerShell. If there are specific differences, we’ll explain which to use.

在 Windows 上，前往 [https://www.rust-lang.org/install.html](https://www.rust-lang.org/tools/install) 并按照说明安装 Rust。在安装过程的某个步骤，你会收到一个信息说明为什么需要安装 Visual Studio 2013 或其更新版本的 MSVC 构建工具。

要获取构建工具，你需要安装 [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/)。当被问及需要安装什么工作负载（Workload）的时候，请确保勾选了以下内容：

- “使用 C++ 的桌面开发”（“Desktop Development with C++”）
- Windows 10（或 11）SDK
- 英语语言包，以及其他你所需要的语言包

本书的余下部分会使用能同时运行于 *cmd.exe* 和 PowerShell 的命令。如果存在特定差异，我们会解释使用哪一个。

#### [Troubleshooting](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#troubleshooting)  [检查安装是否正确（Troubleshooting）](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#检查安装是否正确troubleshooting)

To check whether you have Rust installed correctly, open a shell and enter this line:

要检查是否正确安装了 Rust，打开命令行并输入：

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest stable version that has been released, in the following format:

你应该可以看到按照以下格式显示的最新稳定版本的版本号、对应的 Commit Hash 和 Commit 日期：

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t see this information, check that Rust is in your `%PATH%` system variable as follows.

如果看到了这样的信息，就说明 Rust 已经安装成功了！

In Windows CMD, use:

```console
> echo %PATH%
```

In PowerShell, use:

```powershell
> echo $env:Path
```

In Linux and macOS, use:

```console
$ echo $PATH
```

If that’s all correct and Rust still isn’t working, there are a number of places you can get help. Find out how to get in touch with other Rustaceans (a silly nickname we call ourselves) on [the community page](https://www.rust-lang.org/community).

如果一切正确但 Rust 仍不能使用，有许多地方可以求助。您可以在[社区页面](https://www.rust-lang.org/community)查看如何与其他 Rustaceans（Rust 用户的称号，有自嘲意味）联系。

#### [Updating and Uninstalling](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#updating-and-uninstalling)

Once Rust is installed via `rustup`, updating to a newly released version is easy. From your shell, run the following update script:

```console
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your shell:

```console
$ rustup self uninstall
```

#### [Local Documentation](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#local-documentation) 本地文档

The installation of Rust also includes a local copy of the documentation so that you can read it offline. Run `rustup doc` to open the local documentation in your browser.

Any time a type or function is provided by the standard library and you’re not sure what it does or how to use it, use the application programming interface (API) documentation to find out!

安装程序也自带一份文档的本地拷贝，可以离线阅读。运行 `rustup doc` 在浏览器中查看本地文档。

任何时候，如果你拿不准标准库中的类型或函数的用途和用法，请查阅应用程序接口（application programming interface，API）文档！

### 1.2 Hello,World!

#### [Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html#hello-world)

Now that you’ve installed Rust, it’s time to write your first Rust program. It’s traditional when learning a new language to write a little program that prints the text `Hello, world!` to the screen, so we’ll do the same here!

> Note: This book assumes basic familiarity with the command line. Rust makes no specific demands about your editing or tooling or where your code lives, so if you prefer to use an integrated development environment (IDE) instead of the command line, feel free to use your favorite IDE. Many IDEs now have some degree of Rust support; check the IDE’s documentation for details. The Rust team has been focusing on enabling great IDE support via `rust-analyzer`. See [Appendix D](https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html) for more details.

#### [Creating a Project Directory](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html#creating-a-project-directory)

You’ll start by making a directory to store your Rust code. It doesn’t matter to Rust where your code lives, but for the exercises and projects in this book, we suggest making a *projects* directory in your home directory and keeping all your projects there.

Open a terminal and enter the following commands to make a *projects* directory and a directory for the “Hello, world!” project within the *projects* directory.

For Linux, macOS, and PowerShell on Windows, enter this:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

For Windows CMD, enter this:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

#### [Writing and Running a Rust Program](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html#writing-and-running-a-rust-program)

Next, make a new source file and call it *main.rs*. Rust files always end with the *.rs* extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use *hello_world.rs* rather than *helloworld.rs*.

Now open the *main.rs* file you just created and enter the code in Listing 1-1.

Filename: main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Listing 1-1: A program that prints `Hello, world!`

Save the file and go back to your terminal window in the *~/projects/hello_world* directory. On Linux or macOS, enter the following commands to compile and run the file:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

On Windows, enter the command `.\main.exe` instead of `./main`:

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Regardless of your operating system, the string `Hello, world!` should print to the terminal. If you don’t see this output, refer back to the [“Troubleshooting”](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#troubleshooting) part of the Installation section for ways to get help.

If `Hello, world!` did print, congratulations! You’ve officially written a Rust program. That makes you a Rust programmer—welcome!

#### [Anatomy of a Rust Program](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html#anatomy-of-a-rust-program)

Let’s review this “Hello, world!” program in detail. Here’s the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a function named `main`. The `main` function is special: it is always the first code that runs in every executable Rust program. Here, the first line declares a function named `main` that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses `()`.

The function body is wrapped in `{}`. Rust requires curly brackets around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

> Note: If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called `rustfmt` to format your code in a particular style (more on `rustfmt` in[Appendix D](https://doc.rust-lang.org/stable/book/appendix-04-useful-development-tools.html)). The Rust team has included this tool with the standard Rust distribution, as `rustc` is, so it should already be installed on your computer!

The body of the `main` function holds the following code:

```rust
    println!("Hello, world!");
This line does all the work in this little program: it prints text to the screen. There are four important details to notice here.First, Rust style is to indent with four spaces, not a tab.Second, println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). We’ll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.Third, you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.Fourth, we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.Compiling and Running Are Separate StepsYou’ve just run a newly created program, so let’s examine each step in the process.Before running a Rust program, you must compile it using the Rust compiler by entering the rustccommand and passing it the name of your source file, like this:
$ rustc main.rs
If you have a C or C++ background, you’ll notice that this is similar to gcc or clang. After compiling successfully, Rust outputs a binary executable.On Linux, macOS, and PowerShell on Windows, you can see the executable by entering the lscommand in your shell:
$ ls
main  main.rs
On Linux and macOS, you’ll see two files. With PowerShell on Windows, you’ll see the same three files that you would see using CMD. With CMD on Windows, you would enter the following:
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
This shows the source code file with the .rs extension, the executable file (main.exe on Windows, but main on all other platforms), and, when using Windows, a file containing debugging information with the .pdb extension. From here, you run the main or main.exe file, like this:
$ ./main # or .\main.exe on Windows
If your main.rs is your “Hello, world!” program, this line prints Hello, world! to your terminal.If you’re more familiar with a dynamic language, such as Ruby, Python, or JavaScript, you might not be used to compiling and running a program as separate steps. Rust is an ahead-of-time compiledlanguage, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code. Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs.
```



### 1.3 Hello,Cargo!

#### [Hello, Cargo!](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#hello-cargo)

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs *dependencies*.)

The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies. If we had built the “Hello, world!” project with Cargo, it would only use the part of Cargo that handles building your code. As you write more complex Rust programs, you’ll add dependencies, and if you start a project using Cargo, adding dependencies will be much easier to do.

Because the vast majority of Rust projects use Cargo, the rest of this book assumes that you’re using Cargo too. Cargo comes installed with Rust if you used the official installers discussed in the[“Installation”](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installation) section. If you installed Rust through some other means, check whether Cargo is installed by entering the following in your terminal:

```console
$ cargo --version
```

If you see a version number, you have it! If you see an error, such as `command not found`, look at the documentation for your method of installation to determine how to install Cargo separately.

#### [Creating a Project with Cargo](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#creating-a-project-with-cargo)

Let’s create a new project using Cargo and look at how it differs from our original “Hello, world!” project. Navigate back to your *projects* directory (or wherever you decided to store your code). Then, on any operating system, run the following:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

The first command creates a new directory and project called *hello_cargo*. We’ve named our project *hello_cargo*, and Cargo creates its files in a directory of the same name.

Go into the *hello_cargo* directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a *Cargo.toml* file and a *src* directory with a *main.rs* file inside.

It has also initialized a new Git repository along with a *.gitignore* file. Git files won’t be generated if you run `cargo new` within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`.

> Note: Git is a common version control system. You can change `cargo new` to use a different version control system or no version control system by using the `--vcs` flag. Run `cargo new --help` to see the available options.

Open *Cargo.toml* in your text editor of choice. It should look similar to the code in Listing 1-2.

Filename: Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Listing 1-2: Contents of *Cargo.toml* generated by `cargo new`

This file is in the [*TOML*](https://toml.io/) (*Tom’s Obvious, Minimal Language*) format, which is Cargo’s configuration format.

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use. We’ll talk about the `edition` key in [Appendix E](https://doc.rust-lang.org/stable/book/appendix-05-editions.html).

The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as *crates*. We won’t need any other crates for this project, but we will in the first project in Chapter 2, so we’ll use this dependencies section then.

Now open *src/main.rs* and take a look:

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a “Hello, world!” program for you, just like the one we wrote in Listing 1-1! So far, the differences between our project and the project Cargo generated are that Cargo placed the code in the *src* directory and we have a *Cargo.toml* configuration file in the top directory.

Cargo expects your source files to live inside the *src* directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the *src* directory and create an appropriate *Cargo.toml* file.

#### [Building and Running a Cargo Project](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project)

Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your *hello_cargo* directory, build your project by entering the following command:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in *target/debug/hello_cargo* (or *target\debug\hello_cargo.exe*on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named *debug*. You can run the executable with this command:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

If all goes well, `Hello, world!` should print to the terminal. Running `cargo build` for the first time also causes Cargo to create a new file at the top level: *Cargo.lock*. This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

We just built a project with `cargo build` and ran it with `./target/debug/hello_cargo`, but we can also use `cargo run` to compile the code and then run the resultant executable all in one command:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Using `cargo run` is more convenient than having to remember to run `cargo build` and then use the whole path to the binary, so most developers use `cargo run`.

Notice that this time we didn’t see output indicating that Cargo was compiling `hello_cargo`. Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have seen this output:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? Often, `cargo check` is much faster than `cargo build`because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process of letting you know if your project is still compiling! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they’re ready to use the executable.

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no matter which operating system you’re working on. So, at this point, we’ll no longer provide specific instructions for Linux and macOS versus Windows.

#### [Building for Release](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#building-for-release)

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in *target/release*.

#### [Cargo as Convention](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#cargo-as-convention)

With simple projects, Cargo doesn’t provide a lot of value over just using `rustc`, but it will prove its worth as your programs become more intricate. Once programs grow to multiple files or need a dependency, it’s much easier to let Cargo coordinate the build.

Even though the `hello_cargo` project is simple, it now uses much of the real tooling you’ll use in the rest of your Rust career. In fact, to work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

For more information about Cargo, check out [its documentation](https://doc.rust-lang.org/cargo/).

#### [Summary](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#summary)

You’re already off to a great start on your Rust journey! In this chapter, you’ve learned how to:

- Install the latest stable version of Rust using `rustup`
- Update to a newer Rust version
- Open locally installed documentation
- Write and run a “Hello, world!” program using `rustc` directly
- Create and run a new project using the conventions of Cargo

This is a great time to build a more substantial program to get used to reading and writing Rust code. So, in Chapter 2, we’ll build a guessing game program. If you would rather start by learning how common programming concepts work in Rust, see Chapter 3 and then return to Chapter 2.











