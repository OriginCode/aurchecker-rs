#!/usr/bin/env python3

import json
import subprocess
from pathlib import Path


def genPkgsConf(pkgListPath):
    # Create pkgs.json
    if pkgListPath.exists():
        pkgListPath.unlink()
        pkgListPath.touch()
    else:
        pkgListPath.touch()

    # Get the packages that are not from repositories. Using pacman.
    pmResult = subprocess.Popen(['pacman -Qm'],
                                shell=True,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE
                                ).communicate()[0].decode().split('\n')[:-1]
    pkgsWithVer = {}

    # Generate config.
    for result in pmResult:
        pkg, ver = result.split(' ')
        pkgsWithVer[pkg] = ver

    with open(pkgListPath, 'w') as f:
        f.write(json.dumps(pkgsWithVer, indent=4))
    
    print('Finished.')


if __name__ == "__main__":
    if (pkgListPath := Path(input('Please tell me where to save package list:\n')).expanduser()).is_dir():
        genPkgsConf(pkgListPath / "pkgs.json")
    else:
        genPkgsConf(pkgListPath)
