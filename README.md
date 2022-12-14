# lxd-containers

Configuriing LXD containers for developlemnt in different purposes, that will have all tool required for that jjob ready to go. 
Configuring the LXD container should be done using some package managment utilities such as Nix packages or Ansible. This will allow configuring a native envirnment similarly.

## Pacakge managment tools

- NixOS
- Ansible
- Distribution based package managers:
    - `apt`
    - `pacman`

## Plan

The plan is to have  an automatic container generator, which has the package manament utility configured to setup the tools necessary for the container enviorment.

Container is launch using the host shell configuration and adds the extra PATHs required for the extra tools.


## Progress

- Using Nix-Shell purely seems it could set the paths to the setup tools, while still being able to use system wide binaries.


## Objective

- Run all desirable RISC-V repositories without any configuration overhead. At least not recurrent. Setup a list of configurations that can setup an environment with all the necessy tools. That can be achieved using a native env like Nix or a virtual env like LXD or Docker.

Projects of interest:
- [Riscv-mini](https://github.com/ucb-bar/riscv-mini)
- [Rocket Chip](https://github.com/chipsalliance/rocket-chip)
- [RV Tests](https://github.com/riscv-software-src/riscv-tests)
- [Ariane](https://github.com/openhwgroup/cva6)
- [CV32E40P](https://github.com/openhwgroup/cv32e40p)
- [OpenPiton](https://github.com/PrincetonUniversity/openpiton)


Tools:
- [RV toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain)
- [Spike RISC-V ISA Simulator](https://github.com/riscv-software-src/riscv-isa-sim)
- [RISC-V Tools](https://github.com/riscv-software-src/riscv-tools)

Environments:
- [Nix](https://nixos.org/)
    - [Nix Templates](https://github.com/nix-dot-dev/getting-started-nix-template)
    - [Nix Manual](https://nixos.org/manual/nix/stable/language/index.html)
    - [Declarative and Reproducible](https://nixos.org/guides/declarative-and-reproducible-developer-environments.html#declarative-reproducible-envs)
- LXD
    - Bind mount `\home\<user>` and translate UID to <user> such that full user access is given. Furthermore, make use of *dotfiles* to conserve user default working environment.
- Docker
