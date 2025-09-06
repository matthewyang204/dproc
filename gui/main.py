import tkinter as tk
import platform
import os
import sys
from tkinter import font

# Initalize some variables
print("Retrieiving temporary directory...", end='')
if platform.system() == "Windows":
    temp = os.getenv('TEMP')
    print(temp)
else:
    temp = os.path.join("/tmp")
    print(temp)

root = tk.Tk()
root.title("dproc GUI")
root.minsize(800, 600)
root.pack_propagate(False)

text_frame = tk.Frame(root)
text_frame.pack(fill=tk.BOTH, expand=True)

def get_font_for_platform():
    if os.name == 'nt':
        return font.Font(family="Consolas", size=12)
    elif os.uname().sysname == 'Darwin':
        return font.Font(family="Menlo", size=12)
    else:
        return font.Font(family="DejaVu Sans Mono", size=12)

text_font = get_font_for_platform()
text_area = tk.Text(text_frame, width=100, height=80, wrap=tk.WORD, undo=True)
text_area.config(font=text_font)

scrollbar = tk.Scrollbar(text_frame, orient="vertical", command=text_area.yview)
scrollbar.pack(side=tk.RIGHT, fill=tk.Y)
text_area.config(yscrollcommand=scrollbar.set)

def run_dproc(data):
    print("Unimplemented STUB")

text_area.pack(fill=tk.BOTH, expand=tk.YES, side=tk.LEFT)

root.mainloop()