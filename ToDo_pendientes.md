# FUTSAL MANAGER 27 — Pendientes reales del PRD

> Documento de control para los siguientes hitos. Solo incluye trabajo pendiente o parcialmente implementado.

## Prioridad 1 — Profundidad deportiva

- [x] Moral y felicidad conectadas a resultados y dinámica semanal.
- [ ] Confianza y satisfacción avanzada; las promesas individuales ya disponen de persistencia, validación y evaluación inicial.

> Progreso actual: moral, felicidad, química, cohesión, roles base y capitanes ya están implementados y validados.
- [x] Química y cohesión persistidas con evolución semanal.
- [x] Modelo persistente de compatibilidad individual preparado.
- [x] Cálculo y evolución avanzada de compatibilidad entre jugadores.
- [x] Roles base de plantilla persistidos desde el contrato.
- [x] Capitanes y vicecapitanes configurables desde el Editor.
- [x] Jerarquía automática, capitanes y promesas contractuales avanzadas.
- [x] Conversaciones con jugadores y efectos iniciales sobre moral.
- [ ] Automatismos tácticos avanzados por contexto y rival.
- [ ] IA táctica reactiva de los equipos controlados por la máquina.

## Prioridad 2 — Competiciones completas

- [ ] Motor genérico de formatos configurables desde el Editor (parcial: configuración base de grupos persistida).
- [x] Fases de grupos: grupos, inscripciones, calendario round-robin, clasificación específica, actualización de resultados y generación inicial de cruces de clasificados validada con tests.
- [x] Estructura de eliminatorias a doble partido y marcador global inicial.
- [ ] Desempates por enfrentamiento directo.
- [ ] Reglas específicas de clasificación por competición.
- [ ] Calendarios que eviten solapamientos y saturación de partidos.
- [ ] Prórroga y penaltis plenamente integrados en todos los flujos.

## Prioridad 3 — Selecciones nacionales

- [ ] Plantillas de selecciones nacionales.
- [ ] Convocatorias y elegibilidad por nacionalidad.
- [ ] Seleccionadores y staff internacional.
- [ ] Ventanas internacionales.
- [ ] Clasificaciones y fases de clasificación.
- [ ] Mundial, torneos continentales y competiciones internacionales.
- [ ] Partidos de selecciones integrados en el calendario.

## Prioridad 4 — Historial y estadísticas

- [ ] Estadísticas individuales por partido y temporada.
- [ ] Goles, asistencias, tarjetas y minutos.
- [ ] Porterías imbatidas y estadísticas de porteros.
- [ ] Máximos goleadores y asistentes.
- [ ] Récords de clubes y competiciones.
- [ ] Evolución histórica de CA, PA y atributos.
- [ ] Historial completo de jugadores y clubes.
- [ ] Gráficos comparativos y filtros por temporada.

## Prioridad 5 — Inmersión y dirección

- [ ] Noticias generadas por eventos del mundo.
- [ ] Rumores, medios y reputación pública.
- [ ] Ruedas de prensa y respuestas del entrenador.
- [ ] Objetivos de directiva.
- [ ] Confianza de la directiva.
- [ ] Reuniones y conversaciones con la directiva.
- [ ] Renovación o despido del entrenador.
- [ ] Logros y hitos de carrera.

## Prioridad 6 — Economía avanzada

- [ ] Viajes y costes logísticos.
- [ ] Mantenimiento del pabellón.
- [ ] Costes detallados de personal.
- [ ] Merchandising.
- [ ] Derechos televisivos.
- [ ] Patrocinadores negociables.
- [ ] Mejoras de instalaciones.
- [ ] Demanda dinámica y precios de entradas.
- [ ] Repercusión económica de resultados y reputación.

## Prioridad 7 — Editor completo

- [ ] Configuración detallada de formatos de competición.
- [ ] Reglas de ascenso, descenso y playoff editables.
- [ ] Edición de fases y calendario.
- [ ] Edición completa de selecciones y convocatorias.
- [ ] Edición avanzada de instalaciones y economía.
- [ ] Validación de datos antes de iniciar una partida.
- [ ] Importación/exportación de bases de datos.

## Prioridad 8 — UX y accesibilidad

- [ ] Navegación completa por teclado.
- [ ] Gestión de foco en menús y diálogos.
- [ ] Contraste y estados de foco revisados.
- [ ] Mensajería global no intrusiva.
- [ ] Atajos de teclado.
- [ ] Soporte mejorado para lectores de pantalla.
- [ ] Tutorial y onboarding inicial.
- [ ] Ayuda contextual dentro del juego.

## Prioridad 9 — Rendimiento y distribución

- [ ] Benchmark de generación del mundo.
- [ ] Benchmark de simulación de temporada.
- [ ] Reducción de consultas repetidas a SQLite.
- [ ] Carga bajo demanda de pantallas y datos.
- [ ] Virtualización de listados grandes.
- [ ] Optimización del bundle frontend.
- [ ] Pruebas de memoria y estabilidad.
- [ ] Empaquetado instalable.
- [ ] Sistema de actualización.
- [ ] Beta testing y corrección de errores.

## Ya implementado y fuera de este documento

- Mundo inicial, naciones, ISO y banderas.
- Clubes, jugadores, staff y agentes libres ficticios.
- Posiciones principales/secundarias y doble nacionalidad.
- Contratos, renovaciones, bonus, cláusulas y cesiones básicas.
- Mercado, scouting, conocimiento parcial e informes.
- Cantera U12-U20 y promoción juvenil.
- Entrenamiento y progresión básica.
- Motor de partido 2D con tácticas, cambios, tiempos muertos, faltas, tarjetas, powerplay, prórroga y penaltis.
- Ligas con equipos, calendarios ida/vuelta y simulación diaria/semanal.
- Copas nacionales, supercopas y playoffs básicos.
- Ascensos y descensos automáticos con historial.
- Palmarés básico y campeón vigente.
- Editor de clubes, jugadores, staff, contratos, países, competiciones y pabellones.
- Guardado, carga, autoguardado y backups.
- Interfaz responsive, componentes visuales compartidos y estados básicos.
- Automatismos tácticos entrenables y activación contextual inicial.

## Regla de seguimiento

Cada hito debe:

1. Actualizar este documento.
2. Actualizar `ToDo_actu.md` si cambia el alcance.
3. Incluir pruebas automatizadas cuando sea posible.
4. Verificar `pnpm run build`.
5. Verificar `cargo test --manifest-path src-tauri/Cargo.toml --lib`.
6. Publicarse en `FutsalManager-27_V2` cuando esté validado.
