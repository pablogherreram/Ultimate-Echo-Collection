# Solución de problemas y lecciones aprendidas

## Propósito

Este documento registra fallos reales encontrados durante la creación, compilación, reorganización y validación del primer Echo Fighter funcional.

## 1. Conservar estado funcional

**Error:** continuar modificando la única copia funcional.

**Prevención:** mantener original, copia de trabajo y golden image privada. No sobrescribir la primera versión exitosa con correcciones visuales.

## 2. Separar fuente, assets y artifacts

```text
Repositorio fuente → compila plugin.nro
Mod local → contiene assets y plugin.nro
Golden image privada → conserva el estado probado
```

No mezclar los tres conceptos ni subir assets no autorizados.

## 3. No duplicar estilos existentes

Inspeccionar el árbol completo antes de crear slots. `c20-c28` puede representar nueve estilos legítimos. No crear otro bloque si ya existe el conjunto completo.

## 4. Problemas de markers

Síntomas:

- no aparece la casilla;
- aparecen menos estilos;
- slots posteriores son ignorados;
- el NRO compila pero no detecta el mod.

Revisar nombre exacto, mayúsculas, directorio base, ruta `model/body/cXX`, continuidad y cantidad total.

Un hueco puede truncar la secuencia detectada.

## 5. Placeholders restantes

Buscar:

```text
[CHARACTER]
CHARACTER.marker
fighter_kind_[CHARACTER]
ui_chara_[CHARACTER]
```

No deben existir en el proyecto específico. Sí deben permanecer en la plantilla genérica.

## 6. Entidades HTML

No diagnosticar código como roto solo porque muestre:

```text
&lt;  &gt;  &amp;  &quot;
```

Interpretar primero la versión normalizada.

## 7. Rama incorrecta en Actions

Error real:

```text
No such file or directory:
projects/dry-bowser/plugin-source
```

La ejecución manual se lanzó desde una rama que tenía el workflow, pero no el proyecto. Verificar siempre la rama seleccionada y que contenga workflow y fuente.

## 8. `working-directory` incorrecto

Alinear la ruta con la estructura:

```yaml
working-directory: projects/dry-bowser/plugin-source
```

```yaml
working-directory: template/plugin-source
```

## 9. Dependencia obsoleta

Los workflows sustituyen temporalmente:

```text
https://github.com/blu-dev/skyline-smash.git
```

por:

```text
https://github.com/ultimate-research/skyline-smash.git
```

y eliminan el lockfile solo dentro del runner. Esto no borra el archivo del repositorio.

## 10. Compilar no equivale a funcionar

Una Action verde no reemplaza la prueba en hardware. Verificar CSS, estilos, combate, modelo, efectos, regreso al CSS, segunda partida, resultados y estabilidad.

## 11. Tamaño del artifact

GitHub muestra el tamaño comprimido del artifact, no necesariamente el tamaño del NRO extraído.

## 12. SHA-256

```powershell
$hash1 = (Get-FileHash -LiteralPath $file1 -Algorithm SHA256).Hash
$hash2 = (Get-FileHash -LiteralPath $file2 -Algorithm SHA256).Hash
$hash1 -eq $hash2
```

`True` significa identidad bit a bit.

Hash final de Dry Bowser:

```text
9F081883E36A42DB45F4C9B815EEB70CD11EF32D2C5797E4C9D2A38627F68F54
```

## 13. Upload web no elimina archivos

La carga web agrega y reemplaza, pero no borra sobrantes. Eliminar manualmente archivos antiguos de raíz y workflows obsoletos.

## 14. Archivos ocultos omitidos

`.github`, `.gitignore` y `.gitattributes` pueden quedar fuera de una selección. Subirlos explícitamente y verificar su aparición.

## 15. Texto de interfaz infiltrado

Se copiaron accidentalmente textos como:

```text
Mostrar más líneas
Plain Text
Copiar
```

Revisar nombres de ramas y commits antes de confirmar. Un mensaje extraño no rompe el código, pero ensucia el historial.

