🎨 Production Color System for SurfDesk

This production color system is designed for enterprise-grade applications with focus on accessibility, consistency, and professional appearance across all platforms (Desktop, Web, Terminal).

## Primary Colors

### Production Accent — Success Green
Hex: #10B981
RGB: rgb(16, 185, 129)
Used for success states, successful operations, and SurfPool running status.

### Production Primary — Professional Blue
Hex: #3B82F6
RGB: rgb(59, 130, 246)
Primary action color for buttons, links, and interactive elements.

### Production Warning — Amber
Hex: #F59E0B
RGB: rgb(245, 158, 11)
Used for warning states and attention-grabbing elements.

### Production Error — Red
Hex: #EF4444
RGB: rgb(239, 68, 68)
Used for error states, failed operations, and critical alerts.

## Neutral Colors

### Production Background — Dark Mode
Hex: #111827
RGB: rgb(17, 24, 39)
Primary background color for production applications.

### Production Surface — Elevated Cards
Hex: #1F2937
RGB: rgb(31, 41, 55)
Background for cards, panels, and elevated surfaces.

### Production Border — Subtle Separation
Hex: #374151
RGB: rgb(55, 65, 81)
Used for borders and dividers between sections.

### Production Text — Primary Content
Hex: #F9FAFB
RGB: rgb(249, 250, 251)
Primary text color for readability and contrast.

### Production Text Secondary — Muted Content
Hex: #9CA3AF
RGB: rgb(156, 163, 175)
Secondary text color for less important information.

## SurfPool Status Colors

### SurfPool Running — Success
Hex: #10B981
RGB: rgb(16, 185, 129)
Indicates SurfPool is running and operational.

### SurfPool Stopped — Neutral
Hex: #6B7280
RGB: rgb(107, 114, 128)
Indicates SurfPool is stopped but available.

### SurfPool Error — Critical
Hex: #EF4444
RGB: rgb(239, 68, 68)
Indicates SurfPool encountered an error.

## Production Usage Guidelines

1. **Consistency**: Use these colors consistently across all platforms
2. **Accessibility**: Ensure WCAG AA compliance with proper contrast ratios
3. **Professional**: Maintain enterprise-grade appearance
4. **SurfPool Integration**: Use status colors to clearly indicate SurfPool states
5. **Terminal Compatibility**: Colors work well in terminal environments with proper ANSI codes

## CSS Variables

All colors are available as CSS variables in the consolidated core styles:
```css
--color-primary: #3B82F6;
--color-success: #10B981;
--color-warning: #F59E0B;
--color-error: #EF4444;
--color-background: #111827;
--color-surface: #1F2937;
--color-border: #374151;
--color-text: #F9FAFB;
--color-text-secondary: #9CA3AF;
```
