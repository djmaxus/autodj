# FIXME: rewrite this in Rust

# Run this script right after forking the repository.
# Suitable for any OS with Python installed.

# This could be done using the git module. But why bother installing it.
from os import chdir as cd
from os.path import abspath, dirname, join, pardir, realpath
from os import stat, chmod
from stat import S_IEXEC as EXECUTE_PERMISSION

from subprocess import CalledProcessError, check_output as execute
from sys import exit, platform

from glob import glob

# Introduce ourselves.
print('init.py called')

# Get root directory of this repository.
REPO_DIR = abspath(join(dirname(realpath(__file__)), pardir))

# And move there.
cd(REPO_DIR)

# Let's tell git to look for hooks in our custom hooks directory.
# If something goes wrong, abort and tell user about it.
try:
    execute(['git', 'config', 'core.hooksPath', 'utils/git/hooks'])
    print('Hooks were configured successfully.')
except CalledProcessError as e:
    print(
        f'Something went wrong.\n'
        f'Error code: {e.returncode}\n'
    )
    exit(1)

# If we are not on Linux, our job is completed.
if platform.startswith('win32'):
    exit(0)

# If we are on Linux/Unix, we have to grant execute permissions to each hook.


# List files without extension. Those are the hooks.
hooks = [file for file in glob(f'{REPO_DIR}/hooks/*') if '.' not in file]

# Add execute permission to a list of already granted permissions.
for file in hooks:
    chmod(file, stat(file).st_mode | EXECUTE_PERMISSION)

print('Permissions were granted to all hooks successfully.')
