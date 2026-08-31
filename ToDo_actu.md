# FUTSAL MANAGER 27 — ToDo de actuación

> El seguimiento de pendientes reales y priorizados continúa en `ToDo_pendientes.md`. Este archivo conserva el histórico de hitos completados.

> Registro vivo de implementación. Actualizar al completar cada hito.

## Hito A — Base y seguimiento

- [x] Auditar el estado actual frente al PRD V2.
- [x] Crear `plan_actu.md` con el roadmap completo.
- [x] Crear este `ToDo_actu.md`.

## Hito B — Persistencia del partido

- [x] Guardar resultado del partido en SQLite al finalizar.
- [x] Guardar eventos del partido.
- [x] Actualizar estadísticas básicas de clasificación.
- [x] Evitar doble persistencia del mismo partido.
- [x] Verificar con `cargo test`.

## Hito C — Pantalla inicial y selección de club

- [x] Mantener agrupación por país y división.
- [x] Añadir buscador de clubes.
- [x] Añadir resumen visual del club seleccionado.
- [x] Añadir indicadores de reputación y división.
- [x] Mejorar responsive y navegación en catálogos grandes.

## Hito D — Resumen postpartido

- [x] Añadir API de consulta del último partido finalizado.
- [x] Mostrar marcador, eventos y estadísticas.
- [x] Mostrar impacto indirecto en clasificación mediante persistencia del resultado.
- [x] Añadir navegación desde el Dashboard.

## Hito E — Intervención durante el partido

- [x] Cambios manuales del equipo controlado.
- [x] Tiempos muertos por parte.
- [x] Cambios de formación y sliders de instrucciones en vivo.
- [x] Alertas de sexta falta, tarjetas, powerplay y fatiga en vivo.

## Hito F — Mercado profundo y scouting

- [x] Centro de ojeo inicial con conocimiento por club.
- [x] Asignación de scout por nación.
- [x] Rangos de CA/PA según conocimiento.
- [x] Progreso temporal de asignaciones.
- [x] Recomendaciones iniciales de scouting en la tabla.
- [x] Informes detallados ampliados por jugador.
- [x] Fortalezas, debilidades y atributos por rango.

- [x] Scouting por país mediante asignaciones.
- [x] Conocimiento progresivo y niebla de guerra básica.
- [x] Rangos de CA/PA y atributos.
- [x] Informes y recomendaciones.
- [x] Filtros, búsqueda y ordenación del centro de ojeo.
- [x] Renovaciones de contratos desde Plantilla.
- [x] Duración, salario, rol, cláusula y bonus contractuales.
- [x] Cesiones con duración y salario pactados.
- [x] Consulta de cesiones activas desde Mercado.
- [x] Devolución automática al vencer la cesión.
- [x] Formulario de oferta de cesión desde Mercado.
- [x] Jugadores libres y fichajes sin traspaso.
- [x] Mercado conectado con conocimiento parcial de scouting.

## Hito F.1 — Editor y datos generados

- [x] Confirmar que el Editor permite crear y modificar contratos indirectamente al asignar/liberar jugadores.
- [x] Añadir atributos contractuales y de cesión al modelo persistente.
- [x] Generar staff ficticio para cada club.
- [x] Generar preparadores físicos, técnicos, de porteros y analistas.
- [x] Generar jugadores y staff libres iniciales para pruebas.
- [x] Generar atributos realistas de táctica, gestión, ojeo, motivación, cantera y fisioterapia.
- [x] Exponer edición directa de contratos y cesiones desde el Editor.
- [x] Importar catálogo completo de países ISO y asociar automáticamente las banderas del directorio de recursos.
- [x] Mostrar banderas en perfiles y listados.
- [x] Importador CSV preparado para 250 países e ISO2/ISO3.
- [x] Añadir estructura para ISO2/ISO3, doble nacionalidad y posición secundaria.
- [x] Generar jugadores libres con doble nacionalidad y posición secundaria.

## Hito G — Desarrollo deportivo

- [x] Cantera U12-U20 con equipos, jugadores y desarrollo automático.
- [x] Promoción manual de juveniles U18+ al primer equipo.
- [x] Staff con efectos reales en entrenamiento, cantera, scouting y riesgo de lesiones.
- [x] Resumen visual de impacto del staff en Entrenamientos y Cantera.
- [x] Automatismos tácticos entrenables y configurables, aplicados contextualmente en simulación.
- [x] Dinámica de vestuario: moral, felicidad, química, cohesión, roles y capitanes.
- [x] Prueba de regresión para activación táctica en tramo final.
- [x] Moral, química, roles, compatibilidad, jerarquía, promesas y conversaciones iniciales.

## Hito H — Competiciones y carrera larga