## 16. PR dirigido al upstream

Verificar que la base sea el repositorio propio, no `BigBoss320/Ultimate-Auto-Slotting`. Una historia muy divergente puede mostrar `Can't automatically merge`.

## 17. Reemplazo seguro de rama

Secuencia usada con éxito:

```text
1. Validar la rama reorganizada.
2. Compilar plantilla y proyecto.
3. Probar en hardware.
4. Cambiar la rama predeterminada.
5. Renombrar main anterior a legacy-main.
6. Renombrar la reorganizada a main.
7. Compilar desde el nuevo main.
8. Comparar hashes.
9. Borrar ramas legacy.
```

## 18. Ejecución de PowerShell

Si los scripts están bloqueados:

```bat
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\Ruta\script.ps1"
```

El bypass aplica solo a ese proceso.

`Copy-Item -LiteralPath` no expande `*`. Usar:

```powershell
Get-ChildItem -LiteralPath $Source -Force |
    Copy-Item -Destination $Destination -Recurse -Force
```

o copiar archivos conocidos explícitamente.

## 19. No pegar scripts largos en consola interactiva

Puede causar ejecución parcial, errores de parser y here-strings sin cerrar. Entregar un `.ps1` completo y ejecutarlo con `-File`.

Un script de reorganización debe validar entradas, borrar únicamente la salida designada, copiar explícitamente, verificar hashes, rechazar binarios prohibidos e imprimir inventario final.

## 20. Layout DB es independiente

Dry Bowser funciona sin un layout propio activo, aunque el render VS esté desplazado. No confundir un defecto de presentación con fallo de registro o gameplay.

## 21. Final Smash es independiente

El paquete inicial no incluye Giga Dry Bowser por slot. No corregirlo modificando rutas globales sin comprobar que Bowser normal no resulte afectado.

## 22. Reporte de fallos recomendado

```text
ID de proyecto
Rama
Workflow y run
Paso fallido
Hash del artifact
Luchador base
Ruta del marker
Slots
Punto exacto del fallo
Mods conflictivos
Dependencias instaladas
Logs
```

Punto exacto en hardware:

```text
Inicio del juego
Carga del CSS
Cursor sobre casilla
Cambio de estilo
Confirmación
Selección de escenario
Carga de combate
Gameplay
Resultados
Regreso al CSS
Segunda partida
```


### 23. `chara_0` no llena la cuadrícula

La miniatura pequeña puede depender de `chara_7`. No renombrar directamente un BNTX `chara_0`; usar un contenedor `chara_7` real y conservar sus dimensiones y formato.

### 24. Renders VS y resultados desplazados

Ambas pantallas pueden usar `chara_3` con recortes distintos. Corregir primero escala y posición dentro del PNG maestro antes de activar Layout DB.

### 25. BNTX con artefactos

En Switch Toolbox usar la compresión de máxima calidad. No exportar un BNTX ya comprimido en modo rápido para luego recomprimirlo; volver al PNG maestro.

### 26. Serie personalizada no aparece

El BNTX no basta. Registrar `ui_series_<id>` y asignarlo mediante `CHARA_SERIES`. Si un JSON no sobreescribe una entrada creada por el plugin, registrar la serie y la asignación desde el NRO.

### 27. Narrador personalizado mudo

Comprobar `IS_CHARA_CALL = true`, coincidencia exacta de `YOUR_CHARACALL`, IDSP mono a 48 kHz y la ruta `<Mod>/vc_narration/`. La ruta `sound/bank/narration/` no funcionó en el flujo validado de Baller.

### 28. PowerShell y CMD mezclados

Los comandos con `$variable`, `Write-Host` y tipos `[System.*]` pertenecen a PowerShell. `dir /B`, `where /R` y ciertos bucles pertenecen a CMD. No pegar fragmentos de un shell en el otro. Para lotes compatibles, preferir el modo batch nativo de VGAudioCli.
