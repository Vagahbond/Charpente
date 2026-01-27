# 🏗️ Charpente

## 📖 [Documentation](https://github.com/Vagahbond/Charpente/wiki)

## ❄️ [Example of usage](https://github.com/vagahbond/nix-config)

Charpente holds your config together so you can spend more time nixing and less time debugging imports !

A minimal and flexible layer of structure for your nix configuration : 
* Organize things in a cleaner way while keeping control over your repository and patterns.
* Module-based structure to separate pieces of config semantically
* Apply your modules to whichever hosts you want with a `targets` array.
* Declare your modules in separates parts for Darwin, Nixos, or for both of them.
* Never declare the same piece of config twice.
* Injectable for increased flexibility
* Per-host configuration for an easy and smooth integration of hardware-configuration.nix

Using nix is a love-letter to the idea of not doing the same thing twice... ever. 

With this in mind, it is hard to justify having the same piece of configuration written several time for several machine, is it not ? 

With Charpente, never write the same piece of config twice, and keep your modules in a standardized, pleasant way, while still being able to do your experimentations here and there.
Charpente will do all the heavy linking and you will spend more time writing your configuration than debugging your infinite recursions and broken nix imports.