- [x] Estructura de competiciones de liga y copa en la base de datos.
- [x] Generación inicial de eliminatorias de copa y vínculos partido/club.
- [x] Generación automática de calendarios de liga al crear el mundo.
- [x] Verificar que el calendario se puede simular desde la primera jornada.
- [x] Persistir posesión y estadísticas al simular jornadas automáticamente.
- [x] Dashboard y Calendario muestran estados de carga y error al consultar jornadas.
- [x] Próximo partido filtrado por fecha actual para evitar encuentros ya vencidos.
- [x] Clasificación con estados de carga y error consistentes.
- [x] Los partidos manuales no alteran la clasificación de copas ni competiciones no ligueras.
- [x] Resultados manuales y simulados comparten persistencia idempotente.
- [x] API y calendario distinguen jornadas de liga y rondas de copa.
- [x] Resultados de copa registran el club ganador de cada eliminatoria.
- [x] Avance automático de ganadores a la siguiente ronda.
- [x] Copas nacionales y supercopas con participantes nacionales y formato eliminatorio.
- [x] Supercopa basada en campeones históricos con fallback controlado.
- [x] Grupos, eliminatorias y playoffs: generación inicial de partidos de playoff.
- [x] No asignar ascensos directos cuando el playoff de ascenso siga pendiente.
- [x] Convertir los ganadores de playoff resueltos en ascendidos definitivos.
- [x] Convertir los ganadores de playoff resueltos en ascendidos definitivos.
- [x] Prórroga y penaltis con indicador visual en calendario y resumen.
- [x] Ascensos y descensos automáticos entre divisiones con historial de movimientos.
- [x] Mantener la inscripción real de equipos en la división siguiente al cambiar de temporada.
- [x] Exponer el historial de movimientos mediante API.
- [x] Añadir pantalla de historial de ascensos y descensos.
- [x] Base de selecciones, seleccionadores, convocatorias limitadas, ventanas y partidos internacionales iniciales.
- [ ] Torneos internacionales completos y clasificación.
- [ ] Historial, palmarés y récords.

## Hito I — Interfaz y experiencia visual

- [ ] Unificar tokens de color, tipografía, espaciado y estados.
- [ ] Crear componentes visuales reutilizables para paneles, métricas, tablas y estados vacíos.
- [ ] Rediseñar Dashboard con jerarquía de manager y acciones rápidas.
- [ ] Mejorar Plantilla y perfiles con ficha lateral, métricas y responsive.
- [ ] Mejorar Partido en vivo con HUD, eventos y estados de desempate.
- [ ] Crear bracket visual de copas.
- [ ] Pulir Mercado, Scouting, Cantera y Finanzas.
- [ ] Añadir fotografías de pabellones y fallbacks visuales.
- [ ] Revisar accesibilidad, navegación por teclado y contraste.
- [ ] Validar estados de carga, error y vacío.

## Hito J — Carrera y competiciones avanzadas

- [x] Supercopas y formatos configurables básicos.
- [ ] Fases de grupos y playoffs.
- [x] Ascensos y descensos automáticos entre divisiones con historial de movimientos.
- [x] Mantener la inscripción real de equipos en la división siguiente al cambiar de temporada.
- [x] Exponer el historial de movimientos mediante API.
- [x] Añadir pantalla de historial de ascensos y descensos.
- [x] Historial de temporadas y palmarés básico de competiciones.
- [x] Mostrar campeón vigente en el Calendario.
- [ ] Récords y estadísticas históricas avanzadas.
- [ ] Selecciones y torneos internacionales.
- [ ] Prórroga y penaltis plenamente conectados al motor en vivo y estadísticas.

## Hito K — Inmersión, guardado y producción

- [ ] Noticias, medios y reputación pública.
- [ ] Objetivos de directiva y conversaciones.
- [ ] Economía avanzada e instalaciones.
- [x] Guardado de múltiples slots y autoguardado seguro con copia SQLite independiente.
- [x] Carga/restauración de slots con reapertura segura de SQLite.
- [x] Backup manual de la partida activa.
- [x] Autoguardado automático tras avanzar día o semana.
- [x] Confirmación antes de cargar una partida.
- [ ] Tutorial y onboarding.
- [ ] Benchmarks, optimización y carga bajo demanda.
- [ ] Beta testing, accesibilidad final y distribución.

## Hito I — Inmersión y producción

- [ ] Noticias y medios.
- [ ] Objetivos de directiva.
- [ ] Economía avanzada e instalaciones.
- [ ] Guardado/carga de múltiples partidas y backups.
- [ ] Benchmarks, optimización y virtualización.
- [ ] Tutorial, accesibilidad y pulido visual.

## Hito K.1 — Guardado seguro

