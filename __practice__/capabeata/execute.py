import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

BUILD_DIR_NAME = 'build'
EXECUTABLE_NAME = 'core_capa_module.exe'


def run_command(command, check=True):
    return subprocess.run(
        command,
        check=check,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )


def main():
    parser = argparse.ArgumentParser(description="Automated CMake build script.")
    parser.add_argument(
        '--verbose-build',
        action='store_true',
        help="Enable verbose output for CMake/makefiles."
    )
    args = parser.parse_args()

    project_root = Path(__file__).resolve().parent
    build_dir = project_root / BUILD_DIR_NAME

    print(f"Step 0: Cleaning build directory -> {build_dir}")
    if build_dir.exists():
        shutil.rmtree(build_dir)

    build_dir.mkdir(parents=True, exist_ok=True)

    os.chdir(build_dir)
    print(f"Working directory is now: {Path.cwd()}")

    try:
        print('\nConfiguring project with CMake...')
        cmake_args = ['cmake', '..']
        if args.verbose_build:
            cmake_args.append('-DCMAKE_VERBOSE_MAKEFILE=ON')

        run_command(cmake_args)

        print('\nBuilding project...')

        try:
            ninja_path = shutil.which('ninja')
        except:
            ninja_path = None

        if ninja_path:
            print("Using Ninja to build.")
            build_command = [ninja_path]
        else:
            print("Using default CMake builder.")
            build_command = ['cmake', '--build', '.']

        run_command(build_command)

        exe_candidates = [
            build_dir / 'core_capa_module' / EXECUTABLE_NAME,
            build_dir / 'bin' / EXECUTABLE_NAME
        ]

        exe_path = None
        for cand in exe_candidates:
            if cand.exists():
                exe_path = cand
                break

        if not exe_path:
            raise FileNotFoundError(
                f"Could not locate built executable. Checked: {[str(c) for c in exe_candidates]}"
            )

        print(f"\nRunning: {exe_path}")

        result = run_command([str(exe_path)], check=False)

        if result.returncode != 0:
            raise subprocess.CalledProcessError(
                result.returncode,
                f"Program exited with code {result.returncode}"
            )

        print(result.stdout)

    except (subprocess.CalledProcessError, FileNotFoundError) as e:
        print(f"\n[ERROR] Build failed: {e}", file=sys.stderr)
        sys.exit(1)

    finally:
        os.chdir(project_root)

    print('\nDone.')


if __name__ == '__main__':
    main()
