import tkinter as tk
import platform
import os
import sys
from tkinter import font, messagebox
import subprocess

# Initalize some variables
print("Retrieving script directory...", end='')
script_dir = os.path.dirname(os.path.abspath(__file__))
exe_dir = os.path.dirname(sys.executable)
print(script_dir)
print("Retrieiving executable path...", end='')
if platform.system() == "Windows":
    try:
        executablePath = subprocess.run(['where', 'dproc'], capture_output=True, text=True, check=True)
        dproc = executablePath.stdout.strip().split('\n')[0]
    except Exception as e:
        cwdexe = os.path.join(script_dir, 'dproc.exe')
        if os.path.isfile(cwdexe):
            dproc = cwdexe
        else:
            if os.path.isfile(os.path.join(exe_dir, 'dproc.exe')):
                dproc = os.path.join(exe_dir, 'dproc.exe')
            else:
                raise FileNotFoundError("Backend executable not found. Reinstalling the program may fix this issue.") from e
else:
    print("ERROR: Platform not (yet) supported.")
    sys.exit(1)
print(dproc)
print("Checking version info...")
dprocVersionInfo = subprocess.run([dproc, '--version'], capture_output=True, text=True)
dprocHelpInfo = subprocess.run([dproc, '--help'], capture_output=True, text=True)
guiVersionInfo = """dproc GUI, version 1.0.0
(C) 2025 Matthew Yang"""
versionInfo = f"""dproc GUI:
{guiVersionInfo}

This program uses dproc:
{dprocVersionInfo.stdout}"""
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
root.geometry("800x600")
root.pack_propagate(False)

root.grid_rowconfigure(0, weight=0)
root.grid_rowconfigure(1, weight=40)
root.grid_rowconfigure(2, weight=0)
root.grid_rowconfigure(3, weight=40)
root.grid_columnconfigure(0, weight=1)

top_frame = tk.Frame(root)
top_frame.configure(height=30)
top_frame.grid(row=0, column=0, sticky="nsew")

text_frame = tk.Frame(root)
text_frame.grid(row=1, column=0, sticky="nsew")

button_frame = tk.Frame(root)
button_frame.configure(height=60) 
button_frame.grid(row=2, column=0, sticky="nsew")

result_frame = tk.Frame(root)
result_frame.grid(row=3, column=0, sticky="nsew")

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
result_area = tk.Text(result_frame, width=100, height=80, wrap=tk.WORD, undo=True)
result_area.config(font=text_font)

scrollbar = tk.Scrollbar(text_frame, orient="vertical", command=text_area.yview)
scrollbar.pack(side=tk.RIGHT, fill=tk.Y)
resultbar = tk.Scrollbar(result_frame, orient="vertical", command=result_area.yview)
resultbar.pack(side=tk.RIGHT, fill=tk.Y)
text_area.config(yscrollcommand=scrollbar.set)
result_area.config(yscrollcommand=resultbar.set)
result_area.config(state="disabled")

class run():
    def run_dproc(data):
        result = subprocess.run([dproc, sc1.get(), sc2.get(), "stdin"], input=data, capture_output=True, text=True)
        return result
    
    def process_data(event=None):
        input_data = text_area.get("1.0", tk.END).strip()
        processed_data = run.run_dproc(input_data)
        mutResult(processed_data.stdout + processed_data.stderr)

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
        messagebox.showinfo("About dproc GUI", versionInfo)

    def show_license(event=None):
        messagebox.showinfo("License", "This program is licensed under the GNU GPLv3. If you did not receive a copy with this program, go to https://github.com/matthewyang204/dproc or read the LICENSE file in your copy of the source code.")

    def help(event=None):
        messagebox.showinfo("Help", dprocHelpInfo.stdout)

def mutResult(result):
    result_area.config(state="normal")
    result_area.delete("1.0", tk.END)
    result_area.insert(tk.END, str(result))
    result_area.config(state="disabled")

class selector2_updater:
    def __init__(self, value):
        self.value = value

    def update(self, new_options):
        menu = selector2['menu']
        menu.delete(0, 'end')
        for option in new_options:
            menu.add_command(label=option, command=tk._setit(sc2, option))
        if new_options:
            sc2.set(new_options[0])
        else:     
            sc2.set("")
    
    def service(self, event=None):
        category = sc1.get()

        if category == "round":
            self.update(roundOptions)
        elif category == "deviate":
            self.update(deviateOptions)
        elif category == "organize":
            self.update(organizeOptions)
        elif category == "enumerate":
            self.update(enumerateOptions)
        elif category == "math":
            self.update(mathOptions)
        elif category == "solve":
            self.update(solveOptions)
        else:
            self.update([])

selector2_updater = selector2_updater(value=None)
def update_s2(*args):
    selector2_updater.service()

text_area.pack(fill=tk.BOTH, expand=tk.YES, side=tk.LEFT)
result_area.pack(fill=tk.BOTH, expand=tk.YES, side=tk.LEFT)
process_button = tk.Button(button_frame, text="Process Data", command=run.process_data, font=text_font)
process_button.pack(side=tk.RIGHT, expand=True)

sc1options = ["round", "deviate", "organize", "enumerate", "math", "solve"]
roundOptions = [
    "mean",
    "median",
    "mode",
    "decimal",
    "integer"
]
deviateOptions = [
    "range",
    "variance",
    "standard",
    "meanAbsolute",
    "medianAbsolute"
]
organizeOptions = [
    "sort"
]
enumerateOptions = [
    "sum",
    "count"
]
mathOptions = [
    "lcm",
    "gcd",
    "gcf",
    "prime-check"
]
solveOptions = [
    "quadratic-single",
    "linear-dual"
]

sc1 = tk.StringVar(root)
sc1.set(sc1options[0])
selector1 = tk.OptionMenu(button_frame, sc1, *sc1options)
sc2 = tk.StringVar(root)
selector2 = tk.OptionMenu(button_frame, sc2, *roundOptions)
selector1.config(font=text_font)
selector2.config(font=text_font)

toplabel = tk.Label(top_frame, text="Enter data to process ↓", font=text_font)
midlabel = tk.Label(button_frame, text="Results ↓", font=text_font)
categorylabel = tk.Label(button_frame, text="Category/SUBCMD1:", font=text_font)
operationlabel = tk.Label(button_frame, text="Operation/SUBCMD2:", font=text_font)

midlabel.pack(side=tk.LEFT)
selector1.pack(side=tk.LEFT, expand=True)
selector2.pack(side=tk.LEFT, expand=True)
categorylabel.pack(side=tk.LEFT, before=selector1, expand=True)
operationlabel.pack(side=tk.LEFT, before=selector2)
toplabel.pack(side=tk.LEFT)

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
about_menu.add_command(label="About dproc GUI", command=about.about)
about_menu.add_command(label="License", command=about.show_license)
about_menu.add_command(label="Help", command=about.help)

# Debug only
# import code
# code.interact(local=locals())

selector2_updater.service()

sc1.trace('w', update_s2)

root.mainloop()