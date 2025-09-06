import tkinter as tk
import platform
import os
import sys
from tkinter import font, messagebox

# Initalize some variables
print("Checking version info...")
versionInfo = """dproc, version 1.1.1
(C) 2013-2014 The Rust Project Developers
(C) 2025 Matthew Yang"""
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

class run():
    def run_dproc(data):
        print("Unimplemented STUB")
    
    def process_data(event=None):
        print("Unimplemented STUB")

class clipboard():
    def cut_text(event=None):
        text_area.clipboard_clear()
        text_area.clipboard_append(text_area.get("sel.first", "sel.last"))
        text_area.delete("sel.first", "sel.last")
        print("Cut option succeeded")
        return 'break'

    def copy_text(event=None):
        text_area.clipboard_clear()
        text_area.clipboard_append(text_area.get("sel.first", "sel.last"))
        print("Text copied to clipboard")
        return 'break'


    def paste_text(event=None):
        text_area.insert("insert", text_area.clipboard_get())
        print("Text pasted from clipboard")
        return 'break'


    def select_all_text(event=None):
        text_area.tag_add("sel", "1.0", "end")
        print("Text selected")
        return 'break'

class changes():
    def undo(event=None):
        try:
            text_area.edit_undo()
        except tk.TclError:
            pass
        print("Edit undone")


    def redo(event=None):
        try:
            text_area.edit_redo()
        except tk.TclError:
            pass
        print("Edit redone")

class about():
    def about(event=None):
        messagebox.showinfo("About dproc", versionInfo)

    def show_license(event=None):
        messagebox.showinfo("License", "This program is licensed under the GNU GPLv3. If you did not receive a copy with this program, go to https://github.com/matthewyang204/dproc or read the LICENSE file in your copy of the source code.")

text_area.pack(fill=tk.BOTH, expand=tk.YES, side=tk.LEFT)

menu = tk.Menu(root)
root.config(menu=menu)

edit_menu = tk.Menu(menu, tearoff=0)
menu.add_cascade(label="Edit", menu=edit_menu)
# edit_menu.add_command(label="Jump To Cursor [Debug]", command=text_scroll.to_cursor)
edit_menu.add_command(label="Cut", command=clipboard.cut_text)
edit_menu.add_command(label="Copy", command=clipboard.copy_text)
edit_menu.add_command(label="Paste", command=clipboard.paste_text)
edit_menu.add_command(label="Select All", command=clipboard.select_all_text)
edit_menu.add_separator()
edit_menu.add_command(label="Undo", command=changes.undo)
edit_menu.add_command(label="Redo", command=changes.redo)

about_menu = tk.Menu(menu, tearoff=0)
menu.add_cascade(label="About", menu=about_menu)
about_menu.add_command(label="About dproc", command=about.about)
about_menu.add_command(label="License", command=about.show_license)

root.mainloop()