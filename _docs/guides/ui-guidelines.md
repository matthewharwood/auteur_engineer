# Editorial UI Design Guidelines

## Design Philosophy

Our design system embodies a **contemporary editorial aesthetic** — pages should feel like beautifully typeset publications or art books, not generic dashboards. We achieve this through deliberate typography, generous spacing, and strategic use of color blocks that echo magazine layouts.

## Starting from Scratch

### Start with a feature, not a layout
Design core functionality first before tackling navigation or sidebars. This provides necessary context for broader layout decisions.

### Detail comes later
Work in low-fidelity initially. Design in grayscale to establish hierarchy through spacing and contrast before adding color.

### Don't design too much
Work in short cycles: design simple, build, iterate. Ship the smallest useful version first.

### Define your editorial personality
- **Typography**: Pair large, high-contrast serif headlines with neutral sans-serif body copy
- **Corners**: Use sharp corners for a crisp, publication-like feel
- **Playful elements**: Introduce brightly colored icons or doodle-style illustrations sparingly
- **Color blocks**: Apply saturated accent colors as full-bleed backgrounds or type highlights
- **Voice**: Confident, concise, and creative — like gallery copy, not technical documentation

### Establish your constraints
- **Typefaces**: One serif display face + one sans-serif family with multiple weights and true italics
- **Colors**: 2-3 saturated accent colors (burnt orange, emerald green, sunshine yellow) + comprehensive grey scale
- **Spacing**: Generous margins and clear column spacing for print-like rhythm
- **Grid**: Discrete columns rather than fluid grids — maintain intentional proportions

## Hierarchy is Everything

### Create dramatic scale differences
Use oversized serif headlines contrasted with small caps or lighter sans-serif for supporting information. Many pages should echo magazine spreads with multiple columns.

### Use more than size
- **Weights**: Bold serifs for primary, italic or small caps for secondary
- **Color**: Headlines on colored blocks command more attention
- **Contrast**: White or high-contrast text on saturated backgrounds

### Color backgrounds effectively
- White or high-contrast colored text works best on saturated backgrounds
- For de-emphasis on colored panels, adjust hue or lightness rather than using grey
- Pair deep mustard blocks with lighter yellow text for sophisticated contrast

### Emphasize by de-emphasizing
Make important elements stand out by reducing emphasis on surrounding elements. Soften inactive navigation rather than bolding active items.

### Minimize labels
Avoid "label: value" patterns. Let data format and context speak for themselves. When labels are necessary, treat them as secondary content.

## Layout and Spacing

### Start with print-like margins
Begin with large gutters around pages and clear separation between columns. More breathing room than you think necessary — tighten only if composition feels disconnected.

### Use discrete column systems
- Single-column: 600-800px centered even on large screens
- Double-column: Main content + sidebar for metadata
- Triple-column: Add third for notes, CVs, or supplementary content

### Respect optimal widths
Don't stretch content to fill screens. Let components be their natural width. Wide margins on large displays are intentional, not wasted space.

### Space purposefully
Space between groups must exceed space within groups. In forms, label-to-input spacing should be tighter than spacing between fields.

## Typography

### Establish a refined type scale
Create a hand-picked scale: 12px, 14px, 16px, 18px, 20px, 24px. Use px or rem units exclusively.

### Pair typefaces strategically
- **Serif**: For headlines, pull quotes, and editorial moments
- **Sans-serif**: For body copy, captions, and functional text
- Look for families with multiple weights and true italics

### Control reading experience
- **Line length**: 45-75 characters (20-35em width)
- **Line height**: Taller for wide columns, shorter for large headlines
- **Alignment**: Left-align body text, baseline-align mixed sizes

### Apply letter-spacing thoughtfully
- Tighten tracking on large serif headlines for polished book typography
- Increase spacing for all-caps labels (PROFILE, CV) to prevent crowding

## Color System

### Work in HSL
Manipulate Hue (0-360°), Saturation (0-100%), and Lightness (0-100%) for predictable color relationships.

### Build a sophisticated palette
- **Greys**: 8-10 shades, tinted to complement accent colors
- **Accent colors**: 2-3 saturated choices, each with 5-10 shades
- **Supporting colors**: Subtle shades for system states

### Maintain vibrancy across shades
- Increase saturation as lightness moves from 50%
- Rotate hue slightly toward yellow/cyan (brighter) or blue/red (darker)
- Warm greys with orange tint or cool with green to match palette temperature

### Ensure accessibility creatively
When contrast requirements challenge aesthetics:
- Flip to dark text on light backgrounds
- Rotate hues toward naturally brighter colors
- Never rely on color alone — supplement with icons or contrast variations

## Creating Depth

### Embrace editorial flatness
Achieve depth through contrasting color fields rather than heavy shadows. A bright block behind one column and white behind another creates elegant layering.

### Use shadows sparingly
- Cards float gently with barely-there halos
- Buttons have subtle, tight shadows
- Modals may have slightly larger, soft shadows
- Limit to 2-3 shadow styles maximum

### Layer through overlap
- Extend colored blocks past containers
- Offset cards across background boundaries
- Use color contrast rather than drop shadows for separation

## Working with Images

### Demand quality
Use professional photography or high-quality stock. Crop consistently within fixed aspect-ratio containers aligned to column grid.

### Isolate text from images
Rarely overlay long text on photos. Instead:
- Place type on solid color blocks
- Let images stand alone in their columns
- If overlay necessary, apply semi-transparent tint or color wash

### Respect intended sizes
- Keep icons at 16-24px or embed in colored shapes
- Maintain thin strokes on line illustrations with generous padding
- Crop photos to desired ratios rather than arbitrary scaling

## Finishing Touches

### Enhance existing elements
- Replace bullets with custom icons and timeline connections
- Transform quotation marks into oversized decorative motifs
- Style form elements to echo accent colors and typography

### Use accent borders strategically
- Thin ruling lines between columns
- Colored bars along panel sides
- Underlines for headlines
Match these to accent palette for cohesion.

### Add subtle background interest
- Saturated color blocks behind content sections
- Geometric illustrations or line drawings at low contrast
- Patterns that enhance rather than compete

### Design empty states intentionally
Use illustrations and clear CTAs. Hide filters and auxiliary UI until content exists.

### Minimize borders
Instead of borders, use:
- Subtle box shadows
- Different background colors
- Additional spacing

## Continuous Improvement

### Study unexpected choices
Analyze admired designs for unconventional decisions — unusual layouts, color combinations, or interactions you wouldn't have considered.

### Recreate to understand
Rebuild favorite interfaces from scratch to internalize subtle techniques and understand design decisions at a deeper level.