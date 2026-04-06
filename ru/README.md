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

## Требования
- NixOS

## Установка

## flakes (home-manager)

Добавьте в `flake.nix`:

```nix
inputs.niux = {
    url = "github:sayavc/niux";
    inputs.nixpkgs.follows = "nixpkgs";
};
```

Передать niux в home-manager через extraSpecialArgs:

```nix 
homeConfigurations.youruser = home-manager.lib.homeManagerConfiguration {
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    extraSpecialArgs = { inputs = { inherit niux; }; };
    modules = [ ./home/home.nix ];
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
> **Примечание** Установка без flakes и с модульным home-manager появятся позже.
> Контрибуции приветствуются!
## Конфигурация

Сгенерируйте конфиг по умолчанию:
```bash
niux --gen-config
```

Или по своему пути:
```bash
niux --gen-config --default-path-config ~/my/path/niux.kdl
```

Показать текущий путь:
```bash
niux --get-current-path
```

> **Примечание:** `--default-path-config` требует существующего `.kdl` файла. Всегда сначала запускайте `--gen-config`.

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
| `-H, --home` | Цель — пакеты home |
| `-S, --system` | Цель — пакеты system |
| `-i, --install` | Установить пакеты |
| `-r, --remove` | Удалить пакеты |
| `-a, --apply` | Применить и пересобрать конфиг |
| `-l, --list` | Список или поиск пакетов |
| `-U, --update` | Обновить флейки |
| `--gen-config` | Сгенерировать конфигурацию |
| `--default-path-config` | Указать свой путь к конфигу |
| `--clear` | Очистка мусора |

## Вклад в проект

Вклад приветствуется! Открывайте issue или присылайте pull request.

## Лицензия

Проект лицензирован под GNU General Public License v3.0 — см. файл [LICENSE](../LICENSE).

## Автор

Создано [sayavc](https://github.com/sayavc)
