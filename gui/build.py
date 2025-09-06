import subprocess
import sys
import os

script_dir = os.path.dirname(os.path.abspath(__file__))
arguments = sys.argv

def run_pyinstaller(config):
    try:
        main_script = os.path.join(script_dir, 'main.py')

        # PyInstaller command to build the executable
        if config == "release":
            cmd = [
                'pyinstaller',
                main_script,
                '-w',  # Makes it windowed
                '--name', 'dproc-gui',
                # '--icon=icon.ico',
                '--onefile'  # Bundle into a single executable
            ]
        elif config == "debug":
            cmd = [
                'pyinstaller',
                main_script,
                # '-w',  # Makes it windowed
                '--name', 'dproc-gui',
                # '--icon=icon.ico',
                '--onefile'  # Bundle into a single executable
            ]
        else:
            print("ERROR: Invalid build configuration. Use 'release' or 'debug'.")
            sys.exit(1)

        # Run PyInstaller
        subprocess.check_call(cmd)

        print("Build successful.")
    except Exception as e:
        print(f"Build failed: {e}")


if __name__ == '__main__':
    if "--release" in arguments:
        run_pyinstaller("release")
    elif "--debug" in arguments:
        run_pyinstaller("debug")
    else:
        print("Using default 'release' configuration.")
        run_pyinstaller("release")
