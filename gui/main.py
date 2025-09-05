import tkinter as tk
import platform
import os
import sys

# Initalize some variables
print("Retrieiving temporary directory...", end='')
if platform.system() == "Windows":
    print("Unimplemented STUB")
else:
    temp = os.path.join("/tmp")
    print(temp)

root = tk.Tk()
root.title("dproc GUI")
root.minsize(800, 600)
root.pack_propagate(False)

root.mainloop()