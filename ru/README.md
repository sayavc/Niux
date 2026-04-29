# Niux
[English](../README.md) | **Русский**

<img align='right' src='../assets/icon.png' width='260px' alt="niux logo"/>


Декларативный менеджер пакетов NixOS/home-manager, написанный на Rust.
Устали каждый раз редактировать configuration.nix чтобы установить пакет? Niux позволяет управлять пакетами короткими командами.

[![Release](https://img.shields.io/github/v/release/sayavc/niux?label=Release&logo=github&color=7aa2f7)](https://github.com/sayavc/niux/releases/latest) 
[![License](https://img.shields.io/badge/License-GPL%20v3-bb9af7.svg?logo=gnu)](../LICENSE) 
[![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org)
## Зачем Niux?

Работать с `home-manager` и `NixOS` мощно, но постоянно редактировать конфиги и запускать `switch` — утомительно.

**Niux** делает это простым и быстрым: лёгкий CLI, который позволяет устанавливать, удалять и управлять пакетами **декларативно** с короткими командами — как `apt` или `pacman`, но без нарушения декларативной философии Nix.

- Установка одной командой (`niux -Hi firefox`)
- Автоматическая пересборка конфигов при необходимости
- Обновление флейков, очистка системы и многое другое
- Написан на Rust — быстро, надёжно и безопасно

Коротко: Niux привносит удобство традиционных менеджеров пакетов в NixOS и home-manager, оставаясь полностью декларативным.

## Возможности

- Быстрый и лёгкий интерфейс командной строки
- Управление пакетами home и system декларативно
- Написан на Rust для производительности и надёжности
- Простой и понятный синтаксис команд
- Поддержка standalone и module home-manager
- Поддержка NixOS с флейками и без
- Хуки которые позволяют автоматизировать действия
- Автодополнение как в Pacman и apt 

## Как это работает

Niux управляет пакетами, напрямую редактируя твои nix конфиг файлы.
Можно использовать маркеры по умолчанию или добавить свои.
Если маркеры указаны неверно — Niux сообщит об этом.

```nix
home.packages = [
  # niux-home
  firefox
  vim
  # niux-home-end
];
```

Когда ты запускаешь `niux -Hi firefox` — пакет вставляется после начального маркера.
Когда запускаешь `niux -Hr firefox` — пакет удаляется, но только между маркерами, так что остальной конфиг никогда не затрагивается.

> Маркеры настраиваются через конфиг

## Установка

## flakes (standalone home-manager)

Добавьте в `flake.nix`:

```nix
inputs.niux = {
    url = "github:sayavc/niux";
    inputs.nixpkgs.follows = "nixpkgs";
};
```

Передать niux в home-manager через extraSpecialArgs:

```nix 
outputs = inputs@{ nixpkgs, home-manager, niux, ... }: {
homeConfigurations.youruser = home-manager.lib.homeManagerConfiguration {
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    extraSpecialArgs = { inputs = { inherit niux; }; };
    modules = [ ./home/home.nix ];
   };
};
```

Добавить в home.nix:

```nix
{ inputs, pkgs, ... }: {
    home.packages = [
        inputs.niux.packages.${pkgs.system}.default
    ];
}
```

Запустите `home-manager switch` для применения.

## flakes (module home-manager)

Добавьте в `flake.nix`:

```nix
inputs.niux = {
    url = "github:sayavc/niux";
    inputs.nixpkgs.follows = "nixpkgs";
};
```

Передать в home-manager:

```nix
outputs = inputs@{ self, nixpkgs, home-manager, niux, ... }: {
  nixosConfigurations.yourhostname = nixpkgs.lib.nixosSystem {
    system = "x86_64-linux";
    modules = [
      ./configuration.nix
      home-manager.nixosModules.home-manager
      {
        home-manager.useGlobalPkgs = true;
        home-manager.useUserPackages = true;
        home-manager.extraSpecialArgs = { inputs = { inherit niux; }; };
        home-manager.users.yourusername = import ./home.nix;
      }
    ];
  };
};
```

Добавить в `home.nix`:
```nix
# home.nix
{ inputs, pkgs, ... }: {
  home.packages = [
    inputs.niux.packages.${pkgs.system}.default
  ];
}
```

> **Примечание** Установка без flakes появятся позже.
> Контрибуции приветствуются!

## Конфигурация

Сгенерируйте конфиг по умолчанию:
```bash
niux --gen-config
```

Или по своему пути:
```bash
niux --config /my/path/niux.kdl
niux --gen-config
```
Или для хуков:
```bash
niux --hook-config /my/path/niux.kdl
niux --gen-config
```

Показать текущий путь:
```bash
niux --show-path
```

> **Примечание:** После `--default-path-config` и `--default-hook-path-config` необходим `--gen-config`

## Использование

### Быстрый старт
```bash
niux -Hi firefox        # Установить firefox в home
niux -Si vim            # Установить vim в system
niux -Hr firefox        # Удалить firefox из home
niux -Hl                # Список пакетов home
niux -l firefox         # Поиск везде
niux -U                 # Обновить все флейки
niux -USHa              # Обновить + пересобрать всё
niux -HSa               # Пересобрать оба конфига
```
Смотрите [English README](../README.md) для всех комбинаций   


## Справка по командам

| Флаг | Описание |
|------|----------|
| `-H` - Home | Цель — пакеты home |
| `-S` - System | Цель — пакеты system |
| `-i` - install(add) | Установить пакеты |
| `-r` - remove | Удалить пакеты |
| `-a` - apply(rebuild) | Пересобрать конфиг |
| `-l` - list | Список или поиск пакетов |
| `-U` - Update | Обновить флейки |
| `--gen-config` | Сгенерировать конфигурацию |
| `--config` | Указать свой путь к конфигу |
| `--hook-config` | Указать путь к конфигу хуков |
| `--show-path`| Получить пути конфигов | 
| `--clear` | Очистка мусора |
| `--search`| Поиск|

## Вклад в проект

Вклад приветствуется! Открывайте issue или присылайте pull request.

## Лицензия

Проект лицензирован под GNU General Public License v3.0 — см. файл [LICENSE](../LICENSE).

## Автор

Создано [sayavc](https://github.com/sayavc)