- [x] Migración de metadatos de slots.
- [x] Copia atómica de la base SQLite para slots y autoguardado.
- [x] API y pantalla de gestión de slots.
- [x] Restauración de un slot y backup manual externo.
- [x] Autoguardado automático y confirmación de carga.

## Estado de la última verificación

- [x] `pnpm run build` (typecheck + Vite).
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` — 12 tests correctos.
- [x] Build verificado tras añadir módulo y pantalla de scouting.
- [x] Build y tests verificados tras añadir contratos avanzados.
- [x] Dinámica de vestuario ampliada y validada: 14 tests Rust correctos.
- [x] Formulario configurable de renovación con preview salarial.
- [x] Listado y fichaje de jugadores libres desde Mercado.
- [x] Staff ficticio generado con cuatro perfiles por club y atributos.
- [x] Catálogo CSV de 250 países importado con ISO2/ISO3 y banderas empaquetadas.
- [x] Jugadores y staff libres iniciales generados para pruebas.
- [x] Doble nacionalidad y posiciones principal/secundaria expuestas en Editor y APIs.
- [x] Banderas conectadas al modelo de naciones y disponibles para perfiles/listados.
- [x] Mostrar banderas en todos los componentes de perfil detallado de staff y plantilla.
- [x] API de ofertas de cesión preparada para clubes controlados.
- [x] Snapshot en vivo ampliado con tarjetas, powerplay y tiempos muertos.
- [x] Controles tácticos en vivo conectados al motor.
- [x] Pantalla postpartido implementada en el Dashboard.
- [x] Editor con pestaña de contratos y cesiones, fechas, salarios, roles y estado activo.
- [x] Editor restringido a uso fuera de partidas activas.
- [x] CRUD de pabellones con capacidad, ciudad, club y foto.
- [x] Edición de datos económicos de clubes desde la ficha del Editor.
- [x] Crear `plan_interfaz.md` con la dirección visual inspirada en `Imagenes_Ejemplo/`.
- [x] Consolidar en este documento los nuevos hitos visuales, competitivos y de producción.
- [x] Aplicar progresivamente `plan_interfaz.md` a todas las pantallas.
- [x] Aplicar los componentes compartidos al Calendario y sus estados vacíos.
- [x] Aplicar los componentes compartidos a Plantilla, contratos y estados de error.
- [x] Aplicar los componentes compartidos a Mercado y Scouting.
- [x] Aplicar los componentes compartidos a Cantera y Finanzas.
- [x] Añadir navegación global responsive con menú móvil y agrupación de secciones.
- [x] Aplicar la nueva capa visual al Partido en vivo, marcador, campo y eventos.
- [x] Aplicar paneles visuales al Editor y sus formularios principales.
- [x] Revisar accesibilidad y navegación por teclado del Editor.
- [x] Añadir iconos visuales reutilizables sin dependencias nuevas.
- [x] Añadir etiquetas ARIA básicas al Shell y menú móvil.
- [ ] Añadir accesibilidad completa y navegación por teclado al Shell.
- [x] Añadir métricas de cantera, promoción y economía.
- [x] Añadir estados de carga, error y vacío consistentes.
- [x] Aplicar la nueva paleta visual base al tema global y Dashboard.
- [x] Crear componentes UI reutilizables para paneles, métricas y badges.
- [x] Crear bracket visual de copas y completar indicadores de desempate en todos los resúmenes.
- [x] CRUD de pabellones con asociación a clubes y soporte de fotografía.
- [x] Cantera conectada a la navegación, progreso diario y promociones.
- [x] Atributos del staff aplicados al rendimiento de sistemas deportivos.
- [x] IA táctica reactiva: cambios de plan por marcador, tiempo, faltas y fatiga en equipos controlados por la máquina.
- [x] Estilos tácticos persistentes por club (contragolpe, posesión, presión alta y bloque bajo) aplicados al motor.
- [x] Plan inicial contextual de la IA según calidad relativa, localía y tipo de competición.
- [x] Editor visual para configurar estilo, formación, ritmo, presión, bloque y amplitud de cada club.

## Hito L — Contratos avanzados

- [x] Bonus de aparición y portería imbatida liquidados de forma idempotente por contrato y partido.
- [x] Bonus integrados en simulación automática y partidos en vivo.
- [x] Renovaciones con validación de importes y transacción atómica.
- [x] Precontratos persistentes, consultables desde Mercado y activados automáticamente al vencimiento.
- [x] API Tauri y tipos frontend para consultar y crear precontratos.
- [x] Verificación: `cargo test --lib` (40 tests), `cargo check`, `pnpm run build` y `git diff --check`.

## Verificación de cada hito

- `pnpm run build` / typecheck.
- `cargo test --manifest-path src-tauri/Cargo.toml`.
- Revisión manual del flujo afectado.
- Actualización de `plan_actu.md` y este archivo.
