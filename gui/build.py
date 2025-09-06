import subprocess
import sys
import os


def run_pyinstaller():
    try:
        main_script = os.path.join('src', 'main.py')

        # PyInstaller command to build the executable
        cmd = [
            'pyinstaller',
            main_script,
            '-w',  # Makes it windowed
            '--name', 'dproc-gui'
            '--icon=icon.ico',
            '--onefile'  # Bundle into a single executable
        ]

        # Run PyInstaller
        subprocess.check_call(cmd)

        print("Build successful.")
    except Exception as e:
        print(f"Build failed: {e}")


if __name__ == '__main__':
    run_pyinstaller()
