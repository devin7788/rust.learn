# rust.learn
learn rust
# windows安装rust
1.首先于Rust官网 获取rustup的安装器，下载名为"rustup-init.exe"的文件。<br>
2.依次输入"2"、回车、"x86_64-pc-windows-gnu"、回车、"nightly"、回车、"y"、回车，随后选项更新为 <br>
windows-gnu  <br>
toolchain: nightly <br> 
modify path: yes  <br>
3.随后输入"1"、回车即可开始安装最新版，等待一定时间后提示如下即表示Rust已经正常安装。<br>

# cargo config配置
1.C:\Users\Administrator\.cargo<br><br>
[source.crates-io]<br>
registry = "https://github.com/rust-lang/crates.io-index"<br>
replace-with = 'ustc'<br>
[source.ustc]<br>
registry = "git://mirrors.ustc.edu.cn/crates.io-index"<br>
[target.x86_64-pc-windows-gnu]<br>
linker = "C:\\msys64\\mingw64\\bin\\gcc.exe"<br>
ar = "C:\\msys64\\mingw64\\bin\\ar.exe"<br>
