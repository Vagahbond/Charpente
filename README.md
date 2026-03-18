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

## 🔑 Setup

1. Add Charpente to your flake's inputs
  a. Make sure Charpente follows your `nixpkgs` and/or `nix-darwin` inputs
```nix
    charpente = {
      url = "github:vagahbond/charpente";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nix-darwin.follows = "nix-darwin";
    };
```

2. Create a `hosts` folder and a `modules` folder.
  a. `hosts` is where you define your hosts
  b. `modules` is where you define pieces of configuration that your config uses.
3. In `hosts` create either a dir with a `default.nix` file or a `<hostname>.nix` file for each of your hosts.
  a. If you have a `hardware-configuration` and/or a `diskus` configuration, then it is easier to create a directory and put them in that alongside the `default.nix`.
4. In the `modules` dir, create your `modules` as you wish to make them. Each `module` is either a file or a folder with a `default.nix` file in it. the name of the module is filesystem-based.
  a. You can find examples of `modules` in the `modules` section.
```
.
├── hosts
│   ├── air.nix
│   └── platypute
│       └── default.nix
└── modules
    ├── home.nix
    └── virtualization
        └── libvirt.nix
```
5. Include `mkNixosSystems` and/or `mkDarwinSystems` in your configuration.
```nix
  outputs =
    {
      self,
      charpente,
      ...
    }@inputs:
    let
      # Generate configurations
      systems = charpente.lib.mkSystems {
        root = self;

        hosts = [
          "air"
          "framework"
          "live"
          "platypute"
        ];

        modules = { something = {}; somethingelse = [ "sometingelseelse" ];};

        extraArgs = {
          inherit
            inputs
            self
            ;
        };
      };
    in
    {
      # bind confiugrations
      nixosConfigurations = systems.nixosSystems;
      darwinConfigurations = systems.darwinSystems;
    };
```

## 👨‍💻 Integration
Charpente relies on a function found at `lib.mkSystems` in the flake. 

This functions does a bunch of actions that will generate your hosts's configuration. 

Here is how to call it : 
```nix
{
  outputs =
    {
      self,
      charpente,
      ...
    }@inputs:
    let
      systems = charpente.lib.mkSystems {
        # root is a way to send your current flake to Charpente so it can read your config
        root = self;

        # hosts is the list of hosts that you cant to take in account.
        hosts = [
          "air"
          "framework"
          "live"
          "platypute"
        ];

        # modules is a curated list of all the modules you want Charpente to take in account.
        modules = {

          # multi-file module 
          myFolder = [ "module1" "module 2" "module 3" ];

          # single-file module
          myFileModule = {};
        };  

        # Values that will be provided as arguments to your modules.
        extraArgs = {
          username = "vagahbond";
          inherit
            inputs
            self
            ;
        };
      };

    in
    {
      # Bind the generated systems to nixosConfigurations output
      nixosConfigurations = systems.nixosSystems;

      # Bind the generated systems to darwinConfigurations output
      darwinConfigurations = systems.darwinSystems;
    };

  # Imagine having no clean way to separate your system's dependencies...
  inputs = {
    # [...]
    charpente = {
      url = "github:vagahbond/charpente";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nix-darwin.follows = "nix-darwin";
    };
  };
}

```

* `charpente.lib.mkSystems` is a function that creates an AttrSet with :
  * `nixosSystems` containing your NixOS hosts
  * `darwinSystems` containing your Apple hosts
* `root` is a handle to your flake that you give to Charpente so it can check out your configuration.
* `hosts` is a list of hosts that Charpente will look for in your `hosts` directory. Any other host will be ignored. 
  * The string you put in this list will be matched to a `<name>.nix` file or a `name/` dir with a `default.nix` file inside, and load it with its configuration.
* `modules` is an `attrset` that declares the modules you want Charpente to look for. 
  * A singlefile module is declared as `filename = {};`
  * A multifile module is declared as `foldername = [ "filename1" "filename2" "filename3" ];`
  * Un-declared modules will be ignored
