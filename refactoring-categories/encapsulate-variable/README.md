# 封装变量 (Encapsulate Variable)

[English](README_EN.md) | [简体中文](README.md)

本项目展示了使用 Rust 进行**封装变量**重构的技术，演示了如何将可变的全局状态转换为受控的、封装良好的形式。

## 概述

**封装变量**重构的核心在于将公开暴露的可变全局变量转换为通过函数进行受控访问的变量。这能提供单一的控制点，便于维护不变量，并提高代码的安全性和可维护性。

## 文件结构

- `s00_before_encapsulation.rs`: 原始代码，直接访问全局变量。
- `s01_after_encapsulation.rs`: 基础封装，通过 getter/setter 函数访问。
- `s02_1_find_modify.rs`: 克隆封装的前置步骤，识别并重构修改逻辑。
- `s02_2_clone_encapsulation.rs`: 克隆封装，通过返回副本防止外部修改。
- `s03_set_clone_encapsulation.rs`: Setter 接收副本，确保安全性。
- `main.rs`: 主入口点，运行所有示例。

## 重构步骤详解

### 步骤 0: 封装前 (Before Encapsulation)

全局可变状态被直接暴露，任何代码都可以无控制地读取或修改它。这导致无法维护数据的不变量。

```rust
// 直接读取和修改全局状态，没有访问控制
let mut spaceship = Spaceship {
    owner: DEFAULT_OWNER.lock().unwrap().clone(),
};

*DEFAULT_OWNER.lock().unwrap() = Owner {
    first_name: "Rebecca".to_string(),
    last_name: "Parsons".to_string(),
};
```

### 步骤 1: 基础封装 (Basic Encapsulation)

目标：**控制对变量引用的修改**。

1.  将全局变量设为私有。
2.  提供 getter 和 setter 函数。
    -   **Getter**: 提供对数据的访问（在 Rust 中通常返回 MutexGuard）。
    -   **Setter**: 它是修改引用的唯一入口。

```rust
// 获取默认所有者的读取访问权限
fn get_default_owner() -> std::sync::MutexGuard<'static, Owner> {
    DEFAULT_OWNER.lock().expect("mutex poisoned")
}

// 设置默认所有者
fn set_default_owner(new_owner: Owner) {
    let mut owner = DEFAULT_OWNER.lock().expect("mutex poisoned");
    *owner = new_owner;
}
```

### 步骤 2: 高级封装 - 控制内容修改 (Advanced Encapsulation)

目标：不仅控制引用的修改，还要**控制变量内容的修改**。

#### 2.1 识别修改逻辑
在使用克隆封装前，需要找到所有直接通过引用修改变量内容的代码。可以通过返回一个不可变引用或包装类型（Immutable Wrapper）来利用编译器查找这些修改点。

#### 2.2 克隆封装 (Clone Encapsulation)
Getter 函数返回数据的**副本**（Clone），而不是引用。这样外部代码对数据的修改不会影响到共享的全局状态。

```rust
// Getter 返回数据的克隆
// 客户端获得的是数据的副本，对副本的修改不会影响全局状态
fn default_owner() -> Owner {
    DEFAULT_OWNER_DATA.lock().expect("mutex poisoned").clone()
}
```

### 步骤 3: Setter 接收副本 (Setter Receives Copy)

Setter 函数接收数据的副本（所有权转移）。在 Rust 中，这利用了移动语义（Move Semantics），确保 Setter 拥有完整的数据所有权，避免了共享引用的副作用。

## 关键概念与 Rust 优势

### 封装的好处
-   **控制**: 单一访问控制点。
-   **安全**: 防止意外的外部修改。
-   **可维护性**: 内部实现变更不影响外部调用。
-   **调试**: 数据流向更清晰。

### Rust 的特性支持
-   **所有权系统 (Ownership)**: 天然防止非预期的共享可变性。
-   **Move 语义**: Setter 天然接收所有权（或副本），切断与外部的联系。
-   **Clone Trait**: 显式复制，明确语义，没有隐式深拷贝。
-   **编译时检查**: 借用检查器能发现许多并发和内存安全问题。

## 运行示例

进入目录并运行项目：

```bash
cd refactoring-categories/encapsulate-variable
cargo run
```

运行测试：

```bash
cargo test
```

## 深入阅读

-   [Rust 封装应用场景详解](rust应用场景.md) (包含更多 Rust 特有的封装模式，如 Arc/Mutex, Builder, Newtype 等)
-   《重构：改善既有代码的设计》 - Martin Fowler
