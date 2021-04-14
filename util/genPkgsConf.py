import json
import subprocess
from pathlib import Path


def genPkgsConf(pkgListPath):
    # Attempt to load the existing pkgs.json, if !exist, touch one.
    pkgsWithVer = {}
    if pkgListPath.exists():
        pkgListPath.unlink()
        pkgListPath.touch()

    # Get the packages that are not from repositories. Using pacman.
    pmResult = subprocess.Popen(['pacman -Qm'],
                                shell=True,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE
                                ).communicate()[0].decode().split('\n')[:-1]

    # Generate config.
    for result in pmResult:
        pkg, ver = result.split(' ')
        pkgsWithVer[pkg] = ver

    with open(pkgListPath, 'w') as f:
        f.write(json.dumps(pkgsWithVer, indent=4))
    
    print('Finished.')


if __name__ == "__main__":
    pkgListPath = input('Please tell me where to save package list:\n')
    genPkgsConf(pkgListPath)