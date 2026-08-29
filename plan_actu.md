# FUTSAL MANAGER 27 — Plan de actuación

## Objetivo

Convertir el prototipo jugable actual en un manager de futsal profundo, coherente y distribuible, conectando simulación, gestión, competición, economía e interfaz.

## Estado inicial auditado

### Implementado

- Mundo procedural amplio: naciones, confederaciones, clubes, divisiones y miles de jugadores.
- Calendarios de liga ida/vuelta, clasificación y avance diario/semanal; generación inicial corregida para incluir solo ligas de clubes inscritas, verificada desde la primera jornada y con estadísticas básicas persistidas.
- Motor 2D Rust con campo 40×20, 2×20 minutos, fatiga, cambios, faltas acumuladas, doble penalti y powerplay.
- Tácticas prepartido: formaciones 3-1, 4-0, 2-2, 5-0, tempo, presión, línea defensiva, amplitud y quintero.
- Mercado, ofertas bidireccionales, valoración y bandeja de entrada.
- Entrenamiento, progresión, lesiones y sanciones básicas.
- Finanzas básicas: salarios, patrocinio, taquilla, premios y presupuestos.
- Rollover de temporada, retiradas y regeneración juvenil simplificada.
- Editor de países, clubes, jugadores, staff y competiciones.

### Pendiente o incompleto respecto al PRD

- Guardado, carga, autoguardado automático y backup manual implementados con copias SQLite seguras.
- Scouting real con scouts limitados, asignaciones, informes y niebla de guerra.
- Cantera U12-U20 gestionable, promociones y desarrollo juvenil.
- Copas nacionales, supercopas, grupos, eliminatorias y playoffs; copas nacionales y supercopas básicas ya generan participantes, quedando pendientes formatos avanzados.
- Ascensos y descensos entre divisiones, con reglas configurables e historial de movimientos.
- Sistema completo de selecciones nacionales y calendarios internacionales.
- Moral, felicidad, confianza, química, roles y conversaciones con jugadores.
- Contratos avanzados, renovaciones, cláusulas, bonus, cesiones y jugadores libres.
- Gestión funcional del staff y efecto de sus atributos.
- Tácticas durante el partido, cambios manuales y tiempos muertos.
- Automatismos tácticos entrenables, configurables y aplicados contextualmente durante la simulación.
- IA táctica reactiva y estilos persistentes por club.
- Estadísticas individuales, análisis postpartido, crónicas y gráficos.
- Historial de temporadas, palmarés, récords y evolución de jugadores.
- Noticias, medios, rumores y reputación pública.
- Economía avanzada: viajes, personal, instalaciones, merchandising y TV.
- Optimización medida: benchmarks, virtualización, cache, LOD y carga bajo demanda.
- Tutorial, objetivos de directiva, logros y pulido de errores/estados vacíos.

## Priorización

### Fase 0 — Robustez y calidad de uso

1. Guardado/carga/autoguardado y recuperación.
2. Manejo de errores y estados vacíos sin `catch` silenciosos.
3. Refresco consistente de pantallas tras avanzar, fichar, entrenar o terminar un partido.
4. Tests de regresión y benchmarks básicos.

### Fase 1 — Partido realmente interactivo

1. Persistir el resultado y eventos del partido en SQLite al finalizar.
2. Pantalla postpartido con estadísticas de equipos y jugadores.
3. Mantener visibles prórroga y penaltis en calendario y resumen.
3. Cambios manuales durante el partido.
4. Tiempos muertos por parte.
5. Cambios de formación e instrucciones en vivo.
6. Alertas de faltas, fatiga, tarjetas y powerplay.
7. IA con decisiones según marcador, tiempo y fatiga.

### Fase 2 — Mercado y plantilla profundos

1. Contratos completos: salario, duración, bonus, cláusula y rol.
2. Renovaciones, jugadores libres, cesiones y precontratos.
3. Moral y felicidad conectadas a minutos, resultados, salario y promesas.
4. Staff contratables con límites y efectos reales.
5. Scouting:
   - asignación a país/región/club;
   - límites de scouts;
   - conocimiento progresivo;
   - rangos de CA/PA/atributos;
   - informes y recomendaciones.

### Fase 3 — Desarrollo deportivo

1. Cantera U12, U14, U16, U18 y U20.
2. Equipos juveniles, entrenadores y calendario juvenil.
3. Potencial en rangos y promoción al primer equipo.
4. Instalaciones de entrenamiento y cantera.
5. Automatismos tácticos entrenables.
6. Progreso individual por atributo y posición.

### Fase 4 — Competiciones y carrera larga

1. Motor genérico de formatos de competición.
2. Integrar el plan visual de `plan_interfaz.md` y unificar la identidad visual del manager.
2. Copas nacionales y supercopas con selección de participantes y formato eliminatorio.
3. Fase de grupos y eliminatorias.
4. Prórroga y tandas de penaltis.
5. Playoffs por título/ascenso.
6. Ascensos y descensos automáticos con reglas por competición.
7. Selecciones, convocatorias y torneos internacionales.
8. Historial, palmarés, récords y estadísticas históricas.

### Fase 5 — Interfaz y experiencia visual

1. Aplicar `plan_interfaz.md` y unificar tokens visuales.
2. Crear componentes compartidos de paneles, métricas, tablas y estados.
3. Rediseñar Dashboard, Plantilla y partido en vivo.
4. Crear bracket visual de copas.
5. Pulir Mercado, Scouting, Cantera y Finanzas.
6. Revisar responsive, accesibilidad y estados de error.

## Fase 6 — Inmersión y producción

1. Noticias y medios.
2. Objetivos de directiva y reputación.
3. Economía avanzada e instalaciones.
4. Tutorial y onboarding.
5. Mejoras visuales del campo y HUD.
6. Rendimiento y memoria con el mundo completo.
7. Guardados seguros, backups y migraciones.
8. Beta testing, balance y distribución.

## Primer bloque iniciado

- [x] Crear este plan de actuación y establecer prioridades.
- [x] Añadir persistencia del resultado del partido vivo y eventos al finalizar.
- [ ] Añadir una pantalla de resumen postpartido.
- [ ] Añadir controles mínimos de intervención durante el partido.
- [x] Exponer visualmente el desenlace de prórroga y penaltis en calendario y resumen.
- [x] Verificar build, typecheck y tests.

## Criterios de aceptación de las próximas iteraciones

- Ninguna acción importante se pierde al cerrar o cambiar de pantalla.
- Un partido jugado actualiza resultado, clasificación, estadísticas y calendario.
- El usuario puede tomar decisiones tácticas durante el partido y ver sus efectos.
- La información de mercado depende progresivamente del scouting.
- Las competiciones distintas de liga tienen reglas y fases propias.
- Cada hito incluye tests automatizados y actualización de este documento.
