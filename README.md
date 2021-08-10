# th-tools

该项目主要是同步携程城市数据到redis。

## 使用方法

Mac系统，在公司网络(连接VPN)中执行。

```bash
# clone 代码
git clone https://github.com/cntehang/th-tools
# 执行同步工具
./th-tools
```

##  China cities

You can download the `cities.json` from this repo below.

[https://github.com/modood/Adminitrative-divisions-of-China.git](https://github.com/modood/Adminitrative-divisions-of-China.git)

## Other topics

### How to compile this project?

Here, we use the [rust-lang](https://www.rust-lang.org) language, and use the Cargo tool-chain.

If you have already installed the rust environment, you can follow the tips step by step.

Before we start, you need to clone the repo to your local.

```bash
#clone the codes.
git clone --depth 1 https://github.com/cntehang/th-tools.git
```

#### Run the project.

It's very simple, here we go.

```bash
cd th-tools
cargo run
```

Right?

#### Compiling for develop

After you write your codes, you want to build a binary.

```bash
cd th-tools
cargo build
```

#### Compiling for release.

```bash
cd th-tools
# compie the codes for release.
cargo build --release
cp target/release/th-tools .
# Run the binary.
./th-tools
```

