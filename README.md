# Toma

Editor de vídeo de Nodalix.

El motor de este repositorio es el código de [storytold/filmcraft](https://github.com/storytold/filmcraft) en `771b614ad4c3fbefda5d668ce4384504b8c13f6d`.
La licencia de ese código se conserva, igual que `NOTICE` y `docs/upstream-readme.md`.
Toma es el nombre del producto. El motor puede cambiar sin esperar a upstream.
Un sync posterior compara commits. No se sustituye este árbol con un reset.

## Compilar

```bash
cargo run -p filmcraft
```

El binario de escritorio se llama `toma`.

## Estado

La shell que entra en este import es la de upstream. La interfaz GTK 4 / libadwaita de Nodalix no está cableada todavía. El motor sí está en este repositorio, con sus tests.
