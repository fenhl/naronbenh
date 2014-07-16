#!/usr/bin/env python3

"""Information about dimensions of Naron Benh on Wurstmineberg.

Usage:
  naronbenh [options] [--] [<output_dir>]
  naronbenh [options] [--] <x> <y> <z>
  naronbenh -h | --help

Options:
  -h, --help       Print this message and exit.
  -p, --perimeter  Calculate the perimeter instead of the main building. The perimeter is the same on all y levels, so the <y> argument is ignored in this case.
  -q, --quiet      Don't print render status.
  -v, --verbose    Print result of query in addition to exit status.
  --max-x=<maxx>   Set the maximum x coordinate for renders [Default: 4500].
  --max-z=<maxz>   Set the maximum z coordinate for renders [Default: -4000].
  --min-x=<minx>   Set the minimum x coordinate for renders [Default: 4200].
  --min-z=<minz>   Set the minimum z coordinate for renders [Default: -4300].
"""

import sys

import PIL.Image
from docopt import docopt
import math
import os
import os.path

upper = {
    (4352, 68, -4096),
    (4360, 68, -4096),
    (4352, 68, -4090),
    (4360, 68, -4090),
    (4368, 68, -4240),
    (4374, 68, -4240),
    (4368, 68, -4232),
    (4374, 68, -4232),
    (4352, 64, -4096),
    (4360, 64, -4096),
    (4352, 64, -4090),
    (4360, 64, -4090),
    (4368, 64, -4240),
    (4374, 64, -4240),
    (4368, 64, -4232),
    (4374, 64, -4232)
}

lower = {
    (4352, 35, -4096),
    (4360, 35, -4096),
    (4352, 35, -4090),
    (4360, 35, -4090),
    (4368, 35, -4240),
    (4374, 35, -4240),
    (4368, 35, -4232),
    (4374, 35, -4232)
}

coords = upper | lower

def in_perimeter(x, z, main_building=None):
    def in_main_building(cx, cz):
        if main_building is None:
            return any(in_range(cx, y, cz) for y in range(0, 176))
        return (cx, cz) in main_building
    
    return any(math.sqrt((x - dx) ** 2 + (z - dz) ** 2) <= 128 and in_main_building(dx, dz) for dx in range(x - 128, x + 128) for dz in range(z - 128, z + 128))

def in_range(x, y, z):
    for cx, cy, cz in coords:
        if math.sqrt((x - cx) ** 2 + (y - cy) ** 2 + (z - cz) ** 2) > 128:
            return False
    for cx, cy, cz in upper:
        if math.sqrt((x - cx) ** 2 + (y - cy) ** 2 + (z - cz) ** 2) < 24:
            return False
    return True

def save(minx=4200, maxx=4500, y=66, minz=-4300, maxz=-4000, directory=os.getcwd()):
    img = PIL.Image.new('1', (maxx - minx, maxz - minz))
    for z in range(img.size[1]):
        for x in range(img.size[0]):
            img.putpixel((x, z), int(in_range(x + minx, y, z + minz)))
    img.save(os.path.join(directory, str(y) + '.png'))

def save_perimeter(minx=4200, maxx=4500, minz=-4300, maxz=-4000, directory=os.getcwd(), verbose=False):
    if verbose:
        print('[ ** ]', 'coordinates: x from', minx, 'to', str(maxx) + ', z from', minz, 'to', maxz)
        print('[....]', 'calculating main building', end='\r', flush=True)
    main_building = set()
    for z in range(minz, maxz):
        if verbose:
            progress = min(4, int(5 * (z - minz) / (maxz - minz)))
            print('[' + '=' * progress + '.' * (4 - progress) + ']', 'calculating main building: row', z - minz, 'of', maxz - minz, end='\r', flush=True)
        main_building |= set((x, z) for x in range(minx, maxx) if any(in_range(x, y, z) for y in range(0, 176)))
    if verbose:
        print('[ ok ] calculating main building' + ' ' * (10 + len(str(z - minz)) + len(str(maxz - minz))))
        print('[....]', 'calculating perimeter', end='\r', flush=True)
    img = PIL.Image.new('1', (maxx - minx, maxz - minz))
    for z in range(img.size[1]):
        if verbose:
            progress = min(4, int(5 * z / img.size[1]))
            print('[' + '=' * progress + '.' * (4 - progress) + ']', 'calculating perimeter: row', z, 'of', img.size[1], end='\r', flush=True)
        for x in range(img.size[0]):
            img.putpixel((x, z), int(in_perimeter(x + minx, z + minz, main_building=main_building)))
    if verbose:
        print('[ ok ] calculating perimeter' + ' ' * (10 + len(str(z)) + len(str(img.size[1]))))
        print('[....]', 'saving image', end='\r', flush=True)
    img.save(os.path.join(directory, 'perimeter.png'))
    if verbose:
        print('[ ok ]')

if __name__ == '__main__':
    arguments = docopt(__doc__)
    if arguments['<x>']:
        if arguments['--perimeter']:
            ret = in_perimeter(int(arguments['<x>']), int(arguments['<z>']))
        else:
            ret = in_range(int(arguments['<x>']), int(arguments['<y>']), int(arguments['<z>']))
        if arguments['--verbose']:
            print('[ ** ]', str(arguments['<x>']), str(arguments['<y>']), str(arguments['<z>']), 'is', 'in' if ret else 'out of', 'range')
        sys.exit(0 if ret else 1)
    if arguments['--perimeter']:
        save_perimeter(directory=arguments['<output_dir>'] or os.getcwd(), verbose=not arguments['--quiet'], minx=int(arguments['--min-x']), maxx=int(arguments['--max-x']), minz=int(arguments['--min-z']), maxz=int(arguments['--max-z']))
    else:
        for i in range(0, 176):
            if not arguments['--quiet']:
                progress = min(4, int(5 * i / 176))
                print('[' + '=' * progress + '.' * (4 - progress) + ']', 'saving image for y level', str(i), end='\r', flush=True)
            save(y=i, directory=arguments['<output_dir>'] or os.getcwd(), minx=int(arguments['--min-x']), maxx=int(arguments['--max-x']), minz=int(arguments['--min-z']), maxz=int(arguments['--max-z']))
        if not arguments['--quiet']:
            print('[ ok ] saving images' + ' ' * 11)
