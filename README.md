# Rust转译binutils

## 使用Docker

1. 使用Docker运行需要先拉取**完整**仓库
```bash
git clone --recurse-submodules https://github.com/licheam/binutils-rust
```

2. 运行docker
```bash
docker buildx build -t binutils-rust --platform linux/amd64 -f Dockerfile ./
```

3. binutils二进制文件目录在`/home/user/binutils-rust/target/debug`

4. binutils测试结果在`/home/user/FunctionTest/result.txt`

```
        === libctf Summary ===
# of expected passes		3
# of unsupported tests		2
		=== binutils Summary ===
# of expected passes		293
# of unsupported tests		2
		=== ld Summary ===
# of expected passes		2744
# of expected failures		59
# of untested testcases		6
# of unsupported tests		29
		=== gas Summary ===
# of expected passes		1559
# of unsupported tests		1
```

## 直接编译

1. 拉取仓库
```bash
git clone https://github.com/licheam/binutils-rust
```

2. 安装binutils依赖
``` bash
sudo dnf group install -y "Development Tools"
sudo dnf install -y openssl-devel gmp-devel llvm-libs-12.0.1-2.oe2203 llvm-devel-12.0.1-2.oe2203 clang-devel cmake
```

3. 安装Rust

4. 编译binutils-rust
```bash
cargo build --bins --keep-going -Z sparse-registry
```

5. 检查二进制文件数量，应当为13个
```bash
find ./target/debug -maxdepth 1 -type f -executable | wc -l
```

## 依赖图

### binutils

[binutils依赖关系](./dependencies_binutils.pdf)

### ld

[ld依赖关系](./dependencies_ld.pdf)

### gas

[gas依赖关系](./dependencies_gas.pdf)

### gprof

[gprof依赖关系](./dependencies_gprof.pdf)