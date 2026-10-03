# Rust Magnifier

Lupa nativa para Windows escrita em **Rust 1.99.0**, usando diretamente a API de Magnificação do Windows (`Magnification.dll`) e Win32.

![Rust](https://img.shields.io/badge/Rust-1.99.0-000000?logo=rust)
![Windows](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows)
![License](https://img.shields.io/badge/license-MIT-green)

## Download

Os executáveis oficiais são gerados automaticamente pelo **GitHub Actions**.

Abra a seção **Releases** do repositório e baixe:

- `rust-magnifier.exe` — executável direto para Windows x64;
- `rust-magnifier-windows-x64.zip` — pacote contendo o executável, README e licença.

O workflow também disponibiliza o build mais recente como **Actions artifact** em cada execução na branch `main`.

## Recursos

- **Lente ao redor do mouse**: a área ampliada acompanha o cursor.
- **Tema dark** na janela principal, com barra de título escura no Windows 10/11.
- Lente padrão de **560 × 360**.
- Controles **Área da lente − / +** para ajustar a lente entre 320 e 960 px de largura.
- **Janela fixa**: mantém uma janela de ampliação enquanto a origem acompanha o mouse.
- Zoom de **1.0x a 8.0x**, em passos de 0.25x.
- Opção de **lente circular**.
- Janela da lente **não recebe cliques**, permitindo clicar normalmente no conteúdo por baixo.
- Suporte a **desktop virtual / múltiplos monitores**.
- A lente evita sair dos limites do desktop.
- Filtro para evitar o efeito de "espelho infinito" da própria janela ampliada.
- Executável release sem janela de console.
- Sem crates externos: apenas Rust + APIs nativas do Windows.

## Requisitos para compilar

- Windows 10 ou Windows 11.
- Rust **1.99.0** via `rustup`.
- Toolchain **64-bit MSVC** (`x86_64-pc-windows-msvc`).

## Executar em desenvolvimento

Execute:

```text
run_dev.bat
```

ou:

```powershell
cargo +1.99.0 run --target x86_64-pc-windows-msvc
```

## Gerar o EXE localmente

Execute:

```text
build_release.bat
```

O executável será criado em:

```text
build\x86_64-pc-windows-msvc\release\rust-magnifier.exe
```

## Atalhos globais

- `Ctrl + Alt + M`: iniciar/parar a lupa.
- `Ctrl + Alt + +`: aumentar o zoom.
- `Ctrl + Alt + -`: diminuir o zoom.

## GitHub Actions / Releases

O workflow `.github/workflows/release.yml`:

1. executa em `windows-latest`;
2. instala o Rust 1.99.0;
3. compila para `x86_64-pc-windows-msvc` em modo release;
4. publica o `.exe` e o `.zip` como artifact do Actions;
5. lê a versão diretamente de `Cargo.toml`;
6. cria ou atualiza automaticamente o GitHub Release `v<versão>` e anexa os binários.

Para publicar uma nova versão, altere o campo `version` em `Cargo.toml` e envie a alteração para `main`. O Actions cria a tag/release correspondente automaticamente.

## Notas da v0.1.3

- Corrigido possível deadlock ao ativar a lente: nenhuma chamada Win32/Magnification é feita mantendo o `Mutex` do estado bloqueado.
- A lente é mostrada antes de configurar a superfície de magnificação.
- Atualização em aproximadamente **30 FPS** para diminuir carga do DWM/GPU.
- Cache evita chamadas redundantes de `SetWindowPos` e `MagSetWindowSource`.
- Interface em paleta dark.
- Área padrão da lente aumentada para **560 × 360**.
- Janela fixa aumentada para **900 × 560**.
- Controles para aumentar/reduzir a área da lente sem alterar o fator de zoom.

## Licença

Este projeto é distribuído sob a licença **MIT**. Consulte [`LICENSE`](LICENSE).
