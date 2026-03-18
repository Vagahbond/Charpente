# Charpente

Find documentation [here](https://github.com/vagahbond/charpente)

## 🏗️ What is Charpente?

Charpente holds your config together so you can spend more time nixing and less time debugging imports !

* A minimal and flexible layer of structure for your nix configuration : 
* Organize things in a cleaner way while keeping control over your repository and patterns.
* Module-based structure to separate pieces of config semantically
* Apply your modules to whichever hosts you want with a targetsarray.
* Declare your modules in separates parts for Darwin, Nixos, or for both of them.
* Never declare the same piece of config twice.
* Injectable for increased flexibility
* Per-host confoguration for an easy and smooth integration of hardware-configuration.nix

Using nix is a love-letter to the idea of not doing the same thing twice... ever. 

With this in mind, it is hard to justify having the same piece of configuration written several time for several machine, is it not ? 

With Charpente, never write the same piece of config twice, and keep your modules in a standardized, pleasant way, while still being able to do your experimentations here and there.
Charpente will do all the heavy linking and you will spend more time writing your configuration than debugging your infinite recursions and broken nix imports.

## 📂 Example file structure

Below is an example of what your nix configuration could look like if you used Charpente : 
```
.
├── charpenteModules.nix
├── flake.lock
├── flake.nix
├── hosts
│   ├── air.nix
│   └── platypute
│       ├── default.nix
│       ├── disk-config.nix
│       └── hardware-configuration.nix
└── modules
    ├── home.nix
    ├── impermanence.nix
    ├── locales.nix
    ├── network
    │   ├── ssh.nix
    │   ├── vpn.nix
    │   └── wifi.nix
    ├── nix
    │   ├── live.nix
    │   ├── nix.nix
    │   └── remoteBuild.nix
    ├── services
    │   ├── blog.nix
    │   ├── ssh.nix
    │   └── vaultwarden.nix
    ├── system.nix
    ├── user.nix
    └── virtualization
        ├── docker.nix
        ├── kubernetes.nix
        └── libvirt.nix
```
