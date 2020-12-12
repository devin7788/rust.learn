# rust.learn
learn rust
# windows安装rust
1.首先于Rust官网 获取rustup的安装器，下载名为"rustup-init.exe"的文件。
2.依次输入"2"、回车、"x86_64-pc-windows-gnu"、回车、"nightly"、回车、"y"、回车，随后选项更新为
windows-gnu
toolchain: nightly
modify path: yes
3.随后输入"1"、回车即可开始安装最新版，等待一定时间后提示如下即表示Rust已经正常安装。

// 1.配置信息
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
[http]
proxy = "127.0.0.1:64582"
[https]
proxy = "127.0.0.1:64582"

[target.x86_64-pc-windows-gnu]
linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"
ar = "C:\\msys64\\mingw64\\bin\\ar.exe"

2.安装racer
rustup toolchain add nightly
cargo +nightly install racer

3.msys安装
编辑 /etc/pacman.d/mirrorlist.mingw32 ，在文件开头添加：
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/mingw/i686
编辑 /etc/pacman.d/mirrorlist.mingw64 ，在文件开头添加：
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/mingw/x86_64
编辑 /etc/pacman.d/mirrorlist.msys ，在文件开头添加：
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/msys/$arch

pacman -Sy
pacman -Ss gcc
pacman -S msys/gcc
pacman -Ss make
pacman -S msys/make

mirrorlist.mingw64文件添加一行，Server = http://mirrors.ustc.edu.cn/msys2/mingw/x86_64/
pacman -S gcc
pacman -S mingw-w64-x86_64-toolchain
pacman -S mingw-w64-i686-toolchain
pacman -S mingw-w64-x86_64-SDL2
pacman -S mingw-w64-i686-SDL2
pacman -S base-devel
pacman -S vim
pacman -S yasm
pacman -S nasm
pacman -S make


pacman-key --init
pacman -Syu
pacman -S mingw-w64-x86_64-cmake mingw-w64-x86_64-extra-cmake-modules
pacman -S mingw-w64-x86_64-make
pacman -S mingw-w64-x86_64-gdb
pacman -S mingw-w64-x86_64-toolchain

设置路径, 修复找不到gcc.exe的问题
C:\msys64\mingw64\bin