* `extraArgs` is an `attrset` of which each value will be passed as argument to all configurations. It is useful for creating global variables and whatnot.
* `extraModules` is an array of functions, in which you put modules that will be taken in account the same way as a `sharedModule` would, for ALL of your hosts.

## 🖥️ Hosts
A `host` represents one of your machine. It is declaring the existence of your host and some host-specific config.

### 📂 Architecture
A `host` can be a file or a directory with a `default.nix` file.

**Directory** :
A directory host can have any nix files they want aside the `default.nix`, and they will need to be imported by it (example further below). 
```
hosts
└── blade
    ├── default.nix
    ├── hardware-configuration.nix
    └── network.nix
```

**File** : 
A file host is one that does not need extra host-specific configuration files. 
Defining it in a single file makes the whole thing more clean.
```
hosts
└── macbook.nix
```

### 📄 Attributes

A host is defined the following way : 
```nix
# nh os switch --dry --build-host platypute --target-host platypute --hostname platypute . --show-trace
{
  name = "platypute";
  platform = "x86_64-linux";

  configuration = _: {

    imports = [
      ./hardware-configuration.nix
    ];
    system.stateVersion = "22.11"; # Did you read the comment?
  };
}
```
* `name` is going to be the `hostname` for your host. It will also be used to match with your `modules`'s targets.
* `platform` is the platform of your machine. If you do not know what it is, it is likely to be `x86_64-linux` or `aarch64-darwin`, but in doubt you can check in your auto-generated `hardware-configuration.nix` if you have one. *Note that I am not sure of the necessity of this one and it might just disappear*
* `configuration` is the host-specific configuration you want to apply to this host. It allows you to simply import your `hardware-configuration.nix` and other stuff you might want to apply specifically for your host.

## 🧱 Modules

A module is an `attrset` with pieces of configuration that you wish to bundle together for semantic reasons.

Below is an example of a module : 
```nix
{
  targets = [
    "air"
    "platypute"
    "framework"
  ];

  sharedConfiguration = { pkgs, inputs, ... }: {
    environment.systemPackages = [ nvf ];
  };

  nixosConfiguration = _: {
    environment.sessionVariables = {
      EDITOR = "nvim";
    };
  };

  darwinConfiguration = _: {
    environment.variables = {
      EDITOR = "nvim";
    };
  };
}
```

### ⚙️ Configurations
As of now, it has 3 different pieces of configuration in it : 
* `nixosConfiguration`: A piece of configuration only applied to NixOS hosts 
* `darwinConfiguration`: A piece of configuration only applied to nix-darwin hosts
* `sharedConfiguration`: A piece of configuration applied to both NixOS and nix-darwin

One piece of configuration is a function, that will be imported as a module. 
Its parameters are your usual module parameters, plus the ones you added via `extraArgs` when calling Charpente (learn more in `extraArgs section).

### 🎯 Targets
List the hosts you want to apply your module to in this `targets` attribute. 

The values you input there are compared to the attribute `name` written in your `hosts/myhost.nix` or `hosts/myhost/default.nix` file.

### 📄 Single page modules VS multi-page module
A module can be put in a folder or a single file. Either way, the content of your files will not change.

In the case of a single file module, you will have your `myModule.nix` file in the `modules` directory.

See :
```
modules
└── dev
    ├── db.nix
    ├── git.nix
    └── network.nix
```

In the case of a multi-files module, you will have a folder with a custom name in `modules` and it will contain various modules with their own names.

See :
```
modules
└── home.nix
```

# 🚗 Roadmap
This project was just created and I have ideas that I want to implement when I have the time. 
Some things will definitely happen, some I am not so sure. Also note that before we have a 1.0.0 version, breaking changes will happen here and there.

**Will do**
- [ ] Create a CLI to scaffold modules and hosts, and auto-generate arguments for mkSystems
- [ ] Rework the `hosts` part for a more relevant and flexible configuration from user

**May do**
- [ ] Optimize eval time
- [ ] Add "home-manager" as a third type of system (I do not use that and try to get rid of that so we will see)

# ❤️ Contributing
If you get interested in this project and want to create an issue, a PR, or a discussion, you are more than welcome. 

Rules will be added for PRs but for now, just make a good PR and it will be received and considered.
