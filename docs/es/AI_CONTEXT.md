# Contexto para IA y mantenedores

## Propósito

Este documento proporciona el contexto mínimo necesario para continuar el desarrollo de **Ultimate Echo Collection** de forma segura y coherente.

Ultimate Echo Collection es un repositorio multiproyecto para compilar plugins de registro de Echo Fighters visuales para *Super Smash Bros. Ultimate*. El repositorio almacena código Rust, proyectos Cargo, workflows de GitHub Actions, documentación técnica y metadatos. No almacena mods visuales completos de terceros, archivos extraídos del juego, golden images privadas ni archivos `plugin.nro` generados dentro del historial normal de Git.

## Leer antes de modificar el proyecto

1. Leer el `README.md` de la raíz.
2. Leer `docs/es/ARCHITECTURE.md`.
3. Leer `docs/es/CREATE_NEW_ECHO.md`.
4. Leer `docs/es/TROUBLESHOOTING.md`.
5. Inspeccionar `template/plugin-source`.
6. Usar `projects/dry-bowser` como primera implementación funcional conocida.
7. Conservar una copia privada del último paquete validado en hardware antes de realizar cambios.
8. Preferir cambios mínimos y localizados en vez de reconstrucciones completas.

## Estructura del repositorio

```text
.github/workflows/
├── build-dry-bowser.yml
└── build-template.yml

template/
└── plugin-source/
    ├── src/lib.rs
    ├── Build.bat
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── ui_chara_db.xml
    └── ui_layout_db.xml

projects/
├── README.md
└── dry-bowser/
    ├── README.md
    ├── CREDITS.md
    └── plugin-source/
        ├── src/lib.rs
        ├── Build.bat
        ├── Cargo.lock
        ├── Cargo.toml
        ├── README.md
        ├── ui_chara_db.xml
        └── ui_layout_db.xml
```

## Concepto central

Un Echo Fighter visual registra una entrada de personaje nueva en la interfaz y la redirige hacia un único `fighter_kind` nativo.

```text
Nueva entrada de interfaz
        ↓
Un fighter_kind nativo
        ↓
Rango continuo de slots físicos marcados
        ↓
Assets locales correspondientes a esos slots
```

El `plugin.nro` registra y redirige la entrada. Los assets completos siguen perteneciendo al mod local ensamblado.

## Primera referencia funcional

La implementación de referencia es:

```text
projects/dry-bowser
```

Dry Bowser utiliza:

```text
ID de proyecto: dry-bowser
Personaje UI: ui_chara_koopadry
Personaje UI base: ui_chara_koopa
Fighter kind: fighter_kind_koopa
Directorio base: koopa
Marker: koopadry.marker
Slots físicos: c20-c28
Estilos detectados: 9
```

El proyecto fue compilado desde el repositorio reorganizado, validado mediante GitHub Actions y probado satisfactoriamente en Nintendo Switch.

La validación confirmó:

- inicio correcto del juego;
- aparición de una casilla CSS independiente;
- nueve estilos seleccionables;
- carga correcta del modo entrenamiento;
- carga correcta de modelos;
- funcionamiento de efectos personalizados;
- regreso correcto al CSS;
- inicio correcto de una segunda partida;
- carga correcta de resultados;
- ausencia de cierres inesperados.

El NRO final producido desde `main` fue idéntico bit a bit al NRO que ya había superado la prueba en hardware:

```text
SHA-256: 9F081883E36A42DB45F4C9B815EEB70CD11EF32D2C5797E4C9D2A38627F68F54
```

## Política de golden images

La primera versión funcional completa de Dry Bowser se conserva en una golden image privada fuera del repositorio. La golden image contiene el mod local completo, assets autorizados, markers, configuración, el `plugin.nro` probado, el código fuente exacto y notas de validación.

No sobrescribir la primera golden image al crear correcciones estéticas o versiones experimentales. No registrar en el repositorio rutas personales de nube, nombres de bóvedas, respaldos privados ni el archivo completo.

## Fuentes de verdad

Usar estas fuentes en el siguiente orden:

1. Proyecto estable dentro de `projects/<project-id>`.
2. Plantilla dentro de `template/plugin-source`.
3. Documentación y créditos del proyecto.
4. Golden image privada validada, cuando exista.
5. Artifact generado por GitHub Actions y su `SHA256SUMS.txt`.

La presencia de un archivo o su nombre no demuestra que su lógica o compatibilidad hayan sido validadas.

## Reglas de edición

- Conservar la última versión funcional antes de cambiar código.
- Realizar un cambio lógico a la vez.
- No reconstruir desde cero un proyecto funcional para corregir un detalle pequeño.
- Mantener genérica la plantilla.
- Mantener los identificadores específicos dentro del proyecto del personaje.
- No limpiar el código funcional de Dry Bowser sin una compilación y prueba separadas.
- No modificar silenciosamente markers, slots, IDs de UI, fighter kinds ni dependencias.
- Documentar honestamente problemas y limitaciones.

## Información obligatoria por proyecto

```text
ID de proyecto
Nombre visible
ID de personaje UI
ID de personaje UI base
Fighter kind base
Directorio base
Nombre del marker
Rango de slots físicos
Cantidad de estilos
Configuración de narración
Dependencias requeridas
Autor original de assets
Estado de compilación
Estado de validación en hardware
Problemas conocidos
```

## Archivos que no deben subirse

Salvo autorización explícita, no subir:

- mods visuales completos de terceros;
- archivos extraídos del juego;
- modelos, texturas, animaciones, sonidos, voces, efectos o UI propietarios;
- golden images privadas;
- `plugin.nro` dentro del historial normal;
- carpetas `target/` o `dist/`;
- archivos comprimidos y respaldos;
- credenciales, tokens, claves, sesiones o información personal.

## Estándar de compilación y validación

Una compilación verde no convierte automáticamente un proyecto en estable.

```text
Revisión del código
→ compilación en GitHub Actions
→ validación de identificadores del NRO
→ verificación SHA-256
→ integración en una copia local del mod
→ prueba en Nintendo Switch
→ golden image privada
→ incorporación estable a main
```

Dos NRO con el mismo SHA-256 son idénticos bit a bit.

## Problemas de presentación conocidos de Dry Bowser

La primera versión funcional conserva deliberadamente:

- nombre visible `Bowser Skelet`;
- título de Punch-Out `Le roi des os`;
- render VS desplazado;
- Final Smash con Giga Bowser normal.

Estos problemas no invalidan la referencia funcional. Deben corregirse mediante cambios separados y reversibles.

## Indicaciones específicas para una IA

1. No asumir que cualquier skin puede convertirse sin análisis.
2. Identificar primero el luchador nativo base y el `fighter_kind` real.
3. Inspeccionar el árbol completo antes de recomendar cambios de slots.
4. Diferenciar código fuente, assets locales y binarios generados.
5. Tratar entidades HTML escapadas como posibles artefactos de presentación antes de diagnosticar errores.
6. No inventar contenidos inaccesibles.
7. Sustentar conclusiones con archivos y constantes concretos.
8. Preservar IDs, rutas, geometría y secuencia de markers funcionales.
9. Solicitar únicamente el archivo específico que falte.
10. Usar Dry Bowser como referencia, no como prueba de que todos los personajes requieren los mismos valores.
