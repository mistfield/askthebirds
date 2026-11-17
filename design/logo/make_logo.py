"""Векторный логотип «askthebirds» из EB Garamond (OFL): жирность 600, разрядка -3.5%."""
import sys, io
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen
import uharfbuzz as hb

SRC, OUT = sys.argv[1], sys.argv[2]
TEXT, WGHT, TRACK = "askthebirds", 600, -0.035

font = instancer.instantiateVariableFont(TTFont(SRC), {"wght": WGHT})
buf = io.BytesIO(); font.save(buf); data = buf.getvalue()
upem = font["head"].unitsPerEm
gs = font.getGlyphSet()

# раскладка с кернингом (harfbuzz)
hbfont = hb.Font(hb.Face(data))
b = hb.Buffer(); b.add_str(TEXT); b.guess_segment_properties()
hb.shape(hbfont, b, {"kern": True, "liga": False})
order = font.getGlyphOrder()

track = TRACK * upem
pen = SVGPathPen(gs, ntos=lambda v: f"{v:.0f}")
bounds = BoundsPen(gs)
x = 0
for i, (info, pos) in enumerate(zip(b.glyph_infos, b.glyph_positions)):
    name = order[info.codepoint]
    for p in (pen, bounds):
        gs[name].draw(TransformPen(p, (1, 0, 0, -1, x + pos.x_offset, -pos.y_offset)))
    x += pos.x_advance + (track if i < len(TEXT) - 1 else 0)

xmin, ymin, xmax, ymax = bounds.bounds
# высота строчных: по глифу «x»
xb = BoundsPen(gs); gs[font.getBestCmap()[ord("x")]].draw(xb)
xh = xb.bounds[3]
w, h = xmax - xmin, ymax - ymin
svg = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{xmin:.0f} {ymin:.0f} {w:.0f} {h:.0f}" '
       f'role="img" aria-label="askthebirds"><path fill="currentColor" d="{pen.getCommands()}"/></svg>\n')
open(OUT, "w").write(svg)
print(f"viewBox {xmin:.0f} {ymin:.0f} {w:.0f} {h:.0f}; x-height {xh:.0f}; height/xh {h/xh:.4f}; width/xh {w/xh:.4f}; bytes {len(svg)}")
