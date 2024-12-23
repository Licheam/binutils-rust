FROM openeuler/openeuler:22.03-lts

# 换dnf源
RUN sed -e 's|^metalink=|#metalink=|g' \
    -e 's|^baseurl=http://repo.openeuler.org|baseurl=https://mirrors.tuna.tsinghua.edu.cn/openeuler|g' \
    -e 's|^gpgkey=http://repo.openeuler.org|gpgkey=https://mirrors.tuna.tsinghua.edu.cn/openeuler|g' \
    -i.bak \
    /etc/yum.repos.d/openEuler.repo

RUN dnf update -y
RUN dnf install -y sudo
RUN echo "user ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# 创建用户
RUN useradd -rmg root user
USER user

# 安装coreutils依赖
RUN sudo dnf group install -y "Development Tools"
RUN sudo dnf install -y openssl-devel gmp-devel

RUN sudo dnf install -y llvm-libs-12.0.1-2.oe2203 llvm-devel-12.0.1-2.oe2203 clang-devel cmake curl

# 安装配置rust
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- --default-toolchain nightly-2022-08-08-x86_64-unknown-linux-gnu -y
ENV PATH=/home/user/.cargo/bin:$PATH
COPY --chown=user .cargo/config.toml /home/user/.cargo/config.toml

# 编译
COPY --chown=user . /home/user/binutils-rust
WORKDIR /home/user/binutils-rust
RUN cargo build --bins -Z sparse-registry

RUN test 13 -eq $(find ./target/debug -type f -executable -maxdepth 1 | wc -l)

# 测试
WORKDIR /home/user
RUN mkdir -p FunctionTest
WORKDIR /home/user/FunctionTest
RUN sudo dnf install texinfo wget expect -y
RUN wget https://ftp.gnu.org/gnu/dejagnu/dejagnu-1.6.tar.gz && tar -xzvf dejagnu-1.6.tar.gz
WORKDIR /home/user/FunctionTest/dejagnu-1.6
RUN ./configure && sudo make install -j
WORKDIR /home/user/FunctionTest
RUN wget https://ftp.gnu.org/gnu/binutils/binutils-2.37.tar.gz && tar -zxvf binutils-2.37.tar.gz && cp -r binutils-2.37 rust-binutils && mv binutils-2.37 binutils
WORKDIR /home/user/FunctionTest/binutils
RUN ./configure && make -j && make check -j 2>&1 > /home/user/FunctionTest/binutils-result.txt
WORKDIR /home/user/FunctionTest/rust-binutils
RUN ./configure && make -j
RUN cp -r /home/user/FunctionTest/rust-binutils/ld/ldscripts /home/user/binutils-rust/target/debug/
RUN sed -i '/if !\[info exists ld\] then {/i set ld /home/user/binutils-rust/target/debug/ldmain\n' ld/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists LD\] then {|set LD /home/user/binutils-rust/target/debug/ldmain\n&|' ld/testsuite/config/default.exp
RUN sed -i '/# Set LD_CLASS to "64bit" for a 64-bit \*host\* linker./i set REAL_LD /home/user/binutils-rust/target/debug/ldmain\n' ld/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists AS\] then {|set AS /home/user/binutils-rust/target/debug/as\n&|' gas/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists CXXFILT\] then {|set CXXFILT /home/user/binutils-rust/target/debug/cxxfilt\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists ELFEDIT\] then {|set ELFEDIT /home/user/binutils-rust/target/debug/elfedit\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists SIZE\] then {|set SIZE /home/user/binutils-rust/target/debug/size\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists NM\] then {|set NM /home/user/binutils-rust/target/debug/nm\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists READELF\] then {|set READELF /home/user/binutils-rust/target/debug/readelf\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists STRINGS\] then {|set STRINGS /home/user/binutils-rust/target/debug/strings\n&|' binutils/testsuite/config/default.exp
RUN sed -i 's|if !\[info exists ADDR2LINE\] then {|set STRIP /home/user/binutils-rust/target/debug/addr2line\n&|' binutils/testsuite/config/default.exp
RUN make check -j 2>&1 > /home/user/FunctionTest/rust-result.txt
COPY --chown=user diff.py /home/user/FunctionTest/diff.py