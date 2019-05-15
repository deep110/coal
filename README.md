# coal
Linux container like docker for learning purposes

### Setup

1. If you don't have rust installed, follow the instructions [here](https://www.rust-lang.org/tools/install).

2. Clone and build the repository
```shell
git clone https://github.com/deep110/coal.git
cd coal && cargo build
```

3. Download alpine file system and unzip into folder named **alpine**. We will use this as a base image.
```
wget http://dl-cdn.alpinelinux.org/alpine/v3.9/releases/x86_64/alpine-minirootfs-3.9.4-x86_64.tar.gz
mkdir alpine && tar -xzf alpine-minirootfs-3.9.4-x86_64.tar.gz -C alpine
```

### Running the container
If you are running from debug build:
```
sudo target/debug/coal run /bin/sh
```
In place of bash you can run any command that will be available in alpine by default.

### Reading Material / libc functions used
* [Fork](http://man7.org/linux/man-pages/man2/fork.2.html)
* [Clone](http://man7.org/linux/man-pages/man3/exec.3.html)
* [Chroot](http://man7.org/linux/man-pages/man2/chroot.2.html)
* [Mount](http://man7.org/linux/man-pages/man2/mount.2.html)
* [Clone](http://man7.org/linux/man-pages/man2/clone.2.html)


### TODO
1. Making an `exec` command like docker.
2. Mounting additional volumes and folders.