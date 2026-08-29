# FUTSAL MANAGER 27 — Plan de interfaz

## Dirección visual

Inspiración funcional en las capturas de `Imagenes_Ejemplo/`: interfaz de manager densa pero legible, navegación persistente, paneles de información, tablas operativas y fichas con jerarquía clara. No se copiará la identidad visual: se usará una paleta propia de futsal.

### Paleta propuesta

- Fondo principal: azul tinta `#08111f`.
- Paneles: azul pizarra `#101d2f`.
- Panel elevado: `#172840`.
- Acento principal: turquesa eléctrico `#22d3c5`.
- Acento secundario: coral `#fb7185`.
- Información: azul cielo `#60a5fa`.
- Éxito: verde `#34d399`.
- Advertencia: ámbar `#fbbf24`.
- Texto: `#f8fafc`; texto secundario: `#94a3b8`.

## Arquitectura de navegación

- Shell persistente con cabecera, temporada, fecha, club y buzón.
- Navegación agrupada por contexto:
  - Club: Dashboard, Plantilla, Táctica, Entrenamiento, Cantera.
  - Competición: Calendario, Clasificación, Partido en vivo.
  - Mercado: Mercado, Ojeo, Contratos.
  - Gestión: Finanzas, Buzón, instalaciones.
- Navegación móvil mediante menú compacto y acciones prioritarias visibles.
- Editor separado y bloqueado durante una partida activa.

## Pantallas prioritarias

### Dashboard

- Cabecera con fecha, temporada y próximo partido.
- Tarjeta hero del club con pabellón, escudo, posición y forma.
- Próximo partido como bloque central con rival, competición y acceso táctico.
- Resumen de clasificación y últimos resultados.
- Alertas destacadas: lesiones, contratos, ofertas, cantera y objetivos.
- Acciones rápidas para avanzar día/semana.

### Plantilla y perfiles

- Tabla compacta con posición, nombre, banderas, edad, CA/PA, condición, moral y salario.
- Filtros por posición, estado contractual, lesión y nacionalidad.
- Ficha lateral o modal con atributos agrupados, contrato, historial y posiciones secundaria.
- Barras visuales de condición, forma, moral y desarrollo.

### Partido en vivo

- Marcador persistente con competición, pabellón y reloj.
- Campo 2D como foco visual.
- Panel lateral de eventos, faltas, tarjetas, powerplay y cambios.
- Controles tácticos agrupados en formación, presión, tempo y línea defensiva.
- Estados explícitos para descanso, final, prórroga y penaltis.

### Calendario y competiciones

- Selector de competición con etiquetas de liga/copa.
- Vista por jornada o ronda.
- Eliminatorias en formato visual de enfrentamientos.
- Resultado con indicadores de prórroga, penaltis y clasificado.
- Filtros por mes, competición y partidos del club.

### Mercado y scouting

- Tabla con conocimiento parcial claramente indicado.
- Etiquetas de recomendación y nivel de certeza.
- Banderas y posiciones visibles sin saturar la fila.
- Panel de informe detallado con fortalezas, debilidades y rangos.

### Entrenamiento y cantera

- Semana representada como tarjetas de sesiones.
- Impacto del staff visible junto al progreso.
- Cantera separada por U12, U14, U16, U18 y U20.
- Acciones de promoción con requisitos y confirmación.

### Finanzas y gestión

- Cabecera con balance, presupuesto de fichajes y margen salarial.
- Gráficos ligeros de evolución mensual.
- Desglose de salarios, taquilla, patrocinio, premios y gastos.
- Alertas cuando un presupuesto está comprometido.

### Editor

- Navegación por pestañas: países, clubes, jugadores, staff, contratos, economía, pabellones y competiciones.
- Cabecera de modo prepartida con métricas del módulo activo y búsqueda accesible.
- Pestañas con iconos, roles ARIA, foco visible y desplazamiento horizontal en móvil.
- Estados de carga, mensajes de resultado y estados vacíos accionables.
- Formularios en dos columnas en escritorio y una en móvil.
- Previsualización de escudos, banderas, fotos y pabellones.
- Confirmaciones para borrar o cambiar asociaciones.

## Componentes reutilizables

- `PageHeader`, `Panel`, `MetricCard`, `StatusBadge`, `DataTable`, `EmptyState`, `ConfirmDialog`.
- `Flag`, `ClubBadge`, `PlayerAvatar`, `StadiumPhoto`.
- Barras de progreso para atributos y estados.
- Skeletons de carga y mensajes de error accionables.
- Diseño responsive con tablas convertibles a tarjetas.

## Accesibilidad y calidad

- Contraste AA como mínimo.
- Estados no comunicados únicamente por color.
- Navegación por teclado y foco visible.
- Tooltips para métricas y abreviaturas.
- Fechas y cantidades formateadas de forma consistente.
- No ocultar errores mediante `catch` silenciosos.

## Roadmap visual

1. Unificar tokens de color, espaciado, bordes y tipografía. (En progreso: tema global y Dashboard aplicados.)
2. Crear componentes comunes de paneles, métricas y tablas. (Implementado: `src/components/ui.tsx`.)
3. Rediseñar Dashboard con jerarquía de manager.
4. Mejorar Plantilla, perfiles y Partido en vivo.
5. Crear bracket visual de copas.
6. Mejorar Mercado, Scouting, Cantera y Finanzas.
7. Añadir fotografías de pabellones y avatares con fallback. (Implementado en Editor.)

Progreso: el tema global, Dashboard, Calendario, Plantilla, Mercado, Scouting, Cantera, Finanzas y navegación responsive ya tienen la primera capa visual aplicada.
8. Revisar responsive, accesibilidad, estados vacíos y rendimiento. (Editor revisado; resto pendiente de auditoría final.)
9. Validar visualmente cada flujo con datos reales y capturas de regresión.

## Pendientes funcionales incorporados al plan general

- Prórroga y penaltis completamente conectados a API, calendario y resumen.
- Supercopas y formatos de competición configurables.
- Grupos y playoffs.
- Ascensos y descensos.
- Selecciones y torneos internacionales.
- Historial, palmarés y récords.
- Automatismos tácticos entrenables.
- Moral, química y roles.
- Noticias, objetivos de directiva y economía avanzada.
- Guardado múltiple, autoguardado, backups y migraciones.
- Tutorial, accesibilidad, optimización y distribución.
